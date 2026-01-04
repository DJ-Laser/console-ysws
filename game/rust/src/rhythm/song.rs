use godot::classes::AudioStream;
use godot::prelude::*;

#[derive(Debug, Clone, Copy, GodotConvert, Var, Export)]
#[godot(transparent)]
pub struct DifficultyRating(u8);

impl DifficultyRating {
  pub fn as_u8(&self) -> u8 {
    self.0
  }
}

#[derive(Debug, GodotClass)]
#[class(tool, init, base=Resource)]
pub struct Song {
  #[export]
  name: GString,
  #[export]
  audio: OnEditor<Gd<AudioStream>>,
  #[export]
  bpm: u32,
  #[export]
  first_beat_offset_ms: u32,
  #[export]
  //beatmaps: Array<Gd<Beatmap>>,
  beatmap: OnEditor<Gd<Beatmap>>,

  base: Base<Resource>,
}

impl Song {
  pub fn name(&self) -> GString {
    self.name.clone()
  }

  pub fn audio(&self) -> Gd<AudioStream> {
    (*self.audio).clone()
  }

  pub fn bpm(&self) -> u32 {
    self.bpm
  }

  pub fn first_beat_offset_ms(&self) -> u32 {
    self.first_beat_offset_ms
  }

  pub fn first_beat_offset_secs(&self) -> f64 {
    (self.first_beat_offset_ms as f64) / 1000.0
  }

  pub fn beatmap(&self) -> Gd<Beatmap> {
    (*self.beatmap).clone()
  }
}

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct Beatmap {
  #[export]
  name: GString,

  #[init(val = DifficultyRating(0))]
  #[export]
  difficulty_rating: DifficultyRating,
}

impl Beatmap {
  pub fn difficulty_rating(&self) -> DifficultyRating {
    self.difficulty_rating
  }
}
