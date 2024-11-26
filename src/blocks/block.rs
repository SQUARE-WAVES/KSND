mod arc_block;
mod silent_block;

use arc_block::ArcBlock;
use silent_block::SilentBlock;

#[derive(Clone)]
pub enum Block {
  Arc(ArcBlock),
  Silent(SilentBlock)
}

impl Block {
  pub fn data(samples:Vec<f32>) -> Block {
    let ab = ArcBlock::from_samples(samples.into());
    Self::Arc(ab)
  }

  pub fn silence(len:usize) -> Block {
    let sb = SilentBlock::new(len);
    Self::Silent(sb)
  }

  //THIS IS THE "interface" that different kinds of blocks
  //need to use
  pub fn len(&self) -> usize {
    match self {
      Block::Arc(ab) => ab.len(),
      Block::Silent(sb) => sb.len()
    }
  }

  pub fn rng<R:std::ops::RangeBounds<usize>>(&self,r:R) -> Self {
    match self {
      Block::Arc(ab) => Block::Arc(ab.rng(r)),
      Block::Silent(sb) => Block::Silent(sb.rng(r))
    }
  }

  pub fn get_sample(&self,index:usize) -> Option<&f32> {
    match self {
      Block::Arc(ab) => ab.get_sample(index),
      Block::Silent(sb) => sb.get_sample(index)
    }
  }

  pub fn summary(&self) -> (f32,f32) {
    match self {
      Block::Arc(ab) => ab.summary(),
      Block::Silent(sb) => sb.summary()
    }
  }

  pub fn samples(&self) -> BlockRunner {
    BlockRunner::new(self.clone())
  }

  pub fn into_samples(self) -> BlockRunner {
    BlockRunner::new(self)
  }

  pub fn map<P:FnMut(f32)->f32>(&self,proc:P) -> Self {
    let rendered : Vec<f32> = self.samples().map(proc).collect();
    Block::Arc(ArcBlock::from_samples(rendered.into()))
  }
}

//This is necessary to have a named iterator type
//for all different block types
pub struct BlockRunner {
  b:Block,
  start:usize,
  end:usize
}

impl BlockRunner {
  pub fn new(b:Block) -> Self {
    let end = b.len();

    Self {
      b,
      start:0,
      end
    }
  }
}

impl Iterator for BlockRunner {
  type Item=f32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
     return None
    }

    let out = self.b.get_sample(self.start);
    self.start += 1;
    out.copied()
  }
}

impl DoubleEndedIterator for BlockRunner {

  fn next_back(&mut self) -> Option<Self::Item> {
    self.end -= 1;

    if self.start == self.end {
     return None
    }

    self.b.get_sample(self.end).copied()
  }
}


