use godot::{classes::Sprite2D, prelude::*};

use crate::{
  rhythm::{
    note_manager::NoteManager,
    notes::{Note, NoteEvent, NoteEventType, NoteTimingWindow, RhythmInput},
  },
  utils::shaders::glitch::GlitchShader,
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
  #[export]
  rhythm_input: RhythmInput,

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

  fn hold_animation(&mut self) {
    let mut shader = GlitchShader::new_gd();
    let material = shader.bind().material();

    {
      let mut shader = shader.bind_mut();
      shader.set_glitch_chance(0.95);
      shader.set_chroma_offset(0.0);
      shader.set_glitch_speed(8.0);
      shader.set_slice_strength(0.3);
    }

    self.start_sprite.set_material(&material);
    self.end_sprite.set_material(&material);
    self.track_sprite.set_material(&material);
  }

  fn hit_animation(&mut self) {
    let mut tween = self
      .base_mut()
      .create_tween()
      .expect("Tween should not fail to create");

    let shader = GlitchShader::new_gd();
    let material = shader.bind().material();

    self.start_sprite.set_material(&material);
    self.end_sprite.set_material(&material);
    self.track_sprite.set_material(&material);

    let duration = 0.2;

    tween.tween_property(
      &shader,
      GlitchShader::SLICE_DROP_CHANCE_PARAM,
      &1.0.to_variant(),
      duration,
    );

    tween
      .tween_callback(&self.base().callable("queue_free"))
      .expect("tween shouldn't fail")
      .set_delay(duration);
  }
}

#[godot_dyn]
impl Note for HeldNote {
  fn get_next_event(&self) -> Option<NoteEvent> {
    match self.note_state {
      NoteState::PreHit => Some(NoteEvent::new(
        NoteEventType::Hold,
        self.rhythm_input,
        self.start_beat,
      )),
      NoteState::Holding => Some(NoteEvent::new(
        NoteEventType::Release,
        self.rhythm_input,
        self.release_beat,
      )),
      NoteState::Free => None,
    }
  }

  fn hit(&mut self, rating: NoteTimingWindow) {
    if matches!(rating, NoteTimingWindow::Miss) {
      self.note_state = NoteState::Free;
      self.hit_animation();
      return;
    }

    match self.note_state {
      NoteState::PreHit => {
        self.note_state = NoteState::Holding;
        self.hold_animation();
      }
      NoteState::Holding => {
        self.note_state = NoteState::Free;
        self.hit_animation();
      }
      NoteState::Free => (),
    }
  }
}
