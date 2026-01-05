#[derive(Debug, Clone, Copy)]
pub enum NoteTimingWindow {
  Perfect,
  Great,
  Good,
  Miss,
}

impl NoteTimingWindow {
  fn hit_window_ms(&self) -> f64 {
    match self {
      NoteTimingWindow::Perfect => 50.0,
      NoteTimingWindow::Great => 100.0,
      NoteTimingWindow::Good => 150.0,
      NoteTimingWindow::Miss => 300.0,
    }
  }

  fn hit_window_secs(&self) -> f64 {
    self.hit_window_ms() / 1000.0
  }

  /// Evaluate the hit delta (secs) return a rating
  /// If `None` is returned, the note was too early to hit
  pub fn from_delta(delta: f64) -> Option<Self> {
    if delta > Self::Miss.hit_window_secs() {
      return None;
    }

    for rating in [Self::Perfect, Self::Great, Self::Good] {
      if delta.abs() <= rating.hit_window_secs() {
        return Some(rating);
      }
    }

    Some(Self::Miss)
  }
}

#[derive(Debug, Clone, Copy)]
pub enum NoteEventType {
  Hit,
  Hold,
  Release,
  Dodge,
}

/// Note type, which input to press, and time in beats
#[derive(Debug)]
pub struct NoteEvent {
  event_type: NoteEventType,
  // input: ??
  at: f64,
}

impl NoteEvent {
  pub fn event_type(&self) -> NoteEventType {
    self.event_type
  }

  pub fn at(&self) -> f64 {
    self.at
  }
}

pub trait Note {
  // Get the next note event if the note still has any
  fn get_next_event(&self) -> Option<NoteEvent>;

  // Hit the current note event
  // `get_next_event` should return a new event or `None`
  fn hit(&mut self, rating: NoteTimingWindow);
}
