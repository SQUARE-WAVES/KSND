use std::sync::Arc;
use mlua::prelude::*;
use crate::{
  edit,
  snd::Snd
};

impl LuaUserData for edit::Ctx{}

impl FromLua<'_> for edit::Ctx{
  fn from_lua(v:LuaValue,_l:&Lua) -> LuaResult<edit::Ctx> {
    match v {
      LuaValue::UserData(v) => v.take::<edit::Ctx>(),
      _ => Err("invalid return type").into_lua_err()
    }
  }
}

//this is probably wasteful, but I want these to be opaque
//it's probably better to just have it be a value of some sort
impl LuaUserData for super::Action{}

impl FromLua<'_> for super::Action {
  fn from_lua(v:LuaValue,_l:&Lua) -> LuaResult<super::Action> {
    match v {
      LuaValue::UserData(v) => v.take::<super::Action>(),
      _ => Err("invalid return type").into_lua_err()
    }
  }
}

#[derive(Clone)]
pub struct LuaSnd {
  snd:Arc<Snd>
}

impl From<Arc<Snd>> for LuaSnd {
  fn from(snd:Arc<Snd>) -> Self {
    Self{snd}
  }
}

impl  From<LuaSnd> for Arc<Snd> {
  fn from(ls:LuaSnd) -> Arc<Snd> {
    ls.snd
  }
}

impl AsRef<Snd> for LuaSnd {
  fn as_ref(&self) -> &Snd {
    &self.snd
  }
}

impl LuaUserData for LuaSnd {
  fn add_fields<'a,F: LuaUserDataFields<'a,Self>>(fields: &mut F) {
    fields.add_field_method_get("len", |_, this| Ok(this.snd.len()));
    fields.add_field_method_get("count", |_, this| Ok(Arc::strong_count(&this.snd)));
  }
}

impl FromLua<'_> for LuaSnd {
  fn from_lua(v:LuaValue,_l:&Lua) -> LuaResult<LuaSnd> {
    match v {
      LuaValue::UserData(v) => {
        let clone = v.borrow::<LuaSnd>()?.clone();
        Ok(clone)
      },
      _ => Err("invalid return type").into_lua_err()
    }
  }

}
