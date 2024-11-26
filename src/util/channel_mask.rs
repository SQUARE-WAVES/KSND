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
}

