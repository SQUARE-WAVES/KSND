pub struct Pyramid {
  mips:Vec<Vec<(f32,f32)>>
}

fn min_max(a:f32,b:f32) -> (f32,f32) {
  (a.min(b),b.max(a))
}

fn min_max_pair(min1:f32,min2:f32,max1:f32,max2:f32) -> (f32,f32) {
  (min1.min(min2),max1.max(max2))
}

impl Pyramid {
  fn from_base(base:Vec<(f32,f32)>) -> Self {
    let mut mips = vec![base];

      while mips[mips.len() -1].len() >= 2 {
        let top = &mips[mips.len() - 1];
        let next = top.chunks_exact(2).map(|c|{
          let (a1,a2) = c[0];
          let (b1,b2) = c[1];
          min_max_pair(a1,b1,a2,b2)
      });

      mips.push(next.collect());
    }

    Self {
      mips
    }
  }

  pub fn floats(vals:&[f32]) -> Self {

    //this is the lowest level mip
    let base : Vec<(f32,f32)> = vals.
    chunks_exact(2).
    map(|c|min_max(c[0],c[1])).collect();

    Self::from_base(base)
  }

  pub fn peaks(&self,start:usize,end:usize,base:&[f32]) -> (f32,f32) {
    let mut start = start;
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    while start < end {
      let (asz,lod,idx) = power_chunk(start,end);

      let (cmin,cmax) = if asz == 1 {
        (base[start],base[start])
      }
      else {
        self.mips[lod-1][idx]
      };

      min = min.min(cmin);
      max = max.max(cmax);
      start += asz
    }

    (min,max)
  }
}

fn power_chunk(start:usize,end:usize) -> (usize,usize,usize) {
  let scale = (end-start).ilog2() as usize;

  let align = (1..=scale).rev().map(|n|usize::saturating_pow(2,n as u32)).find(|s|{
    start % s == 0
  }).unwrap_or(1);
      
  let idx = start/align;
  (align,align.ilog2() as usize,idx)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_creation() {
    let samples : Vec<f32> = (0..1000).map(|s|s as f32).collect();
    let pyr = Pyramid::floats(&samples[..]);
    assert_eq!(pyr.mips.len(),9,"the pyramid should have 9 levels");

    //lets get some random bits
    assert_eq!(pyr.peaks(0,20,&samples[..]),(0.0,19.0));
    assert_eq!(pyr.peaks(100,200,&samples[..]),(100.0,199.0));
    assert_eq!(pyr.peaks(512,1000,&samples[..]),(512.0,999.0));
    assert_eq!(pyr.peaks(257,600,&samples[..]),(257.0,599.0));
    assert_eq!(pyr.peaks(0,1000,&samples[..]),(0.0,999.0));
  }
}
