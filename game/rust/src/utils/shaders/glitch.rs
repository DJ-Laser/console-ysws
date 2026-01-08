use godot::{
  classes::{Shader, ShaderMaterial},
  prelude::*,
};

use super::shader_param_impls;

#[derive(Debug, GodotClass)]
#[class(base = RefCounted)]
pub struct GlitchShader {
  material: Gd<ShaderMaterial>,

  #[var(get = glitch_chance, set = set_glitch_chance)]
  #[export(range = (0.0, 1.0))]
  glitch_chance: PhantomVar<f64>,
  #[var(get = glitch_speed, set = set_glitch_speed)]
  #[export]
  glitch_speed: PhantomVar<f64>,
  #[var(get = slice_density, set = set_slice_density)]
  #[export]
  slice_density: PhantomVar<f64>,
  #[var(get = slice_strength, set = set_slice_strength)]
  #[export]
  slice_strength: PhantomVar<f64>,
  #[var(get = shake_strength, set = set_shake_strength)]
  #[export]
  shake_strength: PhantomVar<f64>,
  #[var(get = chroma_offset, set = set_chroma_offset)]
  #[export]
  chroma_offset: PhantomVar<f64>,
  #[var(get = noise_strength, set = set_noise_strength)]
  #[export]
  noise_strength: PhantomVar<f64>,
  #[var(get = color_flash_strength, set = set_color_flash_strength)]
  #[export]
  color_flash_strength: PhantomVar<f64>,
  #[var(get = scanline_strength, set = set_scanline_strength)]
  #[export]
  scanline_strength: PhantomVar<f64>,
  #[var(get = local_warp_strength, set = set_local_warp_strength)]
  #[export]
  local_warp_strength: PhantomVar<f64>,
  #[var(get = flip_chance, set = set_flip_chance)]
  #[export(range = (0.0, 1.0))]
  flip_chance: PhantomVar<f64>,
  #[var(get = slice_drop_chance, set = set_slice_drop_chance)]
  #[export(range = (0.0, 1.0))]
  slice_drop_chance: PhantomVar<f64>,

  base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for GlitchShader {
  fn init(base: Base<RefCounted>) -> Self {
    let shader: Gd<Shader> = load("res://shaders/glitch.gdshader");
    let mut material = ShaderMaterial::new_gd();

    material.set_shader(&shader);

    material.set_shader_parameter("glitch_chance", &1.0.to_variant());
    material.set_shader_parameter("glitch_speed", &7.0.to_variant());
    material.set_shader_parameter("slice_density", &14.0.to_variant());
    material.set_shader_parameter("slice_strength", &0.38.to_variant());
    material.set_shader_parameter("shake_strength", &0.02.to_variant());
    material.set_shader_parameter("chroma_offset", &0.016.to_variant());
    material.set_shader_parameter("noise_strength", &0.0.to_variant());
    material.set_shader_parameter("color_flash_strength", &0.0.to_variant());
    material.set_shader_parameter("scanline_strength", &0.18.to_variant());
    material.set_shader_parameter("local_warp_strength", &0.0.to_variant());
    material.set_shader_parameter("flip_chance", &0.0.to_variant());
    material.set_shader_parameter("slice_drop_chance", &0.0.to_variant());

    Self {
      material,
      glitch_chance: PhantomVar::default(),
      glitch_speed: PhantomVar::default(),
      slice_density: PhantomVar::default(),
      slice_strength: PhantomVar::default(),
      shake_strength: PhantomVar::default(),
      chroma_offset: PhantomVar::default(),
      noise_strength: PhantomVar::default(),
      color_flash_strength: PhantomVar::default(),
      scanline_strength: PhantomVar::default(),
      local_warp_strength: PhantomVar::default(),
      flip_chance: PhantomVar::default(),
      slice_drop_chance: PhantomVar::default(),
      base,
    }
  }
}

shader_param_impls! {
  GlitchShader {
    glitch_chance: f64,
    glitch_speed: f64,
    slice_density: f64,
    slice_strength: f64,
    shake_strength: f64,
    chroma_offset: f64,
    noise_strength: f64,
    color_flash_strength: f64,
    scanline_strength: f64,
    local_warp_strength: f64,
    flip_chance: f64,
    slice_drop_chance: f64
  }
}
