use super::sliding_window;
use super::functions;

pub fn window<const SZ:usize,S,F>(mut src:S,ratio:f64,interp:F) -> Vec<f32> 
where
  S:Iterator<Item=f32>,
  F:Fn(f32,&sliding_window::Window<SZ>) -> f32
{
  let mut window = sliding_window::IterWindow::<S,SZ>::new(&mut src);

  let step :f64 = ratio;
  let mut pos :f64 = 0.0;
  let mut done = false;
  let mut outvec : Vec<f32> = vec![];

  loop {
    let fract = pos.fract() as f32;
    let out = interp(fract,window.buff());
    outvec.push(out);

    let next = pos+step;
    let steps = (next - pos.floor()).floor() as usize;
    
    for _ in 0..steps {
      done = window.pull();
    }

    pos = next;

    if done {
      break;
    }
  }

  outvec
}

pub fn win_sinc7(fract:f32,window:&sliding_window::Window<14>) -> f32 {
  let wn_sinc = |ph| functions::sinc(ph) * functions::blackman_window(ph,7.0);

  let mut out = wn_sinc(fract) * window.get(-1);
  out += wn_sinc(fract + 1.0) * window.get(-2);
  out += wn_sinc(fract + 2.0) * window.get(-3);
  out += wn_sinc(fract + 3.0) * window.get(-4);
  out += wn_sinc(fract + 4.0) * window.get(-5);
  out += wn_sinc(fract + 5.0) * window.get(-6);
  out += wn_sinc(fract + 6.0) * window.get(-7);

  let fract = 1.0 - fract;
  out += wn_sinc(fract) * window.get(0);
  out += wn_sinc(fract + 1.0) * window.get(1);
  out += wn_sinc(fract + 2.0) * window.get(2);
  out += wn_sinc(fract + 3.0) * window.get(3);
  out += wn_sinc(fract + 4.0) * window.get(4);
  out += wn_sinc(fract + 5.0) * window.get(5);
  out += wn_sinc(fract + 6.0) * window.get(6);

  out
}

pub fn win_sinc3(fract:f32,window:&sliding_window::Window<6>) -> f32 {
  let wn_sinc = |ph| functions::sinc(ph) * functions::blackman_window(ph,3.0);

  let mut out = wn_sinc(fract) * window.get(-1);
  out += wn_sinc(fract + 1.0) * window.get(-2);
  out += wn_sinc(fract + 2.0) * window.get(-3);

  let fract = 1.0 - fract;
  out += wn_sinc(fract) * window.get(0);
  out += wn_sinc(fract + 1.0) * window.get(1);
  out += wn_sinc(fract + 2.0) * window.get(2);

  out
}

pub fn lin(fract:f32 ,window:&sliding_window::Window<2>) -> f32 {
  fract * window.get(0) + (1.0 - fract) * window.get(-1)
}
