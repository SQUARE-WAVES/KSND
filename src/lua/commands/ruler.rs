use mlua::prelude::*;
use crate::util::Ruler;

use super::grab_editor;

pub fn set(l:&Lua,args:(Option<f64>,Option<f64>)) -> LuaResult<()> {
  let ed_cell = &mut grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let (offset,scale) = args;
  //if they passed a scale, use that, if they didn't use the existing ruler
  //if there isn't one of those use the whole sound len
  let scale = scale.unwrap_or(ctx.ruler.map(|r|r.scale()).unwrap_or(ctx.len()));
  let offset = offset.unwrap_or(0.0);
  
  let rule : Ruler = (scale,offset).into();
  ctx.ruler = Some(rule);
  Ok(())
}

pub fn previous_mark(l:&Lua,pos:f64) -> LuaResult<f64> {
  let ed_cell = &mut grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  if let Some(r) = ctx.ruler {
    Ok(r.previous_mark((pos-0.5).max(0.0)))
  }
  else {
    Ok(0.0)
  }
}

pub fn next_mark(l:&Lua,pos:f64) -> LuaResult<f64> {
  let ed_cell = &mut grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  if let Some(r) = ctx.ruler {
    Ok(r.next_mark(pos).min(ctx.len()))
  }
  else {
    Ok(ctx.len())
  }
}

pub fn nearest_mark(l:&Lua,pos:f64) -> LuaResult<f64> {
  let ed_cell = &mut grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  let (pre_mark,nxt_mark) = if let Some(r) = ctx.ruler {
    (r.previous_mark((pos-0.5).max(0.0)),r.next_mark(pos).min(ctx.len()))
  }
  else {
    (0.0,ctx.len())
  };

  let (pre_dist,nxt_dist) = (pos-pre_mark,(pos - nxt_mark).abs());

  if pre_dist <= nxt_dist {
    Ok(pre_mark)
  }
  else {
    Ok(nxt_mark)
  }
}

pub fn slide(l:&Lua,amt:f64) -> LuaResult<()> {
  let ed_cell = &mut grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  if let Some(r) = ctx.ruler {
    ctx.ruler = Some(r.slide(amt));
  };

  Ok(())
}

pub fn rescale(l:&Lua,amt:f64) -> LuaResult<()> {
  let ed_cell = &mut grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  if amt == 0.0 {
    ctx.ruler = None
  }
  else {
    ctx.ruler= ctx.ruler.map(|r|r.rescale(amt));
  }

  Ok(())
}

//this gets the position as a function of the current
//ruler, 
pub fn time(l:&Lua,raw:f64) -> LuaResult<f64> {
  let ed_cell = &mut grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();
  let len = ctx.snd.len() as f64;

  if let Some(r) = ctx.ruler {
    Ok(r.time_pt(raw).clamp(0.0,len))
  }
  else {
    let pos = (raw*len).clamp(0.0,len);
    Ok(pos)
  }
}
