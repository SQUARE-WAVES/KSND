use crate::blocks;
use blocks::BlockSequence as Seq;

pub struct Snd {
  sample_rate:usize,
  channels:Vec<Seq>
}

impl Snd {
  pub fn new(sr:usize,channels:Vec<Seq>) -> Self {
    Self {
      sample_rate:sr,
      channels
    }
  }

  pub fn from_iter<T:IntoIterator<Item=Seq>>(sr:usize,i:T) -> Self {
    Self {
      sample_rate:sr,
      channels:i.into_iter().collect()
    }
  }

  //simple data getters
  pub fn len(&self) -> usize {
    self.channels.iter().fold(0,|mx,c|mx.max(c.len()))
  }

  pub fn sample_rate(&self) -> usize {
    self.sample_rate
  }

  pub fn channels(&self) -> usize {
    self.channels.len()
  }

  pub fn channel(&self,n:usize) -> Option<&Seq> {
    self.channels.get(n)
  }

  pub fn seqs(&self) -> &[Seq] {
    &self.channels[..]
  }

  pub fn seconds(&self,sample_time:f64) -> f64 {
    sample_time / self.sample_rate as f64
  }

  pub fn interleaved_audio(&self) -> impl Iterator<Item=f32> + '_ {
    InterLeaved::new(self)
  }
}

impl AsRef<[Seq]> for Snd {
  fn as_ref(&self) -> &[Seq] {
    self.seqs()
  }
}

pub struct InterLeaved<'a> {
  snd:&'a[Seq],
  chan:usize,
  sample:usize,
  end:usize
}

impl<'a> InterLeaved<'a> {
  pub fn new(snd:&'a Snd) -> Self {
    Self {
      snd:snd.as_ref(),
      chan:0,
      sample:0,
      end:snd.len()
    }
  }
}

impl<'a> Iterator for InterLeaved<'a> {
  type Item=f32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.sample == self.end {
      return None;
    }
    
    let out = self.snd[self.chan].get_sample(&self.sample).copied().unwrap_or(0.0);

    self.chan += 1;
    
    if self.chan == self.snd.len() {
      self.chan = 0;
      self.sample += 1;
    }

    Some(out)
  }
}

impl std::fmt::Debug for Snd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f,"Sound data Len:{} sample_rate:{}",self.len(),self.sample_rate())
  }
}
