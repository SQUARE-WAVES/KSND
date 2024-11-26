use std::{
  rc::Rc,
  cell::RefCell
};

use iced::{
  Element,
  Fill,
  widget::{
    container,
    canvas::Cache
  }
};

mod canvas;

use crate::edit::Editor;

pub struct State {
  chorder:crate::util::Chorder,
  stamp: Cache,
  focus:bool,

  //modes
  transport:Tport,

  //configurables?
  zoom_min:f64
}

#[derive(Clone,Debug)]
pub enum Msg {
  Canvas(canvas::Msg)
}

pub enum Action{
  None,
  Chord(f32,f32,f32,f32,String)
}

#[derive(Copy,Clone,Debug,PartialEq)]
enum Tport {
  Stop,
  Play(f64)
}


impl State {
  pub fn new() -> Self {
    Self {
      chorder:Default::default(),
      stamp:Default::default(),

      focus:true,
      transport:Tport::Stop,
      
      //configurable?
      zoom_min:12.0
    }
  }

  pub fn update(&mut self,ed:Rc<RefCell<Editor>>,lua:&mlua::Lua,Msg::Canvas(cnv):Msg) -> Action {
    self.canvas_msg(ed,lua,cnv)
  }

  fn canvas_msg(&mut self,ed_cell:Rc<RefCell<Editor>>,lua:&mlua::Lua, msg:canvas::Msg) -> Action {
    use canvas::Msg as CM;

    match msg {
      CM::Wheel(center_scale,amt) => {
        let mut ed = ed_cell.borrow_mut();
        let ctx = ed.ctx_mut();
        let amt = ctx.zoom * (amt/2.0);
        let zoom_scale = ctx.snd.len() as f64;

        let zoom = (ctx.zoom + amt).clamp(self.zoom_min/zoom_scale,1.0);

        let new_start = ctx.slide + (center_scale * (ctx.zoom - zoom));
        ctx.slide = new_start.clamp(0.0,1.0-zoom);
        ctx.zoom = zoom;
        self.stamp.clear();
      },

      CM::RightDrag(_,_,delta_x,_) => {
        let mut ed = ed_cell.borrow_mut();
        let ctx = ed.ctx_mut();
        let amt = ctx.zoom * (delta_x as f64);
        ctx.slide = (ctx.slide - amt).clamp(0.0,1.0 - ctx.zoom);
        self.stamp.clear();
      },

      CM::KeyDown(x,y,w,h,k) if self.focus => {
        if let Some(s) = self.chorder.key_down(k) {
          return Action::Chord(x,y,w,h,s.to_string())
        }
      },

      CM::KeyUp(_x,_y,_w,_h,k) if self.focus => {
        self.chorder.key_up(k);
      }

      CM::LeftClick(x,y,w,h) if self.focus => {
        let txt = self.chorder.current();
        let _ = crate::lua::run_click(lua,txt,ed_cell,x,y,w,h);
      },

      CM::LeftDrag(x,y,w,h) if self.focus => {
        let txt = self.chorder.current();
        let _ = crate::lua::run_drag(lua,txt,ed_cell,x,y,w,h);
      },

      _=>()
    };

    Action::None
  }

  pub fn view(&self,ed:&mut Editor) -> Element<Msg> {
    if ed.dirty() {
      self.stamp.clear();
      ed.clean_up();
    }

    let ctx = ed.ctx();
    let snd = ctx.snd.clone();

    let cv = canvas::SndCanvas {
      snd,
      cursor:ctx.cursor,
      selection:ctx.selection,
      transport:self.transport,
      ruler:ctx.ruler,
      mask:ctx.channels,
      window:ctx.region(),
      stamp:&self.stamp
    };

    let cnv = iced::widget::canvas(cv).width(Fill).height(Fill);
    let element : Element<canvas::Msg> = container(cnv).padding(10).into();
    element.map(Msg::Canvas)
  }

  //Some actions
  pub fn focus(&mut self) {
    self.focus =true;
  }

  pub fn unfocus(&mut self) {
    self.focus = false;
    self.chorder.reset();
  }

  pub fn stop(&mut self) {
    self.transport = Tport::Stop;
  }

  pub fn playback(&mut self,pos:usize) {
    self.transport = Tport::Play(pos as f64);
  }

  pub fn playing(&self) -> bool{
    self.transport != Tport::Stop
  }

}

