use godot::{
  classes::{ShaderMaterial, Sprite2D},
  prelude::*,
};

use crate::{
  rhythm::{
    note_manager::NoteManager,
    notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow},
  },
  utils::shaders::glitch::GlitchShader,
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

    let mut tween = self
      .base_mut()
      .create_tween()
      .expect("Tween should not fail to create");

    let shader = GlitchShader::new_gd();
    let material = shader.bind().material();
    let material = material.cast::<ShaderMaterial>();

    self.note_sprite.set_material(&material);

    let duration = 0.1;

    tween.tween_property(
      &shader,
      GlitchShader::SLICE_DROP_CHANCE_PARAM,
      &1.0.to_variant(),
      duration,
    );

    /*tween
    .tween_callback(&self.base().callable("queue_free"))
    .expect("tween shouldn't fail")
    .set_delay(duration);*/
  }
}
