use mlua::prelude::*;

pub fn seconds(l:&Lua,secs:f64) -> LuaResult<f64> {
  let ed_cell = super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let sr = ed.ctx().snd.sample_rate() as f64;

  Ok(sr * secs)
}

//this is one quarter note at the given BPM
pub fn bpm(l:&Lua,bpm:f64) -> LuaResult<Option<f64>> {
  let ed_cell = super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let sr = ed.ctx().snd.sample_rate() as f64;

  if bpm == 0.0 {
    return Ok(None)
  }

  Ok(Some((60.0/bpm)*sr))
}


