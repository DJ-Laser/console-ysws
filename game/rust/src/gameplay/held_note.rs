use godot::{classes::Sprite2D, prelude::*};

use crate::rhythm::{
  conductor::Conductor,
  notes::{Note, NoteTimingWindow},
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
  conductor: OnEditor<Gd<Conductor>>,

  #[export]
  start_beat: f64,
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

    let conductor = self.conductor.bind();

    let end_position = conductor.get_note_position(self.release_beat).max(0.0);
    let start_position = if matches!(self.note_state, NoteState::PreHit) {
      conductor.get_note_position(self.start_beat)
    } else {
      0.0
    };

    drop(conductor);

    self.position_track(start_position, end_position);
  }
}

#[godot_api]
impl HeldNote {
  fn position_track(&mut self, start_position: f64, end_position: f64) {
    self
      .start_sprite
      .set_position(Vector2::new(start_position as f32, 0.0));

    let track_width = (end_position - start_position) as f32;
    let mut track_region = self.track_sprite.get_region_rect();
    track_region.size.x = track_width;
    self.track_sprite.set_region_rect(track_region);

    self
      .track_sprite
      .set_position(Vector2::new(track_width / 2.0, 0.0));

    self
      .end_sprite
      .set_position(Vector2::new(end_position as f32, 0.0));
  }
}

impl Note for HeldNote {
  fn get_next_event(&self) -> Option<crate::rhythm::notes::NoteEvent> {
    todo!()
  }

  fn hit(&mut self, rating: NoteTimingWindow) {
    if matches!(rating, NoteTimingWindow::Miss) {
      self.note_state = NoteState::Free;
      return;
    }

    match self.note_state {
      NoteState::PreHit => self.note_state = NoteState::Holding,
      NoteState::Holding => self.note_state = NoteState::Free,
      NoteState::Free => {}
    }
  }
}
