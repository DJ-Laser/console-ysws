use std::collections::BinaryHeap;

use crate::rhythm::note_manager::AssociatedNoteEvent;

/// NoteEvents ordered by beat time
#[derive(Debug)]
struct OrderedNoteEvent(AssociatedNoteEvent);

impl Eq for OrderedNoteEvent {}
impl Ord for OrderedNoteEvent {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    // We want the soonest events first
    self.0.event.at().total_cmp(&other.0.event.at()).reverse()
  }
}
impl PartialEq for OrderedNoteEvent {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other).is_eq()
  }
}
impl PartialOrd for OrderedNoteEvent {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug, Default)]
pub struct NoteEventQueue {
  events: BinaryHeap<OrderedNoteEvent>,
}

impl NoteEventQueue {
  pub fn peek(&self) -> Option<&AssociatedNoteEvent> {
    self.events.peek().map(|e| &e.0)
  }

  pub fn pop(&mut self) -> Option<AssociatedNoteEvent> {
    self.events.pop().map(|e| e.0)
  }

  pub fn push(&mut self, event: AssociatedNoteEvent) {
    self.events.push(OrderedNoteEvent(event));
  }
}
