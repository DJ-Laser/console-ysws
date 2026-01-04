//! 1€ Filter to smooth out jitter in the recorded song position from the audio server
//! https://gery.casiez.net/1euro/

use std::f64;

struct LowPassFilter {
  hat_x_prev: f64,
}

impl LowPassFilter {
  fn new() -> Self {
    Self { hat_x_prev: 0.0 }
  }

  fn filter(&mut self, x: f64, alpha: f64) -> f64 {
    let hat_x = alpha * x + (1.0 - alpha) * self.hat_x_prev;
    self.hat_x_prev = hat_x;
    hat_x
  }
}

pub struct OneEuroFilter {
  min_cutoff: f64,
  beta: f64,
  d_cutoff: f64,

  x_filter: LowPassFilter,
  dx_filter: LowPassFilter,
}

impl OneEuroFilter {
  pub fn new(min_cutoff: f64, beta: f64, d_cutoff: f64) -> Self {
    Self {
      min_cutoff,
      beta,
      d_cutoff,
      x_filter: LowPassFilter::new(),
      dx_filter: LowPassFilter::new(),
    }
  }

  fn alpha(rate: f64, cutoff: f64) -> f64 {
    let tau = 1.0 / (f64::consts::TAU * cutoff);
    let te = 1.0 / rate;
    1.0 / (1.0 / tau / te)
  }

  pub fn filter(&mut self, x: f64, delta: f64) -> f64 {
    let rate = 1.0 / delta;
    let dx = (x - self.x_filter.hat_x_prev) * rate;

    let edx = self.dx_filter.filter(dx, Self::alpha(rate, self.d_cutoff));
    let cutoff = self.min_cutoff + self.beta * edx.abs();
    self.x_filter.filter(x, Self::alpha(rate, cutoff))
  }
}
