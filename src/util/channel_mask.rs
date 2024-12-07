//this is maybe not the best way to handle things, but I don't want to make
//active and inactive channels a part of the snd struct, and it feels like 
//a big pain in the ass to allocate a vector or something.
#[derive(Debug,Clone,Copy)]
pub struct Mask{
  mask:u32
}

impl Default for Mask {
  fn default() -> Self { Self {mask:0xFFFFFFFF} }
}

impl Mask{
  pub fn is_on(&self,chan:usize) -> bool {
    self.mask >> chan & 0x01  == 1
  }

  pub fn toggle(&mut self,chan:usize) {
    self.mask ^= 0x01 << chan;
  }

  //this is for when you delete a channel
  pub fn shift_after(&mut self,chan:usize) {
    let splitter = (1<<chan) - 1;
    let post = self.mask & !(splitter);
    let pre = self.mask & splitter;

    self.mask = pre | (post>>1);
  }

  //this is for when you solo a channel
  pub fn solo(&mut self) {
    self.mask = 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_shift_after() {
    let mut z = Mask { mask:0x10101010 };

    z.shift_after(24);
    assert_eq!(0x08101010,z.mask);
    z.shift_after(16);
    assert_eq!(0x04081010,z.mask);
    z.shift_after(0);
    assert_eq!(0x02040808,z.mask);
  }
}

