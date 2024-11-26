use mlua::prelude::*;

pub fn step(l:&Lua,(amt,width):(f64,f32)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  if let Some(pt) = ctx.cursor {
    let scale = ctx.window_width()/width as f64;
    let scale = scale.max(1.0);

    let amt = amt * scale;
    let end = (pt+amt).clamp(0.0,ctx.len());
    let end = end.floor();

    ctx.cursor = Some(end);
    ctx.selection = None;
  }

  Ok(())
}

pub fn feather(l:&Lua,(amt,width):(f64,f32)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  if let Some(pt) = ctx.cursor {
    let scale = ctx.window_width()/width as f64;
    let scale = scale.max(1.0);

    let amt = amt * scale;
    let mod_len = ctx.selection.unwrap_or(0.0) + amt;
    let mod_len = mod_len.clamp(-pt,ctx.len()-pt).floor();
    if mod_len == 0.0 {
      ctx.selection = None
    }
    else {
      ctx.selection = Some(mod_len)
    }
  }

  Ok(())
}

pub fn clear_cursor(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  ctx.cursor = None;
  ctx.selection = None;
  Ok(())
}

//these are easier to do in rust cause of all the different cases
pub fn expand_selection_left(l:&Lua,_:())->LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();
  
  match (ctx.cursor,ctx.selection) {
    (Some(pt),None) => {
      let left_pt = ctx.ruler.map(|r|r.previous_mark(pt-0.5).max(0.0)).unwrap_or(0.0);
      if left_pt == pt {
        ctx.selection = None
      }
      else {
        ctx.selection = Some(left_pt-pt)
      }
    },

    (Some(pt),Some(len)) => {
      let origin = pt+len;
      let left_pt = ctx.ruler.map(|r|r.previous_mark(origin-0.5).max(0.0)).unwrap_or(0.0);

      if left_pt == pt {
        ctx.selection = None
      }
      else {
        ctx.selection = Some(left_pt-pt)
      }
    },

    _ => ()
  }

  Ok(())
}

pub fn expand_selection_right(l:&Lua,_:())->LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();
  
  match (ctx.cursor,ctx.selection) {
    (Some(pt),None) => {
      let right_pt = ctx.ruler.map(|r|r.next_mark(pt).min(ctx.len())).unwrap_or(ctx.len());

      if right_pt == pt {
        ctx.selection = None
      }
      else {
        ctx.selection = Some(right_pt-pt);
      }
    },

    (Some(pt),Some(len)) => {
      let origin = pt+len;
      let right_pt = ctx.ruler.map(|r|r.next_mark(origin).min(ctx.len())).unwrap_or(ctx.len());

      if right_pt == pt {
        ctx.selection=None;
      }
      else {
        ctx.selection = Some(right_pt-pt);
      }
    },

    _ => ()
  }

  Ok(())
}

pub fn set_cursor(l:&Lua,pos:f64) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();
  ctx.cursor = Some(pos);
  ctx.selection = None;
  Ok(())
}

pub fn select_len(l:&Lua,amt:f64) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  match (ctx.cursor,ctx.selection) {
    (Some(pt),None) => {
      let len = amt.clamp(-pt,ctx.len() - pt);
      ctx.selection = Some(len) 
    },

    (Some(pt),Some(len)) => {
      let end_pt = pt + len;
      let extra_len = amt.clamp(-end_pt,ctx.len() - end_pt);
      ctx.selection = Some(len+extra_len);
    }

    _ => () //nothing
  }

  Ok(())
}

pub fn select_region(l:&Lua,(start,end):(f64,f64)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let len = end - start;
  ctx.cursor = Some(start);
  ctx.selection = Some(len);

  Ok(())
}

pub fn look_at(l:&Lua,(start,end):(f64,f64)) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  let end = end.min(ctx.len());
  let start = start.max(0.0);

  let len = end-start;
  let scale = ctx.len();
  ctx.zoom = len/scale;
  ctx.slide = start/scale;
  ed.dirty_up();
  Ok(())
}

//basic accessors
pub fn cursor(l:&Lua,_:()) -> LuaResult<Option<f64>> {
  let ed_cell = &mut super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  Ok(ctx.cursor)
}

pub fn selection(l:&Lua,_:()) -> LuaResult<Option<f64>> {
  let ed_cell = &mut super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  Ok(ctx.selection)
}

//the funny return is cause you can't do tuple options easily
pub fn selected_region(l:&Lua,_:()) -> LuaResult<(Option<f64>,Option<f64>)> {
  let ed_cell = &mut super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();
  let region = ctx.selected_region();
  let (start,end) = (region.map(|r|r.start()),region.map(|r|r.end()));
  Ok((start,end))
}

pub fn total_len(l:&Lua,_:()) -> LuaResult<f64> {
  let ed_cell = &mut super::grab_editor(l)?;
  let ed = ed_cell.borrow();
  let ctx = ed.ctx();

  Ok(ctx.len())
}

//some handy ones
pub fn zoom_out(l:&Lua,_:()) -> LuaResult<()> {
  let ed_cell = &mut super::grab_editor(l)?;
  let mut ed = ed_cell.borrow_mut();
  let ctx = ed.ctx_mut();

  ctx.zoom = 1.0;
  ctx.slide = 0.0;
  ed.dirty_up();
  Ok(())
}

pub fn zoom_to_selected(l:&Lua,_:()) -> LuaResult<()> {
  //you have to be careful with borrowing stuff
  let sel_region = {
    let ed_cell = &super::grab_editor(l)?;
    let ed = ed_cell.borrow();
    ed.ctx().selected_region()
  };
  
  if let Some(r) = sel_region {
    look_at(l,r.into())
  }
  else {
    Ok(())
  }
}
