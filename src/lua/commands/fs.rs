use mlua::prelude::*;
use crate::lua::Action;

pub fn load(l:&Lua,p:Option<String>) -> LuaResult<()> {
  let p : std::path::PathBuf = match p {
    Some(words) => words.into(),
    None => {
      if let Some(pb) = rfd::FileDialog::new()
      .add_filter("sound",&["wav"])
      .set_directory(".")
      .pick_file()
      {
        pb
      }
      else {
        return Ok(())
      }
    }
  };

  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();

  let path_str = p.to_str().ok_or("couldn't convert file path to string").into_lua_err()?;
  let path_strn = path_str.to_string();
  let snd = crate::snd::load_wav(p).into_lua_err()?;
  let snd : std::sync::Arc<crate::snd::Snd> = snd.into();
  ed.set_path(Some(path_strn));
  ed.reset_stack(snd.into());

  Ok(())
}

pub fn load_new(_l:&Lua,p:Option<String>) -> LuaResult<Option<Action>> {
  let p : std::path::PathBuf = match p {
    Some(words) => words.into(),
    None => {
      if let Some(pb) = rfd::FileDialog::new()
      .add_filter("sound",&["wav"])
      .set_directory(".")
      .pick_file()
      {
        pb
      }
      else {
        return Ok(None)
      }
    }
  };

  let path_str = p.to_str().expect("wow").to_string();
  let snd = crate::snd::load_wav(p).into_lua_err()?;
  Ok(Some( Action::NewWindow(snd.into(),Some(path_str))) )
}

pub fn save(l:&Lua,p:Option<String>) -> LuaResult<()> {
  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
 
  //ok they gave us a path
  if let Some(ref words) = p {
    let ctx = ed.ctx();
    crate::snd::save_wav(&ctx.snd,words).into_lua_err()?;
    ed.set_path(p);
    return Ok(())
  }

  if let Some(words) = ed.path() {
    let ctx = ed.ctx();
    crate::snd::save_wav(&ctx.snd,words).into_lua_err()?;
    return Ok(())
  }

  let dialog_path =  rfd::FileDialog::new()
  .add_filter("sound",&["wav"])
  .set_directory(".")
  .save_file();

  if let Some(pb) = dialog_path{
    let ctx = ed.ctx();
    let path_str = pb.to_str().expect("wow").to_string();
    crate::snd::save_wav(&ctx.snd,pb).into_lua_err()?;
    ed.set_path(path_str);
    return Ok(())
  }

  Ok(())
}

pub fn save_as(l:&Lua,p:Option<String>) -> LuaResult<()> {
  let ed_cell = super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
 
  //ok they gave us a path
  if let Some(ref words) = p {
    let ctx = ed.ctx();
    crate::snd::save_wav(&ctx.snd,words).into_lua_err()?;
    ed.set_path(p);
    return Ok(())
  }

  let dialog_path =  rfd::FileDialog::new()
  .add_filter("sound",&["wav"])
  .set_directory(".")
  .save_file();

  if let Some(pb) = dialog_path{
    let ctx = ed.ctx();
    let path_str = pb.to_str().expect("wow").to_string();
    crate::snd::save_wav(&ctx.snd,pb).into_lua_err()?;
    ed.set_path(path_str);
    return Ok(())
  }

  Ok(())
}

