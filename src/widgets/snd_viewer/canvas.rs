use std::collections::HashSet;

use crate::snd::Snd;
use crate::util::{Ruler,Region};

use iced::{
  mouse::{self,Cursor},
  keyboard,
  widget::{
    canvas,
    canvas::{
      Path,
      Cache,
      Geometry,
      Stroke,
      Fill,
      stroke,
      event::Status,
      Event
    }
  },
  Color,
  Theme,
  Rectangle,
  Point,
  Renderer
};

use super::Tport;

#[derive(Clone,Debug)]
pub enum Msg {
  Wheel(f64,f64),
  LeftClick(f32,f32,f32,f32),
  RightClick(f32,f32,f32,f32),
  LeftDrag(f32,f32,f32,f32),
  RightDrag(f32,f32,f32,f32),
  LeftDragEnd,
  RightDragEnd,
  KeyDown(f32,f32,f32,f32,keyboard::key::Physical),
  KeyUp(f32,f32,f32,f32,keyboard::key::Physical)
}

pub struct SndCanvas<'a> {
  pub snd:std::sync::Arc<Snd>,
  pub window:Region,
  pub cursor:Option<f64>,
  pub selection:Option<f64>,
  pub ruler:Option<Ruler>,
  pub mask:crate::util::Mask,
  pub transport:Tport,
  pub stamp:&'a Cache
}

#[derive(Default)]
pub struct CvState {
  ptr_x:f32,
  ptr_y:f32,
  left_down:Option<(f32,f32)>,
  right_down:Option<(f32,f32)>,
  keys_down:HashSet<iced::keyboard::key::Physical>,
}

//this normalizes coordinates between 0 and 1;
//relative to the drawing area size and offset
fn norm_points(Point{x,y}:Point,rect:Rectangle) -> (f32,f32) {
  let out_x = (x-rect.x).clamp(0.0,rect.width)/rect.width;
  let out_y = (y-rect.y).clamp(0.0,rect.height)/rect.height;
  (out_x,out_y)
}

fn window_to_screen(x:f64,window:&Region,screen_w:f32) -> f32 {
  let x_pct = (x-window.start())/window.len();
  x_pct as f32 * screen_w
}

impl<'a> canvas::Program<Msg> for SndCanvas<'a> {
  type State = CvState; 
  
  fn update(&self,
    state:&mut CvState,
    event: Event,
    r: Rectangle,
    cursor: Cursor
  ) 
    -> (Status,Option<Msg>) 
  {
    match event {
      Event::Mouse(mev) => {
        let is_over = cursor.position_over(r).is_some();

        match mev {
          mouse::Event::CursorMoved{position,..} => {
            let (nrm_x,nrm_y) = norm_points(position,r);

            let ev = match (state.left_down,state.right_down) {
              (Some((x,y)),None) if x != nrm_x && y != nrm_y => {
                Some(Msg::LeftDrag(nrm_x,nrm_y,r.width,r.height))
              },
              (_,Some((_,_))) => {
                let delta_x = nrm_x- state.ptr_x;
                let delta_y = nrm_y - state.ptr_y;
                Some(Msg::RightDrag(nrm_x,nrm_y,delta_x,delta_y))
              }
              _ => {
                None
              }
            };

            state.ptr_x = nrm_x;
            state.ptr_y = nrm_y;

            (Status::Captured,ev)
          },

          mouse::Event::WheelScrolled{delta:mouse::ScrollDelta::Lines{y,..}} if is_over => {
            (Status::Captured,Some(Msg::Wheel(state.ptr_x as f64,y as f64/100.0)))
          },

          mouse::Event::WheelScrolled{delta:mouse::ScrollDelta::Pixels{y,..}} if is_over  => {
            (Status::Captured,Some(Msg::Wheel(state.ptr_x as f64,y as f64/100.0)))
          },

          mouse::Event::ButtonPressed(mouse::Button::Left)  if is_over => {
            state.left_down = Some((state.ptr_x,state.ptr_y));
            (Status::Captured,Some(Msg::LeftClick(state.ptr_x,state.ptr_y,r.width,r.height)))
          }

          mouse::Event::ButtonPressed(mouse::Button::Right)  if is_over => {
            state.right_down = Some((state.ptr_x,state.ptr_y));
            (Status::Captured,Some(Msg::RightClick(state.ptr_x,state.ptr_y,r.width,r.height)))
          },

          mouse::Event::ButtonReleased(mouse::Button::Left) => {
            state.left_down = None;
            (Status::Captured,Some(Msg::LeftDragEnd))
          }

          mouse::Event::ButtonReleased(mouse::Button::Right) => {
            state.right_down = None;
            (Status::Captured,Some(Msg::RightDragEnd))
          },

          _ => (Status::Ignored,None)
        }
      },

      Event::Keyboard(kev) => {
        match kev {
          keyboard::Event::KeyPressed{physical_key,..} => {
            if state.keys_down.insert(physical_key) {
              let x = state.ptr_x;
              let y = state.ptr_y;

              (Status::Captured,Some(Msg::KeyDown(x,y,r.width,r.height,physical_key)))
            }
            else {
              (Status::Captured,None)
            }
          },

          keyboard::Event::KeyReleased{physical_key,..} => {
            if state.keys_down.remove(&physical_key) {
              let x = state.ptr_x;
              let y = state.ptr_y;
              (Status::Captured,Some(Msg::KeyUp(x,y,r.width,r.height,physical_key)))
            }
            else {
              (Status::Captured,None)
            }
          },

          _ => (Status::Ignored,None)
        }
      },
  
      //ignoring touch for now
      _ => (Status::Ignored,None)
    }
  }

  //This is the mouse cursor by the way, this like changes the cursor to the grabby hand
  //or something like that
  fn mouse_interaction(&self,st: &Self::State,_: Rectangle,_: Cursor) -> mouse::Interaction {
    if st.right_down.is_some() {
      mouse::Interaction::Grabbing
    }
    else {
      Default::default()
    }
  }

  fn draw(&self,_: &Self::State,renderer: &Renderer,_: &Theme,bounds: Rectangle,
    _: mouse::Cursor,
  ) -> Vec<Geometry> {
    let on_chan_strk = line(0.0,1.0,0.0,1.0);
    let off_chan_strk = line(0.6,0.6,0.6,1.0);
    let cursor_srk = line(0.0,1.0,1.0,1.0);
    let sel_fill= fill(0.0,1.0,1.0,0.4);
    let play_strk = line(1.0,1.0,0.0,1.0);

    let stamp = self.stamp.draw(renderer,bounds.size(),|frame| {
      //draw the background
      let recto = Path::rectangle((0.0f32,0.0f32).into(),frame.size());
      frame.fill(&recto,Color::from_rgba(0.0,0.0,0.0,1.0));
      let channels = self.snd.channels();

      //draw the lines
      let center_line = {
        let cy = frame.height()/2.0;
        Path::line((0.0,cy).into(),(frame.width(),cy).into())
      };

      frame.stroke(&center_line,line(1.0,1.0,1.0,1.0));

      for cn in 0..channels {

        frame.push_transform();

        let box_height = frame.height()/channels as f32;
        let box_start = box_height * cn as f32;
        let pad = 2.0;

        frame.translate([0.0,box_start + (box_height/2.0)].into());
        frame.scale_nonuniform([1.0,(2.0*pad - box_height)/2.0]);

        let center_line = Path::line((0.0,0.0).into(),(frame.width(),0.0).into());
        frame.stroke(&center_line,line(0.4,0.8,0.8,1.0));

          let strk = if self.mask.is_on(cn) {
            on_chan_strk
          }
          else {
            off_chan_strk
          };

        let start = self.window.start();
        let frame_region_width = self.window.len() as f32/frame.width();
        
        let chan = self.snd.channel(cn).unwrap();

        let mut last_x =0.0;
        let mut last_min = 0.0;
        let mut last_max = 0.0;

        for i in 0..frame.width() as usize {
          let start = start as f32 + (i as f32 * frame_region_width);
          let end = start + frame_region_width;
          
          //it feels like this should be start.ceil, but that gives bad
          //results
          let sample_start = start.floor() as usize; 
          let sample_end = end.floor() as usize;

          let (min,max) = chan.summary(sample_start,sample_end);
          let x = i as f32;

          let min_line = Path::line((last_x,last_min).into(),(x,min).into());
          frame.stroke(&min_line,strk);

          let max_line = Path::line((last_x,last_max).into(),(x,max).into());
          frame.stroke(&max_line,strk);
          
          let p = Path::line((x,min).into(),(x,max).into());
          frame.stroke(&p,strk);

          last_x = x;
          last_min=min;
          last_max=max;
        }

        frame.pop_transform();
      }
    });

    let mut frame = canvas::Frame::new(renderer,bounds.size());

    //Cursor / selection
    match (self.cursor,self.selection) {
      (Some(pt),None) if self.window.contains(pt) => {
        let rel_x = window_to_screen(pt,&self.window,bounds.width);
        let ln = Path::line((rel_x,0.0).into(),(rel_x,bounds.height).into());
        frame.stroke(&ln,cursor_srk);
      },

      (Some(pt),Some(len)) => {
        let pt_sc = window_to_screen(pt,&self.window,bounds.width);
        let len_sc = (len/self.window.len())as f32 *bounds.width;
        frame.fill_rectangle((pt_sc,0.0).into(),(len_sc,bounds.height).into(),sel_fill);
      }

      _=>()
    }

    //playback head
    match self.transport {
      Tport::Play(pos) if self.window.contains(pos) => {
        let rel_x = window_to_screen(pos,&self.window,bounds.width);
        let ln = Path::line((rel_x,0.0).into(),(rel_x,bounds.height).into());
        frame.stroke(&ln,play_strk);
      },

      _ => ()
    }

    //ruler
    if let Some(r) = self.ruler {
      const TK_H:f32 = 24.0;

      let top_y = 1.0;
      let btm_y = bounds.height-1.0;
      

      //draw the top and bottom lines
      let top_line = Path::line((0.0,top_y).into(),(bounds.width,top_y).into());
      let btm_line = Path::line((0.0,btm_y).into(),(bounds.width,btm_y).into());
      frame.stroke(&top_line,cursor_srk);
      frame.stroke(&btm_line,cursor_srk);
      
      let (s,e) = self.window.into();
      let mut line_x = r.next_or_current(s);

      while line_x < e {
        let rel_x = window_to_screen(line_x,&self.window,bounds.width) + 1.0;
        let top_ln = Path::line((rel_x,top_y).into(),(rel_x,top_y + TK_H).into());
        let btm_ln = Path::line((rel_x,btm_y - TK_H).into(),(rel_x,btm_y).into());
        frame.stroke(&top_ln,cursor_srk);
        frame.stroke(&btm_ln,cursor_srk);

        line_x = r.next_mark(line_x);
      }

    }
    
    vec![stamp,frame.into_geometry()]
  }
}

//-----------------------------------------------------------------------------
//some helpers for making stroke and fill styles
const fn line(r:f32,g:f32,b:f32,a:f32) -> Stroke<'static> {
  use iced::widget::canvas::{LineCap,LineJoin,LineDash};

  Stroke { 
    width:1.0,
    style:stroke::Style::Solid(Color::from_rgba(r,g,b,a)),
    line_cap:LineCap::Butt,
    line_join:LineJoin::Miter,
    line_dash:LineDash{segments:&[],offset:0}
  }
}

const fn fill(r:f32,g:f32,b:f32,a:f32) -> Fill {
  use iced::widget::canvas::{Style,fill};
  Fill{ 
    style:Style::Solid(Color::from_rgba(r,g,b,a)),
    rule:fill::Rule::NonZero
  }
}
