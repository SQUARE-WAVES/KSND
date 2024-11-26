mod widget;
mod actions;

pub use actions::Cursor;

use iced::keyboard::Key;

#[derive(Default)]
pub struct State {
  focus:bool,
  content:String,
  cursor:Cursor
}

#[derive(Debug)]
pub enum OutMsg {
  Cancel,
  Submit,
  TakeFocus,
  None
}

pub use widget::Msg as Msg;

impl State {
  pub fn reset(&mut self) {
    self.focus = false;
    self.content.clear();
    self.cursor = Cursor::Ins(0);
  }

  pub fn content(&self) -> &str {
    &self.content
  }

  pub fn focus(&mut self) {
    self.focus = true;
  }
  
  pub fn update(&mut self,msg:Msg) -> OutMsg {
    match msg {
      Msg::Key(k,m) => {
        match k.as_ref() {
          Key::Named(n) => self.named_keys(n,m),
          Key::Character(c) => self.char_key(c),
          _ => OutMsg::None
        }
      },

      Msg::RequestFocus => {
        OutMsg::TakeFocus
      },

      Msg::SelectAll => {
        if !self.content.is_empty() {
          self.cursor = Cursor::Sel(0,self.content.len() as isize);
        }

        OutMsg::None
      },

      Msg::SetCursor(c) => {
        if self.focus {
          self.cursor = Cursor::Ins(c.min(self.content.len()));
        }

        OutMsg::None
      }

      Msg::SetSelection(pos,len) => {
        if self.focus {
          self.cursor = Cursor::Sel(pos,len);
        }

        OutMsg::None
      }
    }
  }

  pub fn view(&self) -> widget::TI {
    widget::new(self.content.as_ref(),self.cursor,self.focus)
  }
}
