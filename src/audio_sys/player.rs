use std::sync::Arc;

use crate::snd::Snd;
use crate::util::lerp;

use super::Id;

pub struct Player {
  id:Id,
  snd:Arc<Snd>,
  start:f64,
  end:f64,
  play_loop:bool,
  play_head:f64,
  ratio:f64,
}

impl Player {
  pub fn new(id:Id,snd:Arc<Snd>,sr:f64,start:f64,end:f64,ph:f64,lp:bool) -> Self {
    let ratio = snd.sample_rate() as f64/sr;

    Self {
      id,
      snd,
      start,
      end,
      play_loop:lp,
      play_head:ph,
      ratio
    }
  }

  pub fn id(&self) -> Id {
    self.id
  }

  fn fill_outs(&self,out:&mut [f32]) {
    let floor = self.play_head.floor();
    let fract = self.play_head.fract() as f32;
    let channels = self.snd.channels();
    
    for i in 0..out.len() {
      if i <  channels {
        let idx = floor as usize;
        let left = self.snd.channel(i).and_then(|s|s.get_sample(&idx)).unwrap_or(&0.0);
        let right = self.snd.channel(i).and_then(|s|s.get_sample(&(idx+1))).unwrap_or(&0.0);
        out[i] = lerp(*left,*right,fract);
      }
      else {
        out[i] = out[channels-1];
      }
    }
  }

  pub fn tick(&mut self,out:&mut [f32]) -> bool {
    self.fill_outs(out);
    self.play_head += self.ratio;

    match (self.play_head,self.play_loop) {
      (ph,false) if ph >= self.end => {
        false
      },
      (ph,true) if ph >= self.end => {
        self.play_head = (self.play_head - self.end) + self.start;
        true
      },
      _ => true
    }
  }

  pub fn play_pos(&self) -> usize {
    self.play_head.floor() as usize
  }
}

