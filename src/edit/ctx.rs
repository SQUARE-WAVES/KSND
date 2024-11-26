use std::sync::Arc;

use crate::{
  util::{
    Mask,
    Ruler,
    Region
  },
  blocks::BlockSequence as Seq,
  snd::Snd
};

#[derive(Clone)]
pub struct Ctx {
  pub snd:Arc<Snd>,
  pub cursor:Option<f64>,
  pub selection:Option<f64>,
  pub ruler:Option<Ruler>,
  pub channels:Mask,
  pub zoom:f64,
  pub slide:f64,
  pub loop_mode:bool
}

impl Ctx {
  pub fn flip(&self,new_snd:Arc<Snd>) -> Self {
    let snd_len = new_snd.len() as f64;
    let (nc,ns)= match (self.cursor,self.selection){
      (Some(c),Some(s))=> {
        let new_c = c.min(snd_len);
        let new_s = if s < 0.0 {
          s.max(-new_c)
        }
        else {
          s.min(snd_len - new_c)
        };

        (Some(new_c),Some(new_s))
      },
      (Some(c),None) => {
        let new_c = if c > snd_len {
          None
        }
        else {
          Some(c)
        };

        (new_c,None)
      },
      _ => (None,None)
    };

    Self {
      snd:new_snd,
      cursor:nc,
      selection:ns,
      ruler:self.ruler,
      channels:self.channels,
      zoom:self.zoom,
      slide:self.slide,
      loop_mode:self.loop_mode
    }
  }

  //this is the region being viewed!
  pub fn region(&self) -> Region {
    let scale = self.snd.len() as f64;
    let start = self.slide * scale;
    let end = start + self.zoom * scale;
    (start,end).into()
  }

  //this is the region that has been selected
  pub fn selected_region(&self) -> Option<Region> {
    match (self.cursor,self.selection) {
      (Some(pt),Some(len)) => {
        Some((pt,pt+len).into())
      },
      _ => None
    }
  }

  pub fn sample_region(&self) -> (usize,usize) {
    self.selected_region().map(|r|r.sample_range()).unwrap_or((0,self.snd.len()))
  }

  pub fn window_width(&self) -> f64 {
    self.snd.len() as f64 * self.zoom
  }

  pub fn len(&self) -> f64 {
    self.snd.len() as f64
  }

  pub fn seqs(&self)-> impl Iterator<Item=(usize,bool,&Seq)> {
    self.snd.seqs().iter().enumerate().map(|(i,s)|(i,self.channels.is_on(i),s))
  }

  pub fn copy(&self) -> Arc<Snd> {
    let (s,e) = self.selected_region().map(|r|r.sample_range()).unwrap_or((0,self.snd.len()));
    let sqs = self.snd.seqs().iter().enumerate().filter(|(i,_)|self.channels.is_on(*i));
    let new_snd = Snd::from_iter(self.snd.sample_rate(),sqs.map(|(_,seq)|{
      seq.sub_seq(s..e)
    }));

    new_snd.into()
  }

  pub fn default_click(&mut self,x:f64) {
    let window = self.region();

    let place = window.len() * x;
    let place = place.floor() + window.start();

    self.cursor = Some(place);
    self.selection = None;
  }

  pub fn default_drag(&mut self,x:f64) {
    let window = self.region();
    let window_x = x * window.len();
    let window_x = window_x + window.start();
    let window_x = window_x.floor();
    
    let new_sel = match self.cursor {
      None => None,
      Some(c) if (window_x - c).abs() < 1.0 => None,
      Some(c) => Some(window_x - c),
    };

    self.selection = new_sel;
  }
}

impl From<Arc<Snd>> for Ctx {
  fn from(s:Arc<Snd>) -> Self {
    Self{
      snd:s,
      cursor:None,
      selection:None,
      ruler:None,
      channels:Default::default(),
      zoom:1.0,
      slide:0.0,
      loop_mode:false
    }
  }
}



