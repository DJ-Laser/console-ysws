use godot::{
  classes::{AudioServer, AudioStreamPlayer},
  prelude::*,
};

use crate::rhythm::song::Song;

#[derive(GodotClass)]
#[class(base=Node)]
struct Conductor {
  #[export]
  song: OnEditor<Gd<Song>>,

  song_player: OnReady<Gd<AudioStreamPlayer>>,
  audio_latency: f64,

  song_position_secs: f64,
  last_recorded_beat: u32,

  base: Base<Node>,
}

#[godot_api]
impl INode for Conductor {
  fn init(base: Base<Node>) -> Self {
    Self {
      song: OnEditor::default(),

      song_player: OnReady::from_node("%MainSongPlayer"),
      audio_latency: AudioServer::singleton().get_output_latency(),

      song_position_secs: 0.0,
      last_recorded_beat: 0,

      base,
    }
  }

  fn ready(&mut self) {
    let audio_stream = self.song.bind().get_audio();
    self.song_player.set_stream(audio_stream.as_ref());
    self.song_player.play();
  }

  fn process(&mut self, _delta: f64) {
    self.update_song_pos();
  }
}

#[godot_api]
impl Conductor {
  fn update_song_pos(&mut self) {
    if !self.song_player.is_playing() {
      self.song_position_secs = 0.0;
      self.last_recorded_beat = 0;
      return;
    }

    let estimated_position_secs: f64 = (self.song_player.get_playback_position() as f64)
      + AudioServer::singleton().get_time_since_last_mix()
      - self.audio_latency;

    if estimated_position_secs > self.song_position_secs {
      self.song_position_secs = estimated_position_secs;
    }

    let song_position_beats = self.song_position_secs / 60.0 * (self.song.bind().bpm() as f64);
    let song_position_beats = song_position_beats.floor() as u32;

    if song_position_beats > self.last_recorded_beat {
      self.last_recorded_beat = song_position_beats;
      godot_print!("Song pos: {}", song_position_beats);
    }
  }
}
