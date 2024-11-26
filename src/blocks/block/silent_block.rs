use crate::util::range_bounds;

#[derive(Clone)]
pub struct SilentBlock {
  len:usize
}

impl SilentBlock {
  pub fn new(len:usize) -> Self {
    Self{len}
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn rng<R:std::ops::RangeBounds<usize>>(&self,r:R) -> Self {
    let (start,end) = range_bounds(r,self.len);
    Self::new(end-start)
  }

  pub fn get_sample(&self,index:usize) -> Option<&f32> {
    if index < self.len {
      Some(&0.0)
    }
    else {
      None
    }
  }

  pub fn summary(&self) -> (f32,f32) {
    (0.0,0.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_creation() {
    let block = SilentBlock::new(100);
    assert_eq!(block.len(),100,"silent block: length should be correct");
    assert_eq!(block.summary(),(0.0,0.0),"silent block: the summary should be silent");

    let sub1 = block.rng(10..50);
    assert_eq!(sub1.len(),40,"cutting silent block: sub block length");
    assert_eq!(sub1.summary(),(0.0,0.0),"cutting silent block: sub block summary");
    assert_eq!(sub1.get_sample(4),Some(&0.0),"cutting silent block: sub block sample");

    let sub2 = sub1.rng(10..20);
    assert_eq!(sub2.len(),10,"cutting silent block: sub block length");
    assert_eq!(sub2.summary(),(0.0,0.0),"cutting silent block: sub block summary");
    assert_eq!(sub2.get_sample(4),Some(&0.0),"cutting silent block: sub block sample");
  }
}
