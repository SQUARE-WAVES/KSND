#[derive(Debug,Clone,Copy)]
pub struct Region {
  start:f64,
  end:f64
}

//these are closed regions i.e [Start,end],
//so start is in the region but end isnt
//like regular ranges
impl Region {
  pub fn len(&self) -> f64 {
    self.end-self.start
  }

  pub fn sample_range(&self) -> (usize,usize) {
    //so if your region is like 10.2 to 13.5,
    //your sample range should be 11 to 14,
    //that way a non-inclusive top range gets all samples
    //that are covered
    let start = self.start.max(0.0).ceil() as usize;
    let end = self.end.ceil() as usize;
    (start,end)
  }

  pub fn intersect(&self,other:&Self) -> Option<Self> {
    if self.end < other.start  || other.end < self.start{
      None
    }
    else {
      let start = self.start.max(other.start);
      let end = self.end.min(other.end);

      Some(Self{start,end})
    }
  }

  pub fn start(&self) -> f64 {
    self.start
  }

  pub fn end(&self) -> f64 {
    self.end
  }

  pub fn center(&self) -> f64 {
    (self.start + self.end)/2.0
  }

  pub fn contains(&self,pt:f64) -> bool {
    pt>=self.start && pt<=self.end
  }

  pub fn clamp(&self,pt:f64) -> f64 {
    pt.clamp(self.start,self.end)
  }

  pub fn slide(&self,dist:f64) -> Self {
    (self.start + dist,self.end + dist).into()
  }
}

impl From<(f64,f64)> for Region {
  fn from((s,e):(f64,f64)) -> Self {
    let (start,end) = super::min_max(s,e);
    Self{start,end}
  }
}

impl From<Region> for (f64,f64) {
  fn from(Region{start,end}:Region) -> Self {
    (start,end)
  }
}

#[cfg(test)]
mod tests {
use super::*;
  #[test]
  fn start_end_and_len() {
    let x :Region = (256.0,16.0).into();
    assert_eq!(x.start,16.0,"the start should be the lower of the 2 numbers");
    assert_eq!(x.end,256.0,"the end should be the greater of the 2 numbers");
    assert_eq!(x.len(),240.0,"the regions len should be correct");

    let x: Region = (-5.4,10.0).into();
    assert_eq!(x.start,-5.4,"the start should be the lower of the 2 numbers");
    assert_eq!(x.end,10.0,"the end should be the greater of the 2 numbers");
    assert_eq!(x.len(),15.4,"the regions len should be correct");
  }

  #[test]
  fn sample_cover(){
    let x: Region = (-5.4,10.0).into();
    assert_eq!(x.sample_range(),(0,10),"sample range should truncate negative numbers");

    let x: Region = (3.4,10.7).into();
    assert_eq!(x.sample_range(),(4,11),"sample range should round start up and end down");
  }

  #[test]
  fn intersection(){
    let x : Region = (0.0,10.0).into();

    let left :Region  = (-5.0,-3.0).into();
    assert!(x.intersect(&left).is_none(),"a region to the left should have no intersection");

    let right :Region = (11.0,15.0).into();
    assert!(x.intersect(&right).is_none(),"a region to the right should have no intersection");

    let overlap :Region = (-3.0,7.0).into();
    if let Some(Region{start,end}) = x.intersect(&overlap) {
      assert_eq!(0.0,start,"the left overlapping region should start correctly");
      assert_eq!(7.0,end,"the left overlapping region should end correctly");
    }
    else {
      panic!("an overlapping region should have an intersection");
    }

    let overlap :Region = (7.0,11.0).into();
    if let Some(Region{start,end}) = x.intersect(&overlap) {
      assert_eq!(7.0,start,"the right overlapping region should start correctly");
      assert_eq!(10.0,end,"the right overlapping region should end correctly");
    }
    else {
      panic!("an overlapping region should have an intersection");
    }

    let overlap :Region = (10.0,11.0).into();
    if let Some(Region{start,end}) = x.intersect(&overlap) {
      assert_eq!(10.0,start,"the right overlapping region should start correctly");
      assert_eq!(10.0,end,"the right overlapping region should end correctly");
    }
    else {
      panic!("an overlapping region should have an intersection");
    }
  }

  #[test]
  fn contains(){
    let x: Region = (-5.4,10.0).into();
    assert!(x.contains(0.0),"contains should return true if the number is in the region");
    assert!(x.contains(-5.4),"a region contains it's start endpoint");
    assert!(x.contains(10.0),"a region contains it's end endpoint");
    assert!(!x.contains(-60.0),"contains should return false if the number isn't in the region");
  }

  #[test]
  fn center(){
    let x: Region = (-5.0,10.0).into();
    assert_eq!(x.center(),2.5,"the center should be correct");

    let x: Region = (-5.0,-10.0).into();
    assert_eq!(x.center(),-7.5,"negative centers should be correct");

    let x: Region = (6.0,6.0).into();
    assert_eq!(x.center(),6.0,"single point centers should be correct");
  }
}
