use godot::{classes::Sprite2D, prelude::*};

use crate::rhythm::conductor::Conductor;

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct HeldNote {
  #[export]
  conductor: OnEditor<Gd<Conductor>>,

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
    //let current_beat = self.conductor.bind().get_current_beat();
    self.position_track();
  }
}

#[godot_api]
impl HeldNote {
  fn position_track(&mut self) {
    let track_width = 1024_f32;

    let mut track_region = self.track_sprite.get_region_rect();
    track_region.size.x = track_width;
    self.track_sprite.set_region_rect(track_region);

    self.track_sprite.set_position(Vector2 {
      x: track_width / 2.0,
      y: 0.0,
    });

    self.end_sprite.set_position(Vector2 {
      x: track_width,
      y: 0.0,
    });
  }
}
