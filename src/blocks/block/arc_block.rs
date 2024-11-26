use std::sync::Arc;
use crate::blocks::mips::Pyramid;
use crate::util::range_bounds;

#[derive(Clone)]
pub struct ArcBlock {
  data:Arc<[f32]>,
  mips:Arc<Pyramid>,
  start:usize,
  end:usize
}

impl ArcBlock {
  pub fn len(&self) -> usize { self.end - self.start }

  fn wrap(data:Arc<[f32]>,mips:Arc<Pyramid>) -> Self {
    Self {
      data:data.clone(),
      mips:mips.clone(),
      start:0,
      end:data.len()
    }
  }

  pub fn from_samples(data:Arc<[f32]>) -> Self {
    let new_mips : Pyramid = Pyramid::floats(&data[..]);

    Self::wrap(data,Arc::new(new_mips))
  }

  pub fn rng<R:std::ops::RangeBounds<usize>>(&self,r:R) -> Self {
    let (start,end) = range_bounds(r,self.end);

    let s = self.start + start;
    let e = self.start + end;

    Self {
      data:self.data.clone(),
      mips:self.mips.clone(),
      start:s.clamp(self.start,self.end),
      end:e.clamp(self.start,self.end)
    }
  }
  
  pub fn get_sample(&self,index:usize) -> Option<&f32> {
    let sample_index = self.start + index;

    if sample_index >= self.end {
      None
    }
    else {
      //we shouldn't have to check bounds
      Some(&self.data[sample_index])
    }
  }

  pub fn summary(&self) -> (f32,f32) {
    self.mips.peaks(self.start,self.end,&self.data[..])
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_creation() {
    let samples = (0..1000).map(|s|s as f32).collect();
    let block = ArcBlock::from_samples(samples);

    assert_eq!(block.len(),1000,"creating an arc block: the length should be correct");
    assert_eq!(block.summary(),(0.0,999.0),"creating arc block: the summary should be correct");

    let sub1 = block.rng(10..50);
    assert_eq!(sub1.len(),40,"cutting arc block: sub block length");
    assert_eq!(sub1.summary(),(10.0,49.0),"cutting arc block: sub block summary");
    assert_eq!(sub1.get_sample(4),Some(&14.0),"cutting arc block: sub block sample");

    let sub2 = sub1.rng(10..20);
    assert_eq!(sub2.len(),10,"cutting arc block: sub block length");
    assert_eq!(sub2.summary(),(20.0,29.0),"cutting arc block: sub block summary");
    assert_eq!(sub2.get_sample(4),Some(&24.0),"cutting arc block: sub block sample");
    assert_eq!(sub2.get_sample(11),None,"cutting arc block: sub block sample out of range");
  }
}
