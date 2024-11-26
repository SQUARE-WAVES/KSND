pub mod region;
pub use region::Region;

pub mod ring_vec;
pub use ring_vec::Ring;

pub mod channel_mask;
pub use channel_mask::Mask;

pub mod chorder;
pub use chorder::Chorder;

pub mod ruler;
pub use ruler::Ruler;

pub mod formatters;

pub fn range_bounds<R:std::ops::RangeBounds<usize>>(rng:R,limit:usize) -> (usize,usize) {
  use std::ops::Bound;

  let start = match rng.start_bound() {
    Bound::Unbounded => 0,
    Bound::Included(s) => *s,
    Bound::Excluded(s) => *s+1
  };

  let end = match rng.end_bound() {
    Bound::Unbounded => limit,
    Bound::Included(s) => *s+1,
    Bound::Excluded(s) => *s
  };

  (start,end)
}

pub fn min_max(x:f64,y:f64) -> (f64,f64) {
  (x.min(y),x.max(y))
}

pub fn lerp(left:f32,right:f32,frac:f32) -> f32 {
  let diff = right-left;
  diff.mul_add(frac,left) //now with fused multiply adds!
}
