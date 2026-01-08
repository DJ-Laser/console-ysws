use godot::{
  classes::{AudioStreamPlayer, node::ProcessMode},
  prelude::*,
};

use crate::rhythm::{
  conductor::{filter::OneEuroFilter, timer::SongTimer},
  song::Song,
};

mod filter;
mod timer;

/// Handles Playing the song audio and synchronizing game events to the music
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Conductor {
  #[export]
  song: OnEditor<Gd<Song>>,

  // /// If `true`, the song is paused.
  // /// Setting this property will pause/unpause the song
  // #[var(get = is_paused, set = set_paused)]
  // #[export]
  // is_paused: PhantomVar<bool>,
  /// AudioStreamPlayer for the main song audio
  #[export]
  song_player: OnEditor<Gd<AudioStreamPlayer>>,

  #[export_group(name = "Filter Parameters")]
  /// `cutoff` for the 1€ filter. Decrease to reduce jitter.
  #[export]
  #[init(val = 0.1)]
  allowed_jitter: f64,
  /// `beta` for the 1€ filter. Increase to reduce lag.
  #[export]
  #[init(val = 5.0)]
  lag_reduction: f64,

  #[init(val = OnReady::manual())]
  song_timer: OnReady<SongTimer>,

  base: Base<Node>,
}

#[godot_api]
impl INode for Conductor {
  fn ready(&mut self) {
    self.base_mut().set_process_mode(ProcessMode::ALWAYS);
    self.song_timer.init(SongTimer::new(OneEuroFilter::new(
      self.allowed_jitter,
      self.lag_reduction,
      self.allowed_jitter,
    )));

    self.play();
  }

  fn process(&mut self, _delta: f64) {
    if self.is_paused() {
      return;
    }

    self
      .song_timer
      .update(self.song_player.get_playback_position() as f64);
  }

  fn physics_process(&mut self, delta: f64) {
    if self.is_paused() {
      return;
    }

    self.song_timer.fixed_update(delta);
  }
}

#[godot_api]
impl Conductor {
  #[func]
  pub fn is_paused(&self) -> bool {
    self.song_player.get_stream_paused()
  }

  #[func]
  pub fn set_paused(&mut self, paused: bool) {
    self.song_player.set_stream_paused(paused)
  }

  #[func]
  pub fn play(&mut self) {
    let song = self.song.bind();
    self.song_player.set_stream(&song.audio());
    self.song_timer.set_song(&song);

    self.song_player.play();
    self.song_timer.reset_song_start_time();
  }

  #[func]
  pub fn stop(&mut self) {
    self.song_player.stop();
  }

  #[func]
  pub fn get_current_beat(&self) -> f64 {
    self.song_timer.get_current_beat()
  }

  #[func]
  pub fn get_beat_duration(&self) -> f64 {
    self.song_timer.get_beat_duration()
  }
}
