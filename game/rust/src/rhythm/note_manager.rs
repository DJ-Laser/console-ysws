use godot::{classes::Input, prelude::*};

use crate::{
  gameplay::{held_note::HeldNote, single_note::SingleNote},
  rhythm::{
    conductor::Conductor,
    note_manager::event_queue::NoteEventQueue,
    notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow},
  },
};

mod event_queue;

const RHYTHM_INPUT_ACTION: &str = "rhythm_low";

type DynNote = DynGd<Node, dyn Note>;

#[derive(Debug)]
pub struct AssociatedNoteEvent {
  event: NoteEvent,
  note: DynNote,
}

impl AssociatedNoteEvent {
  fn event_type(&self) -> NoteEventType {
    self.event.event_type()
  }

  fn at(&self) -> f64 {
    self.event.at()
  }
}

impl AssociatedNoteEvent {
  fn new(event: NoteEvent, note: DynNote) -> Self {
    Self { event, note }
  }
}

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct NoteManager {
  #[export]
  conductor: OnEditor<Gd<Conductor>>,

  /// How many pixels represent one beat
  /// Beat 1 will start x * 1 pixels away
  /// Beat 2 will start x * 2 pixels away
  /// Higher values will result in faster scrolling because the note must travel further in the same time
  /// This value is independent of BPM, higher BPM songs will naturally scroll faster
  #[export]
  #[init(val = 400.0)]
  scroll_speed: f64,

  #[export]
  #[init(val = 25.0)]
  input_latency_ms: f64,

  events: NoteEventQueue,
  held_events: Vec<AssociatedNoteEvent>,

  #[init(load = "res://objects/single_note/single_note.tscn")]
  single_note_scene: OnReady<Gd<PackedScene>>,

  #[init(load = "res://objects/held_note/held_note.tscn")]
  held_note_scene: OnReady<Gd<PackedScene>>,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for NoteManager {
  fn ready(&mut self) {
    for i in 1..=20 {
      let note = self
        .single_note_scene
        .instantiate()
        .expect("Scene should be valid");
      let mut note: Gd<SingleNote> = note.cast();

      note.bind_mut().set_hit_beat(i as f64 * 1.0);
      note.bind_mut().set_note_manager(Some(self.to_gd()));

      self.base_mut().add_child(&note);
      self.add_note(note.into_dyn().upcast());
    }

    let note = self
      .held_note_scene
      .instantiate()
      .expect("Scene should be valid");
    let mut note: Gd<HeldNote> = note.cast();

    note.bind_mut().set_start_beat(24.0);
    note.bind_mut().set_release_beat(30.0);
    note.bind_mut().set_note_manager(Some(self.to_gd()));

    self.base_mut().add_child(&note);
    self.add_note(note.into_dyn().upcast());
  }

  fn process(&mut self, _delta: f64) {
    while let Some(next_event) = self.events.peek() {
      let delta = self.get_event_delta(next_event);

      if NoteTimingWindow::is_too_late(delta) {
        // Dodge notes should get handled befote the miss window
        if matches!(next_event.event_type(), NoteEventType::Dodge) {
          godot_warn!("Dodge note not handled before miss timing");
        }

        self.hit_next_note(NoteTimingWindow::Miss);
        // Continue to remove all missed notes this frame
        continue;
      }

      let Some(rating) = NoteTimingWindow::from_delta(delta) else {
        // Event was too early, wait till next frame
        break;
      };

      match next_event.event_type() {
        NoteEventType::Hit | NoteEventType::Hold => {
          if Input::singleton().is_action_just_pressed(RHYTHM_INPUT_ACTION) {
            self.hit_next_note(rating);
          }
        }
        NoteEventType::Release => {
          // Releases are handled seperately, and should not be in the main queue
          godot_warn!("Release event in main event queue");
          self.events.pop();
        }
        NoteEventType::Dodge => todo!("Dodge note handling"),
      }

      break;
    }

    let released_events: Vec<_> = self
      .held_events
      .extract_if(.., |_event| {
        !Input::singleton().is_action_pressed(RHYTHM_INPUT_ACTION)
      })
      .collect();

    for event in released_events {
      let delta = self.get_event_delta(&event);
      let rating = NoteTimingWindow::from_delta(delta).unwrap_or(NoteTimingWindow::Miss);
      self.hit_note(event, rating);
    }
  }
}

#[godot_api]
impl NoteManager {
  fn add_note(&mut self, note: DynNote) {
    let Some(event) = note.dyn_bind().get_next_event() else {
      return;
    };

    match event.event_type() {
      NoteEventType::Release => {
        self.held_events.push(AssociatedNoteEvent::new(event, note));
      }
      _ => {
        self.events.push(AssociatedNoteEvent::new(event, note));
      }
    }
  }

  /// Hit the specified not event with the provided rating
  fn hit_note(&mut self, mut event: AssociatedNoteEvent, rating: NoteTimingWindow) {
    //TODO: Check hold notes for realy release, probably with a vec of notes or something
    event.note.dyn_bind_mut().hit(rating);
    self.add_note(event.note);
    godot_print!("Note {:?}", rating);
  }

  /// Hit the next note in the event queue with the provided rating
  fn hit_next_note(&mut self, rating: NoteTimingWindow) {
    let Some(event) = self.events.pop() else {
      godot_warn!("hit_note called with empty event queue");
      return;
    };

    self.hit_note(event, rating);
  }

  // Gets the time between now and the note event in seconds
  fn get_event_delta(&self, event: &AssociatedNoteEvent) -> f64 {
    let beat_delta = self.get_current_beat() - event.at();
    beat_delta * self.conductor.bind().get_beat_duration()
  }

  pub fn get_current_beat(&self) -> f64 {
    let current_beat = self.conductor.bind().get_current_beat();
    let latency_beats =
      (self.input_latency_ms / 1000.0) / self.conductor.bind().get_beat_duration();

    current_beat - latency_beats
  }

  /// Get the position in px for note's beat poition
  #[func]
  pub fn get_note_position(&self, position_beats: f64) -> f64 {
    let beat_offset = self.get_current_beat() - position_beats;
    // Scaling changes the relative speed of the notes, so divide to keep the scroll speed relative to global units
    let scroll_speed = self.scroll_speed / (self.base().get_scale().x as f64);
    -beat_offset * scroll_speed
  }
}
