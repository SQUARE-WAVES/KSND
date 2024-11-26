use std::collections::VecDeque;

pub struct Ring<T,const SZ:usize>(VecDeque<T>);

impl<T,const SZ:usize> Ring<T,SZ> {
  pub fn new() -> Self {
    Self(VecDeque::with_capacity(SZ))
  }

  pub fn push(&mut self,item:T) {
    self.0.push_front(item);
    self.0.truncate(SZ);
  }

  pub fn iter(&self) -> impl Iterator<Item=&T> {
    self.0.iter().rev()
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_structure_and_iteration() {
    let mut z : Ring<usize,5> = Ring::new();
    z.push(1);
    z.push(2);
    z.push(3);
    z.push(4);
    z.push(5);
    z.push(6);

    let z2 :Vec<usize> = z.iter().copied().collect();
    assert_eq!(z2[0],2);
    assert_eq!(z2[1],3);
    assert_eq!(z2[2],4);
    assert_eq!(z2[3],5);
    assert_eq!(z2[4],6);
  }
}
