use godot::prelude::*;

use crate::rhythm::{
  conductor::Conductor,
  note_manager::event_queue::NoteEventQueue,
  notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow},
};

mod event_queue;

type DynNote = DynGd<RefCounted, dyn Note>;

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
  scroll_speed: f64,

  events: NoteEventQueue,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for NoteManager {
  fn ready(&mut self) {}

  fn process(&mut self, _delta: f64) {
    while let Some(next_event) = self.events.peek() {
      let delta = self.get_event_delta(next_event);

      let Some(rating) = NoteTimingWindow::from_delta(delta) else {
        // Event was too early, wait till next frame
        break;
      };

      match next_event.event_type() {
        NoteEventType::Hit => todo!(),
        NoteEventType::Hold => todo!(),
        NoteEventType::Release => todo!(),
        NoteEventType::Dodge => todo!(),
      }
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

  // Gets the time between now and the note event in seconds
  fn get_event_delta(&self, event: &AssociatedNoteEvent) -> f64 {
    let conductor = self.conductor.bind();
    let beat_delta = conductor.get_current_beat() - event.at();
    beat_delta * conductor.get_beat_duration()
  }
}
