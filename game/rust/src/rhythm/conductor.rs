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
#[class(init, base=Node)]
struct Conductor {
  #[export]
  song: OnEditor<Gd<Song>>,

  /// If `true`, the song is paused.
  /// Setting this property will pause/unpause the song
  #[export]
  #[var(get = is_paused, set = set_paused)]
  is_paused: PhantomVar<bool>,

  #[export_group(name = "Nodes")]
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

  song_timer: Option<SongTimer>,

  base: Base<Node>,
}

#[godot_api]
impl INode for Conductor {
  fn ready(&mut self) {
    self.base_mut().set_process_mode(ProcessMode::ALWAYS);

    self.play();
  }

  fn process(&mut self, _delta: f64) {
    if self.is_paused() {
      return;
    }

    if let Some(song_timer) = &mut self.song_timer {
      song_timer.update(self.song_player.get_playback_position() as f64);
    }
  }

  fn physics_process(&mut self, delta: f64) {
    if self.is_paused() {
      return;
    }

    if let Some(song_timer) = &mut self.song_timer {
      song_timer.fixed_update(delta);
    }
  }
}

#[godot_api]
impl Conductor {
  #[func]
  fn is_paused(&self) -> bool {
    self.song_player.get_stream_paused()
  }

  #[func]
  fn set_paused(&mut self, paused: bool) {
    self.song_player.set_stream_paused(paused)
  }

  #[func]
  fn play(&mut self) {
    let song = self.song.bind();
    self.song_player.set_stream(&song.audio());

    let mut song_timer = SongTimer::new(
      &song,
      OneEuroFilter::new(self.allowed_jitter, self.lag_reduction, self.allowed_jitter),
    );

    self.song_player.play();
    song_timer.reset_song_start_time();
    self.song_timer = Some(song_timer)
  }

  #[func]
  fn stop(&mut self) {
    self.song_player.stop();
    self.song_timer = None;
  }

  fn get_current_beat(&self) -> Option<f64> {
    self
      .song_timer
      .as_ref()
      .map(|song_timer| song_timer.get_current_beat())
  }
}
