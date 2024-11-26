use mlua::prelude::*;
use super::super::edit_userdata::LuaSnd;
use crate::{
  edit::paste,
  edit::delete
};

pub fn get_snd(l:&Lua,_:()) -> LuaResult<LuaSnd> {
  let ed_cell = super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  let snd = ctx.copy();

  Ok(snd.into())
}

pub fn paste(l:&Lua,snd:LuaSnd) -> LuaResult<()> {
  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx();
  
  let new_ctx = paste::insert_or_replace(ctx,snd.as_ref());

  ed.push_new(new_ctx);
  ed.dirty_up();

  Ok(())
}

pub fn mix_in(l:&Lua,(src,src_gain,tgt_gain):(LuaSnd,Option<f32>,Option<f32>)) -> LuaResult<()> {
  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx();

  let src_gain = src_gain.unwrap_or(1.0);
  let tgt_gain = tgt_gain.unwrap_or(1.0); 
  
  let new_ctx = paste::mix_in(ctx,src.as_ref(),src_gain,tgt_gain);
  ed.push_new(new_ctx);

  Ok(())
}

//this is here for now
pub fn delete(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx();

  if let Some(new_guy) = delete::remove_selected(ctx) {
    ed.push_new(new_guy);
    ed.dirty_up();
  }

  Ok(())
}
