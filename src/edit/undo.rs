use super::Ctx;

pub struct Stack {
  btm:Ctx,
  undos:Vec<Ctx>
}

impl Stack {
  pub fn new(btm:Ctx) -> Self {
    Self{ btm, undos:vec![] }
  }

  pub fn push(&mut self, new:Ctx) {
    self.undos.push(new);
  }

  pub fn pop(&mut self) {
    self.undos.pop();
  }

  pub fn top(&self) -> &Ctx {
    self.undos.last().unwrap_or(&self.btm)
  }

  pub fn top_mut(&mut self) -> &mut Ctx {
    self.undos.last_mut().unwrap_or(&mut self.btm)
  }
}
