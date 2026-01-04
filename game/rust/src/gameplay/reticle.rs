use godot::{classes::Sprite2D, prelude::*};

use crate::rhythm::conductor::Conductor;

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct Reticle {
  #[export]
  conductor: OnEditor<Gd<Conductor>>,

  #[init(node = "%Crosshair")]
  crosshair_sprite: OnReady<Gd<Sprite2D>>,
  #[init(node = "%Ring")]
  ring_sprite: OnReady<Gd<Sprite2D>>,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Reticle {
  fn process(&mut self, _delta: f64) {
    let current_beat = self.conductor.bind().get_current_beat();

    self
      .crosshair_sprite
      .set_rotation((current_beat / 2.0) as f32);
    self.ring_sprite.set_rotation((-current_beat / 2.0) as f32);
  }
}
