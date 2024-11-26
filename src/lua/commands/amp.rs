use mlua::prelude::*;
use crate::edit::amp;

pub fn gain(l:&Lua,amt:f32) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = amp::gain(ctx,amt);
  ed.push_new(new_ctx);

  Ok(())
}

pub fn fade(l:&Lua,(start,end):(f32,f32)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = amp::lin_fade(ctx,start,end);
  ed.push_new(new_ctx);

  Ok(())
}

pub fn normalize(l:&Lua,level:Option<f32>) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = amp::normalize(ctx,level);
  ed.push_new(new_ctx);

  Ok(())
}
