use godot::{classes::Sprite2D, prelude::*};

use crate::rhythm::{
  note_manager::NoteManager,
  notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow},
};

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct SingleNote {
  #[export]
  note_manager: OnEditor<Gd<NoteManager>>,

  #[export]
  hit_beat: f64,

  #[init(val = false)]
  hit: bool,

  #[init(node = "%Sprite")]
  note_sprite: OnReady<Gd<Sprite2D>>,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for SingleNote {
  fn process(&mut self, _delta: f64) {
    if self.hit {
      self.base_mut().queue_free();
      return;
    }

    let position = self.note_manager.bind().get_note_position(self.hit_beat);
    self
      .note_sprite
      .set_position(Vector2::new(position as f32, 0.0));
  }
}

#[godot_dyn]
impl Note for SingleNote {
  fn get_next_event(&self) -> Option<NoteEvent> {
    if !self.hit {
      Some(NoteEvent::new(NoteEventType::Hit, self.hit_beat))
    } else {
      None
    }
  }

  fn hit(&mut self, _rating: NoteTimingWindow) {
    self.hit = true;
  }
}
