use godot::{
  classes::{AudioServer, Time},
  prelude::*,
};

use crate::rhythm::{conductor::filter::OneEuroFilter, song::Song};

/// Measures the position of the song
pub struct SongTimer {
  bpm: f64,
  time_to_first_beat: f64,

  cached_audio_latency: f64,
  audio_server_position: f64,

  /// System clock time when the song started playing
  system_clock_start_time: f64,
  system_clock_position: f64,

  filter: OneEuroFilter,
  /// Difference between audio server position and system clock position, filtered for jitter
  filtered_position_discrepancy: f64,
}

impl SongTimer {
  pub fn new(filter: OneEuroFilter) -> Self {
    Self {
      bpm: 0.0,
      time_to_first_beat: 0.0,

      system_clock_start_time: 0.0,
      system_clock_position: 0.0,

      cached_audio_latency: AudioServer::singleton().get_output_latency(),
      audio_server_position: 0.0,

      filter,
      filtered_position_discrepancy: 0.0,
    }
  }

  pub fn set_song(&mut self, song: &Song) {
    self.bpm = song.bpm() as f64;
    self.time_to_first_beat = song.first_beat_offset_secs();
  }

  /// Update the system and audio server positions
  /// Call every frame for the best accuracy
  pub fn update(&mut self, playback_position: f64) {
    // Handle a web bug where AudioServer.get_time_since_last_mix() occasionally
    // returns unsigned 64-bit integer max value. This is likely due to minor
    // timing issues between the main/audio threads, thus causing an underflow
    // in the engine code.
    let time_since_last_mix = {
      let t = AudioServer::singleton().get_time_since_last_mix();
      if t > 1000.0 { 0.0 } else { t }
    };

    // Calculate jittery song position estimate from audio server data
    self.audio_server_position = (playback_position) + time_since_last_mix
      - self.time_to_first_beat
      - self.cached_audio_latency;

    let system_time_secs = (Time::singleton().get_ticks_usec() as f64) / 1000000.0;
    self.system_clock_position = system_time_secs - self.system_clock_start_time;
  }

  /// Update the filtered position
  /// Call in a fixed loop such as `physics_process` for the best consistency
  pub fn fixed_update(&mut self, delta: f64) {
    let position_discrepancy = self.audio_server_position - self.system_clock_position;
    self.filtered_position_discrepancy = self.filter.filter(position_discrepancy, delta);
  }

  /// Resets the song start time to the current moment.
  /// Call this after calling `play()` on the `AudioStreamPlayer`
  pub fn reset_song_start_time(&mut self) {
    let system_time_secs = (Time::singleton().get_ticks_usec() as f64) / 1000000.0;
    let time_to_next_mix = AudioServer::singleton().get_time_to_next_mix();

    self.system_clock_start_time =
      system_time_secs + self.time_to_first_beat + time_to_next_mix + self.cached_audio_latency;
  }

  /// Get the song position in seconds
  pub fn get_current_position(&self) -> f64 {
    self.system_clock_position + self.filtered_position_discrepancy
  }

  pub fn get_beat_duration(&self) -> f64 {
    if self.bpm == 0.0 {
      return 0.0;
    }

    60.0 / self.bpm
  }

  /// Get the song position in baets
  pub fn get_current_beat(&self) -> f64 {
    if self.get_beat_duration() == 0.0 {
      return 0.0;
    }

    self.get_current_position() / self.get_beat_duration()
  }
}
