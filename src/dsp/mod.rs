mod functions;
mod sliding_window;
mod interpolate;

pub fn window_resample<S:Iterator<Item=f32>>(src:S,ratio:f64,q:usize) -> Vec<f32> {
  match q {
    0 => interpolate::window(src,ratio,interpolate::lin),
    1 => interpolate::window(src,ratio,interpolate::win_sinc3),
    _ => interpolate::window(src,ratio,interpolate::win_sinc7)
  }
}
