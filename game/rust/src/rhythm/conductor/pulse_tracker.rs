use godot::prelude::*;

use crate::rhythm::conductor::Conductor;

/// Tracks on-beat events that recur to the beat like pulsing.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct PulseTracker {
  #[export]
  conductor: OnEditor<Gd<Conductor>>,

  /// Offset of the `pulse` signal in beats
  /// negative emits `beat early`, positive emits beat late
  #[export]
  beat_offset: f64,

  /// Offset of the `pulse` signal in seconds
  /// negative emits `beat early`, positive emits beat late
  #[var(get = beat_offset_secs, set = set_beat_offset_secs)]
  #[export]
  beat_offset_secs: PhantomVar<f64>,

  // Tracks the last time we sent the `beat` signal
  #[init(val = f64::MIN)]
  last_floored_beat: f64,

  base: Base<Node>,
}

#[godot_api]
impl INode for PulseTracker {
  fn process(&mut self, _delta: f64) {
    // Handle emitting the beat signal
    let current_beat = self.conductor.bind().get_current_beat() - self.beat_offset;
    let current_floored_beat = current_beat.floor();
    if current_floored_beat > self.last_floored_beat {
      self.last_floored_beat = current_floored_beat;
      self.signals().pulse().emit(current_beat);
    }
  }
}

#[godot_api]
impl PulseTracker {
  #[func]
  pub fn beat_offset_secs(&self) -> f64 {
    self.beat_offset * self.conductor.bind().get_beat_duration()
  }

  #[func]
  pub fn set_beat_offset_secs(&mut self, value: f64) {
    self.beat_offset = value / self.conductor.bind().get_beat_duration()
  }

  /// Fires once after every whole number beat
  ///
  /// `current_beat` is the precise value for the current time in beats.
  #[signal]
  pub fn pulse(current_beat: f64);
}
