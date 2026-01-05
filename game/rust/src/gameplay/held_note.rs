use godot::{classes::Sprite2D, prelude::*};

use crate::rhythm::{
  note_manager::NoteManager,
  notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow},
};

#[derive(Debug)]
enum NoteState {
  PreHit,
  Holding,
  Free,
}

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct HeldNote {
  #[export]
  note_manager: OnEditor<Gd<NoteManager>>,

  #[export]
  start_beat: f64,
  #[export]
  release_beat: f64,

  #[init(val = NoteState::PreHit)]
  note_state: NoteState,

  #[init(node = "%Start")]
  start_sprite: OnReady<Gd<Sprite2D>>,
  #[init(node = "%Track")]
  track_sprite: OnReady<Gd<Sprite2D>>,
  #[init(node = "%End")]
  end_sprite: OnReady<Gd<Sprite2D>>,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for HeldNote {
  fn process(&mut self, _delta: f64) {
    if matches!(self.note_state, NoteState::Free) {
      self.base_mut().queue_free();
      return;
    }

    let note_manager = self.note_manager.bind();

    let start_position = if matches!(self.note_state, NoteState::PreHit) {
      note_manager.get_note_position(self.start_beat)
    } else {
      0.0
    };

    let end_position = if note_manager.get_current_beat() <= self.release_beat {
      note_manager.get_note_position(self.release_beat)
    } else {
      0.0
    };

    drop(note_manager);

    self.position_track(start_position, end_position);
  }
}

#[godot_api]
impl HeldNote {
  fn position_track(&mut self, start_position: f64, end_position: f64) {
    self
      .start_sprite
      .set_position(Vector2::new(start_position as f32, 0.0));

    let track_width = (start_position - end_position).abs() as f32;
    let mut track_region = self.track_sprite.get_region_rect();
    track_region.size.x = track_width;
    self.track_sprite.set_region_rect(track_region);

    self
      .track_sprite
      .set_position(Vector2::new(start_position as f32 + track_width / 2.0, 0.0));

    self
      .end_sprite
      .set_position(Vector2::new(end_position as f32, 0.0));
  }
}

#[godot_dyn]
impl Note for HeldNote {
  fn get_next_event(&self) -> Option<NoteEvent> {
    match self.note_state {
      NoteState::PreHit => Some(NoteEvent::new(NoteEventType::Hold, self.start_beat)),
      NoteState::Holding => Some(NoteEvent::new(NoteEventType::Release, self.release_beat)),
      NoteState::Free => None,
    }
  }

  fn hit(&mut self, rating: NoteTimingWindow) {
    if matches!(rating, NoteTimingWindow::Miss) {
      self.note_state = NoteState::Free;
      return;
    }

    match self.note_state {
      NoteState::PreHit => self.note_state = NoteState::Holding,
      NoteState::Holding => self.note_state = NoteState::Free,
      NoteState::Free => (),
    }
  }
}
