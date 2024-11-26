use std::f32::consts::PI;

pub fn sinc(ph:f32) -> f32 {
  if ph == 0.0 {
    1.0
  }
  else {
    let norm_ph = ph * PI;
    norm_ph.sin()/norm_ph
  }
}

pub fn blackman_window(ph:f32,scale:f32) -> f32 {
  let ph = (ph/scale + 1.0) * std::f32::consts::PI;
  let cos = ph.cos();
  let cos2 = (2.0*ph).cos();
  0.42659 - (0.49656*cos) + (0.076489*cos2)
}
