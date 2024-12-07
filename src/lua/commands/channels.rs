use mlua::prelude::*;
use crate::edit::channels;

pub fn solo(l:&Lua,chan:usize) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = channels::solo(ctx,chan);
  ed.push_new(new_ctx);

  Ok(())
}

pub fn delete(l:&Lua,chan:usize) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = channels::delete(ctx,chan);
  ed.push_new(new_ctx);

  Ok(())

}

pub fn insert(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let new_ctx = channels::insert(ctx);
  ed.push_new(new_ctx);

  Ok(())

}

