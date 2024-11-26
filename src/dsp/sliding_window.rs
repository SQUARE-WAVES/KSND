pub struct Window<const SZ:usize> { 
  buff:[f32;SZ],
  head:usize
}

impl<const SZ:usize> Default for Window<SZ> {
  fn default() -> Self {
    Self {
      buff:[0.0;SZ],
      head:SZ/2
    }
  }
}

impl<const SZ:usize> Window<SZ> {
  pub fn push(&mut self,val:f32) {
    self.buff[(self.head + SZ/2) % SZ] = val;
    self.head = (self.head + 1) % SZ
  }

  pub fn get(&self,idx:isize) -> f32 {
    if idx < 0 {
      let ui = -idx as usize;
      self.buff[(self.head + (SZ-ui)) % SZ]
    }
    else {
      let ui = idx as usize;
      self.buff[(self.head + ui) % SZ]
    }
  }
}

pub struct IterWindow<'a,S,const SZ:usize> {
  buff:Window<SZ>,
  overhead:usize,
  src:&'a mut S
}

impl<'a,S:Iterator<Item=f32>,const SZ:usize> IterWindow<'a,S,SZ> {
  pub fn new(src:&'a mut S) -> Self {
    let mut out = Self {
      buff:Window::<SZ>::default(),
      overhead:SZ/2,
      src
    };

    for _ in 0..(SZ/2) + 1 {
      let _ = out.pull();
    }

    out
  }

  pub fn pull(&mut self) -> bool {
    if self.overhead == 0 {
      return true
    }

    let v = self.src.next().unwrap_or_else(||{
      self.overhead -= 1;
      0.0
    });
    
    self.buff.push(v);

    self.overhead == 0
  }

  pub fn buff(&self) -> &Window<SZ> {
    &self.buff
  }
}

