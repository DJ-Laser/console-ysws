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

  #[export]
  #[init(val = 400.0)]
  scroll_speed: f64,

  #[export]
  #[init(val = 25.0)]
  input_latency_ms: f64,

  events: NoteEventQueue,

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
        match next_event.event_type() {
          // Dodge notes should get handled befote the miss window
          NoteEventType::Dodge => godot_warn!("Dodge note not handles before miss timing"),
          _ => {
            self.hit_note(NoteTimingWindow::Miss);
          }
        }

        continue;
      }

      let Some(rating) = NoteTimingWindow::from_delta(delta) else {
        // Event was too early, wait till next frame
        break;
      };

      match next_event.event_type() {
        NoteEventType::Hit | NoteEventType::Hold => {
          if Input::singleton().is_action_just_pressed(RHYTHM_INPUT_ACTION) {
            self.hit_note(rating);
          }
        }
        NoteEventType::Release => {
          if Input::singleton().is_action_just_released(RHYTHM_INPUT_ACTION) {
            self.hit_note(rating);
          }
        }
        NoteEventType::Dodge => todo!("Dodge note handling"),
      }

      // The current event was handled, break
      break;
    }
  }
}

#[godot_api]
impl NoteManager {
  fn add_note(&mut self, note: DynNote) {
    let Some(event) = note.dyn_bind().get_next_event() else {
      return;
    };

    self.events.push(AssociatedNoteEvent::new(event, note));
  }

  /// Hit the next note in the event queue with the provided rating
  fn hit_note(&mut self, rating: NoteTimingWindow) {
    let Some(mut event) = self.events.pop() else {
      godot_warn!("hit_note called with empty event queue");
      return;
    };

    //TODO: Check hold notes for realy release, probably with a vec of notes or something
    event.note.dyn_bind_mut().hit(rating);
    self.add_note(event.note);
    godot_print!("Note {:?}", rating);
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
    let time_offset =
      (self.get_current_beat() - position_beats) * self.conductor.bind().get_beat_duration();
    -time_offset * self.scroll_speed
  }
}
