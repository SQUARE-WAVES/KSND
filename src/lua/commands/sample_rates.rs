use mlua::prelude::*;
use crate::edit::sample_rates;

pub fn resample(l:&Lua,(rate,quality):(f64,Option<usize>)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let q = quality.unwrap_or(1);
  let new_ctx = sample_rates::resample(ctx,rate,q);
  ed.push_new(new_ctx);
  Ok(())
}

pub fn pitch(l:&Lua,(ratio,quality):(f64,Option<usize>)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let q = quality.unwrap_or(1);
  let new_ctx = sample_rates::pitch(ctx,ratio,q);
  ed.push_new(new_ctx);
  Ok(())
}

