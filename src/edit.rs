use std::sync::Arc;
use crate::{
  snd::Snd,
  widgets::console::Ptype,
  util::Ring
};

mod util;
pub mod undo;
pub mod ctx;
pub use ctx::Ctx;

//these mods are the core "editing"
//features
pub mod paste;
pub mod delete;
pub mod amp;
pub mod sample_rates;
pub mod fx;

pub struct Editor {
  stack:undo::Stack,
  dirty:bool,
  console:Ring<Ptype,20>,
  path:Option<String>
}

impl Editor {
  pub fn new(s:Arc<Snd>,file:Option<String>) -> Self {
    Self {
      stack:undo::Stack::new(s.into()),
      dirty:true,
      console:Ring::new(),
      path:file
    }
  }

  pub fn ctx(&self) -> &Ctx {
    self.stack.top()
  }

  pub fn ctx_mut(&mut self) -> &mut Ctx {
    self.stack.top_mut()
  }

  pub fn path(&self) -> Option<&str> {
    self.path.as_deref()
  }

  pub fn set_path<P:Into<Option<String>>>(&mut self,newp:P) {
    self.path = newp.into()
  }
  
  pub fn playback_settings(&self) -> (Arc<Snd>,f64,f64,f64,bool) {
    let ctx = self.stack.top();
    let snd = ctx.snd.clone();
    let snd_len = snd.len() as f64;

    match (ctx.cursor,ctx.selection) {
      (Some(pt),None) => (snd,0.0,snd_len,pt,ctx.loop_mode),
      (Some(pt),Some(len)) => {
        let pt2 = pt + len;
        let (s,e) = (pt2.min(pt),pt.max(pt2));
        (snd,s,e,s,ctx.loop_mode)
      },
      _ => (snd,0.0,snd_len,0.0,ctx.loop_mode)
    }
  }

  pub fn print_nfo(&mut self,msg:String) {
    self.console.push(Ptype::Nfo(msg))
  }

  pub fn print_err(&mut self,msg:String) {
    self.console.push(Ptype::Err(msg))
  }

  pub fn clear_console(&mut self) {
    self.console.clear()
  }

  pub fn con_txt(&self) -> Vec<Ptype> {
    self.console.iter().cloned().collect()
  }

  pub fn undo(&mut self) {
    self.stack.pop();
  }

  pub fn push_new(&mut self,ctx:Ctx) {
    self.stack.push(ctx);
    self.dirty = true;
  }

  pub fn dirty(&self) -> bool {
    self.dirty
  }

  pub fn clean_up(&mut self) {
    self.dirty = false;
  }

  pub fn dirty_up(&mut self) {
    self.dirty = true;
  }

  pub fn reset_stack(&mut self,ctx:Ctx) {
    self.stack = undo::Stack::new(ctx);
    self.dirty = true;
  }
}
