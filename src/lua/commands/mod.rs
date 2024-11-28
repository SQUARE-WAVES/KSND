use std::{
  rc::Rc,
  cell::RefCell
};

use mlua::prelude::*;
use crate::edit::Editor;

mod basics;
mod ruler;
mod nav;
mod copypaste;
mod amp;
mod sample_rates;
mod fs;
mod time;

//Ok this function is gonna get real big, but I think it's nice to have it as
//a reference for all the function names rather than splitting them out into
//the different mods and having them each add themselves
//if I decide to split them out into different tables or something
//it could be worth re-designing
pub fn add_command_fns(l:&Lua,globals:&LuaTable) -> LuaResult<()> {
  //fs
  globals.set("load",l.create_function(fs::load)?)?;
  globals.set("load_new",l.create_function(fs::load_new)?)?;
  globals.set("save",l.create_function(fs::save)?)?;
  globals.set("save_as",l.create_function(fs::save_as)?)?;

  //basics
  globals.set("insert_silence",l.create_function(basics::insert_silence)?)?;
  globals.set("undo",l.create_function(basics::undo)?)?;
  globals.set("toggle_loop",l.create_function(basics::toggle_loop)?)?;
  globals.set("play",l.create_function(basics::play)?)?;
  globals.set("toggle_channel",l.create_function(basics::toggle_channel)?)?;
  globals.set("toggle_over",l.create_function(basics::toggle_over)?)?;
  globals.set("activate_cmd_line",l.create_function(basics::activate_cmd_line)?)?;
  globals.set("print_nfo",l.create_function(basics::print_nfo)?)?;
  globals.set("clear",l.create_function(basics::clear_console)?)?;
  globals.set("configure_audio",l.create_function(basics::config)?)?;

  //ruler stuff
  globals.set("set_ruler",l.create_function(ruler::set)?)?;
  globals.set("previous_mark",l.create_function(ruler::previous_mark)?)?;
  globals.set("next_mark",l.create_function(ruler::next_mark)?)?;
  globals.set("nearest_mark",l.create_function(ruler::nearest_mark)?)?;
  globals.set("slide_ruler",l.create_function(ruler::slide)?)?;
  globals.set("scale_ruler",l.create_function(ruler::rescale)?)?;
  globals.set("rule_time",l.create_function(ruler::time)?)?;

  //nav
  globals.set("step_cursor",l.create_function(nav::step)?)?;
  globals.set("feather_selection",l.create_function(nav::feather)?)?;
  globals.set("clear_cursor",l.create_function(nav::clear_cursor)?)?;
  globals.set("set_cursor",l.create_function(nav::set_cursor)?)?;
  globals.set("select_len",l.create_function(nav::select_len)?)?;
  globals.set("select_region",l.create_function(nav::select_region)?)?;
  globals.set("look_at",l.create_function(nav::look_at)?)?;
  globals.set("zoom_out",l.create_function(nav::zoom_out)?)?;
  globals.set("zoom_selected",l.create_function(nav::zoom_to_selected)?)?;
  globals.set("expand_left",l.create_function(nav::expand_selection_left)?)?;
  globals.set("expand_right",l.create_function(nav::expand_selection_right)?)?;

  //getters!
  globals.set("cursor",l.create_function(nav::cursor)?)?;
  globals.set("selection",l.create_function(nav::selection)?)?;
  globals.set("selected_region",l.create_function(nav::selected_region)?)?;
  globals.set("snd_len",l.create_function(nav::total_len)?)?;

  //copy and paste
  globals.set("copy_snd",l.create_function(copypaste::get_snd)?)?;
  globals.set("paste",l.create_function(copypaste::paste)?)?;
  globals.set("mix",l.create_function(copypaste::mix_in)?)?;

  //delete
  globals.set("delete",l.create_function(copypaste::delete)?)?;
  globals.set("crop",l.create_function(copypaste::crop)?)?;


  //amp
  globals.set("gain",l.create_function(amp::gain)?)?;
  globals.set("fade",l.create_function(amp::fade)?)?;
  globals.set("normalize",l.create_function(amp::normalize)?)?;

  //pitch and resample
  globals.set("resample",l.create_function(sample_rates::resample)?)?;
  globals.set("pitch_shift",l.create_function(sample_rates::pitch)?)?;

  //time helpers
  globals.set("seconds",l.create_function(time::seconds)?)?;
  globals.set("bpm",l.create_function(time::bpm)?)?;

  //fx
  globals.set("reverse",l.create_function(basics::reverse)?)?;

  Ok(())
}

fn grab_editor(l:&Lua) -> LuaResult<Rc<RefCell<Editor>>> {
  let ed = l.app_data_mut::<Rc<RefCell<Editor>>>().ok_or("invalid lua context").into_lua_err()?;
  Ok(ed.clone())
}
