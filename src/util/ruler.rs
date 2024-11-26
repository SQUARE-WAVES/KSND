#[derive(Copy,Clone)]
pub struct Ruler {
  scale:f64,
  offset:f64
}

impl Ruler {
  pub fn new(scale:f64,offset:f64) -> Self {
    let scale = scale.max(1.0);

    Self {
      scale:scale.floor(),
      offset:offset.floor()
    }
  }

  pub fn slide(&self, offset:f64) -> Self {
    Self {
      scale:self.scale,
      offset:offset.floor() % self.scale
    }
  }

  pub fn rescale(&self, div:f64) -> Self {
    if div <= 0.0 {
      return *self
    }

    Self {
      scale:self.scale * div,
      offset:self.offset 
    }
  }

  pub fn previous_mark(&self,pos:f64) -> f64 {
    let ruler_dist = (pos - self.offset)/self.scale;
    let ruler_divs = ruler_dist.floor(); //floor always moves away from 0
    (ruler_divs * self.scale) + self.offset
  }

  pub fn next_mark(&self,pos:f64) -> f64 {
    let pmark = self.previous_mark(pos);
    pmark + self.scale 
  }

  pub fn next_or_current(&self,pos:f64) -> f64 {
    let pmar = self.previous_mark(pos);
    if pos == pmar {
      pos
    }
    else {
      pmar + self.scale
    }
  }

  pub fn time_pt(&self,pt:f64) -> f64 {
    (pt*self.scale) + self.offset
  }

  pub fn scale(&self) -> f64 {
    self.scale
  }
}

impl From<(f64,f64)> for Ruler {
  fn from((s,o):(f64,f64)) -> Self {
    Self::new(s,o)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_previous_mark() {
    let r = Ruler::new(1000.0,0.0);
    assert_eq!(r.previous_mark(5.0),0.0,"previous mark should be lower when not on a division");
    assert_eq!(r.previous_mark(1999.0),1000.0,"previous mark should work on higher divisions");
    assert_eq!(r.previous_mark(-10.0),-1000.0,"the previous mark should work in the negative");

    assert_eq!(r.previous_mark(1000.0),1000.0,"previous mark should be = when on a division");

    let r2 = r.slide(250.0);
    assert_eq!(r2.previous_mark(295.0),250.0,"previous mark should work with an offset");
    assert_eq!(r2.previous_mark(1999.0),1250.0,"higher divisions with an offset");
    assert_eq!(r2.previous_mark(-10.0),-750.0,"negatives with an offset");
    assert_eq!(r2.previous_mark(2250.0),2250.0,"previous mark should be = when on a division");
  }

  #[test]
  pub fn test_next_mark() {
    let r = Ruler::new(1000.0,0.0);
    assert_eq!(r.next_mark(5.0),1000.0,"next mark should be higher when not on a division");
    assert_eq!(r.next_mark(1999.0),2000.0,"next mark should work on higher divisions");
    assert_eq!(r.next_mark(-10.0),0.0,"the next mark should work in the negative");

    assert_eq!(r.next_mark(1000.0),2000.0,"next mark should not be = when on a division");

    let r2 = r.slide(250.0);
    assert_eq!(r2.next_mark(295.0),1250.0,"next mark should work with an offset");
    assert_eq!(r2.next_mark(1999.0),2250.0,"higher divisions with an offset");
    assert_eq!(r2.next_mark(-10.0),250.0,"negatives with an offset");
    assert_eq!(r2.next_mark(2250.0),3250.0,"next mark should not be = when on a division");
  }

  #[test]
  pub fn test_next_or_current() {
    let r = Ruler::new(1000.0,0.0);
    assert_eq!(r.next_or_current(5.0),1000.0,"next_or_current should work when not on a div");
    assert_eq!(r.next_or_current(1000.0),1000.0,"next_or_current should work when on a div");
  }
}
