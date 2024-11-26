use iced::{
  Rectangle as Rect,
  Event,
  event::Status,
  Length,
  Size,
  alignment,
  Color,
  advanced::{
    text,
    text::Paragraph,
    text::paragraph::Plain,
    Text,
    Widget,
    widget::Tree,
    widget::tree::Tag,
    widget::tree::State as TState,
    renderer,
    layout::{
      Layout,
      Limits,
      Node
    },
    renderer::Style,
    mouse::{
      Click,
      click::Kind
    }
  },
  keyboard::{
    self,
    Key,
    Modifiers
  },
  mouse
};

use super::Cursor;

#[derive(Debug,Clone)]
pub enum Msg  {
  Key(Key,Modifiers),
  SetCursor(usize),
  SetSelection(usize,isize),
  SelectAll,
  RequestFocus
}

pub struct TI<'a> {
  content:&'a str,
  cursor:Cursor,
  focus:bool
}

pub fn new(content:&str,cursor:Cursor,focus:bool) -> TI {
  TI{
    content,
    cursor,
    focus
  }
}

#[derive(Default)]
pub struct WidgetState<P:Paragraph> {
  pub para:Plain<P>,
  pub last_click:Option<Click>,
  pub drag:Option<usize>
}

impl<'a,Th, Rndr> Widget<Msg, Th, Rndr> for TI<'a>
where
    Rndr: text::Renderer
{
  fn tag(&self) -> Tag {
    Tag::of::<WidgetState<Rndr::Paragraph>>()
  }

  fn state(&self) -> TState { 
    TState::new(WidgetState::<Rndr::Paragraph>::default())
  }

  // Required methods
  fn size(&self) -> Size<Length> {
    Size{width:Length::Fill,height:Length::Shrink}
  }

  fn layout(&self,tree: &mut Tree,r: &Rndr,limits: &Limits) -> Node {
    //this part just says "take up the row you are on!"
    let padding = iced::Padding::new(5.0);
    let text_size = r.default_size();
    let height = iced::widget::text::LineHeight::default().to_absolute(text_size);
    let limits = limits.width(Length::Fill).shrink(padding);
    let bounds = limits.resolve(Length::Fill,height,Size::ZERO);

    let state = tree.state.downcast_mut::<WidgetState<Rndr::Paragraph>>();
    
    //this part caches the content in the paragraph object
    //so that the display won't have to copy the string each frame
    let txt = Text {
      font:r.default_font(),
      line_height:Default::default(),
      content:self.content,
      size:text_size,
      bounds,
      horizontal_alignment: TEXT_ALIGN_X,
      vertical_alignment: TEXT_ALIGN_Y,
      shaping: text::Shaping::Advanced, //TEXT SHAPING BASIC MESSES EVERYTHING UP USE ADVANCED
      wrapping: text::Wrapping::None //THIS THING IS A ONE LINER, NO WRAPPING NO NOTHING
    };

    state.para.update(txt);

    Node::new(bounds)
  }

  fn draw(&self,t: &Tree,r: &mut Rndr,_: &Th,_: &Style,l: Layout<'_>,_: mouse::Cursor,vp: &Rect) {
    let state = t.state.downcast_ref::<WidgetState<Rndr::Paragraph>>();
    let bounds = l.bounds(); 
    let bg_quad = renderer::Quad{bounds,..Default::default()};

    if !self.focus {
      r.fill_quad(bg_quad,BACKGROUND_NON_FOCUS_COLOR);
      return;
    }
        
    let p = state.para.raw();

    let (draw_pt,curs_bnds,curs_color) = text_zones(&bounds,p,self.cursor);

    let curs_quad = renderer::Quad{bounds:curs_bnds,..Default::default()};
    r.fill_quad(bg_quad,BACKGROUND_COLOR);
    r.fill_quad(curs_quad,curs_color);
    r.fill_paragraph(state.para.raw(),draw_pt,TEXT_COLOR,*vp);
  }

  //I really hate functions where they are like this there is no good way to
  //format the end where the return value is
  fn on_event(
    &mut self,
    tree: &mut Tree,
    event: Event,
    l: Layout<'_>,
    mouse_cursor: mouse::Cursor,
    _: &Rndr,
    _clipboard: &mut dyn iced::advanced::Clipboard,
    sh: &mut iced::advanced::Shell<'_, Msg>,
    _viewport: &Rect,
  ) 
    -> Status 
  {
    let state = tree.state.downcast_mut::<WidgetState<Rndr::Paragraph>>();

    match event {
      Event::Keyboard(keyboard::Event::KeyPressed {modified_key:key,modifiers,..}) => {
        if !self.focus {
          return Status::Ignored;
        }

        sh.publish(Msg::Key(key,modifiers));
        Status::Captured
      },

      Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
        if let Some(pt) = mouse_cursor.position_over(l.bounds()) {
          let click = Click::new(pt,mouse::Button::Left,state.last_click);

          let status = match(click.kind(),self.focus) {
            (Kind::Double,false) => {
              sh.publish(Msg::RequestFocus);
              Status::Captured
            },
            (Kind::Double,true) => {
              sh.publish(Msg::SelectAll);
              Status::Captured
            },
            (Kind::Single,true) => {
              if self.content.is_empty() {
                Status::Captured
              }
              else if let Some(h) = state.para.raw().hit_test(pt) {
                sh.publish(Msg::SetCursor(h.cursor()));
                state.drag = Some(h.cursor());
                Status::Captured
              }
              else {
                Status::Ignored
              }
            },

            _ => Status::Ignored
          };

          state.last_click = Some(click);

          status
        }
        else {
          Status::Ignored
        }
      },

      Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
        state.drag = None;
        Status::Captured
      },

      Event::Mouse(mouse::Event::CursorMoved{position:pt}) if state.drag.is_some() => {
        if self.focus {
          let drag_start = state.drag.unwrap();
          let end_hit = state.para.raw().hit_test(pt);

          match end_hit {
            Some(pos) if pos.cursor() < drag_start => {
              let len = (drag_start - pos.cursor()) as isize;
              sh.publish(Msg::SetSelection(drag_start,-len));
            },
            Some(pos) if pos.cursor() == drag_start => {
              sh.publish(Msg::SetCursor(drag_start));
            },
            Some(pos) => {
              let len = (pos.cursor()-drag_start) as isize;
              sh.publish(Msg::SetSelection(drag_start,len));
            }
            None => ()
          };

          Status::Captured
        }
        else {
          Status::Ignored
        }
      }

      _ => Status::Ignored
    }
  }
}

fn text_zones<P:Paragraph>(bounds:&Rect,p:&P,cursor:Cursor) -> (iced::Point,Rect,Color) {
  let txt_sz = p.min_bounds();
  let w = txt_sz.width;
  let h = txt_sz.height;
  
  let (x,xoff) = match p.horizontal_alignment() {
    alignment::Horizontal::Left => (bounds.x,bounds.x),
    alignment::Horizontal::Center => (bounds.center_x(),bounds.center_x() - w/2.0),
    alignment::Horizontal::Right => (bounds.x + bounds.width,bounds.x +bounds.width - w)
  };

  let (y,yoff) = match p.vertical_alignment() {
    alignment::Vertical::Top => (bounds.y,bounds.y),
    alignment::Vertical::Center => (bounds.center_y(),bounds.center_y()-h/2.0),
    alignment::Vertical::Bottom => (bounds.y + bounds.height,bounds.y + bounds.height -h)
  };

  let txt_offset : iced::Vector = [xoff,yoff].into();

  //ok the way grapheme position works is it returns None if there is no text
  //if the index is after the end of the text it returns the position at the end
  //of the text
  match cursor {
    Cursor::Ins(curs) => {
      let curs_start = p.grapheme_position(0,curs);
      let curs_start = curs_start.unwrap_or(iced::Point::ORIGIN) + txt_offset;
      let curs_w =2.0;

      let curs_bounds = Rect{x:curs_start.x,y:curs_start.y,width:curs_w,height:h};

      ((x,y).into(),curs_bounds,CURSOR_COLOR)
    },

    Cursor::Sel(_,_) => {
      let (start,end) = cursor.range();
      let curs_start = p.grapheme_position(0,start);
      let curs_start = curs_start.unwrap_or(iced::Point::ORIGIN) + txt_offset;

      let curs_end = p.grapheme_position(0,end);
      let curs_end = curs_end.unwrap_or(iced::Point::ORIGIN) + txt_offset;

      let curs_w = curs_end.x - curs_start.x;

      let curs_bounds = Rect{x:curs_start.x,y:curs_start.y,width:curs_w,height:h};
      ((x,y).into(),curs_bounds,SELECTION_COLOR)
    }
  }
}

impl<'a,Theme, Renderer> From<TI<'a>> for iced::Element<'a, Msg, Theme, Renderer>
where
    Renderer: text::Renderer,
{
    fn from(ti:TI<'a>) -> Self {
      Self::new(ti)
    }
}


//This is some style stuff I'm putting it down here so it's easy to find and change and mess with
const TEXT_COLOR : Color = Color::from_rgba(0.0,1.0,0.0,1.0);
const BACKGROUND_COLOR : Color = Color::from_rgba(0.0,0.0,0.0,1.0);
const BACKGROUND_NON_FOCUS_COLOR : Color = Color::from_rgba(0.2,0.2,0.2,1.0);
const CURSOR_COLOR: Color = Color::from_rgba(1.0,1.0,1.0,1.0);
const SELECTION_COLOR: Color = Color::from_rgba(0.8,0.0,0.8,0.5);

const TEXT_ALIGN_X : alignment::Horizontal = alignment::Horizontal::Left;
const TEXT_ALIGN_Y : alignment::Vertical = alignment::Vertical::Center;
