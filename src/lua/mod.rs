use std::{
  rc::Rc,
  cell::RefCell,
  sync::Arc
};

use mlua::prelude::*;

use crate::{
  edit::Editor,
  snd::Snd
};

mod edit_userdata;
mod commands;

pub fn setup() -> LuaResult<mlua::Lua> {
  let lua = Lua::new();
  
  {
    let chords = lua.create_table()?;
    let click_modes = lua.create_table()?;
    let drag_modes = lua.create_table()?;
    let g = lua.globals();
    g.set("chords",chords)?;
    g.set("click_modes",click_modes)?;
    g.set("drag_modes",drag_modes)?;

    commands::add_command_fns(&lua,&g)?;

    lua.load(std::path::Path::new("./setup.lua")).exec()?;
  }

  Ok(lua)
}

pub enum Action {
  Play,
  ActivateCmdLine,
  NewWindow(Arc<Snd>,Option<String>),
  ConfigAudio
}

type Ret = LuaResult<Option<Action>>;

pub fn run_cmd(lua:&mut Lua,txt:&str,ed:Rc<RefCell<Editor>>) -> Ret {
  lua.set_app_data(ed);
  let chunk = lua.load(txt);
  let act : Option<Action> = chunk.eval()?;
  let _ = lua.remove_app_data::<Rc<RefCell<Editor>>>().unwrap();
  Ok(act)
}

pub fn run_chord(lua:&mut Lua,txt:&str,ed:Rc<RefCell<Editor>>,x:f32,y:f32,w:f32,h:f32) -> Ret {
  let globals = lua.globals();
  
  let chords : LuaTable = globals.get("chords")?;
  let chord_func :Option<LuaFunction> = chords.get(txt)?;

  if let Some(f) = chord_func {
    lua.set_app_data(ed);
    let act: Option<Action> = f.call((x,y,w,h))?;
    let _ = lua.remove_app_data::<Rc<RefCell<Editor>>>().unwrap();
    Ok(act)
  }
  else {
    Ok(None)
  }
}

pub fn run_click(lua:&Lua,txt:&str,ed:Rc<RefCell<Editor>>,x:f32,y:f32,w:f32,h:f32) -> Ret {
  let globals = lua.globals();
  
  let modes : LuaTable = globals.get("click_modes")?;
  let chord_func :Option<LuaFunction> = modes.get(txt)?;

  if let Some(f) = chord_func {
    lua.set_app_data(ed);
    let act: Option<Action> = f.call((x,y,w,h))?;
    let _ = lua.remove_app_data::<Rc<RefCell<Editor>>>().unwrap();
    Ok(act)
  }
  else {
    ed.borrow_mut().ctx_mut().default_click(x as f64);
    Ok(None)
  }
}

pub fn run_drag(lua:&Lua,txt:&str,ed:Rc<RefCell<Editor>>,x:f32,y:f32,w:f32,h:f32) -> Ret {
  let globals = lua.globals();
  
  let modes : LuaTable = globals.get("drag_modes")?;
  let chord_func :Option<LuaFunction> = modes.get(txt)?;

  if let Some(f) = chord_func {
    lua.set_app_data(ed);
    let act: Option<Action> = f.call((x,y,w,h))?;
    let _ = lua.remove_app_data::<Rc<RefCell<Editor>>>().unwrap();
    Ok(act)
  }
  else {
    ed.borrow_mut().ctx_mut().default_drag(x as f64);
    Ok(None)
  }
}

