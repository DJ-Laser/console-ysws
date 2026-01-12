use godot::{
  classes::{
    Sprite2D, Tween,
    tween::{EaseType, TransitionType},
  },
  prelude::*,
};

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

  pulse_tween: Option<Gd<Tween>>,

  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Reticle {
  fn ready(&mut self) {
    let mut pulse_tracker = self.conductor.bind().new_pulse_tracker();
    pulse_tracker.bind_mut().set_beat_offset_secs(-0.03);
    self.base_mut().add_child(&pulse_tracker);

    pulse_tracker
      .signals()
      .pulse()
      .connect_other(&self.to_gd(), Self::pulse);
  }

  fn process(&mut self, _delta: f64) {
    let current_beat = self.conductor.bind().get_current_beat();

    self.ring_sprite.set_rotation((-current_beat / 2.0) as f32);
  }
}

impl Reticle {
  fn pulse(&mut self, _current_beat: f64) {
    if let Some(mut tween) = self.pulse_tween.take() {
      tween.kill();
    }

    let mut tween = self.base_mut().create_tween().unwrap();

    let scale_up_duration = 0.03;
    let max_scale = Variant::from(Vector2::new(1.3, 1.3));
    let scale_down_duration = 0.5;
    let normal_scale = Variant::from(Vector2::new(1.0, 1.0));

    tween.set_trans(TransitionType::QUAD);
    tween.set_ease(EaseType::IN);
    tween.tween_property(&*self.ring_sprite, "scale", &max_scale, scale_up_duration);
    tween.parallel().unwrap().tween_property(
      &*self.crosshair_sprite,
      "scale",
      &max_scale,
      scale_up_duration,
    );

    tween.set_ease(EaseType::OUT);
    tween.tween_property(
      &*self.ring_sprite,
      "scale",
      &normal_scale,
      scale_down_duration,
    );
    tween.parallel().unwrap().tween_property(
      &*self.crosshair_sprite,
      "scale",
      &normal_scale,
      scale_down_duration,
    );

    self.pulse_tween = Some(tween);
  }
}
