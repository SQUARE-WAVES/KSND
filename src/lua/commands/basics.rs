use mlua::prelude::*;
use crate::lua::Action;

pub fn insert_silence(l:&Lua,len:usize) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let silence = crate::blocks::Block::silence(len);
  let new_sqs = ctx.snd.seqs().iter().map(|s| {
    s.insert(0,&silence.clone().into())
  });

  let new_snd = crate::snd::Snd::from_iter(ctx.snd.sample_rate(),new_sqs);
  let out = ctx.flip(new_snd.into());

  ed.push_new(out);
  Ok(())
}

pub fn undo(l:&Lua,_:()) -> LuaResult<Option<Action>> {
  let ed = super::grab_editor(l)?;
  ed.borrow_mut().undo();
  ed.borrow_mut().dirty_up();
  Ok(None)
}

pub fn activate_cmd_line(_l:&Lua,_:()) -> LuaResult<Action> {
  Ok(Action::ActivateCmdLine)
}

pub fn play(_l:&Lua,_:()) -> LuaResult<Action> {
  Ok(Action::Play)
}

pub fn print_nfo(l:&Lua,s:String) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  ed.print_nfo(s);
  Ok(())
}

pub fn clear_console(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  ed.clear_console();
  Ok(())
}

pub fn toggle_loop(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();
  ctx.loop_mode = !ctx.loop_mode;
  Ok(())
}

pub fn toggle_channel(l:&Lua,chan:usize) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();
  ctx.channels.toggle(chan);
  ed.dirty_up();
  Ok(())
}

pub fn toggle_over(l:&Lua,y:f32) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let chans = ctx.snd.channels() as f32;
  let over = (y*chans).floor() as usize;
  ctx.channels.toggle(over);
  ed.dirty_up();
  Ok(())
}

pub fn reverse(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx();

  let revd = crate::edit::fx::reverse(ctx);

  ed.push_new(revd);
  Ok(())
}

pub fn config(_l:&Lua,_:()) -> LuaResult<Action> {
  Ok(Action::ConfigAudio)
}
