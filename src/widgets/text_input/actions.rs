use iced::keyboard::{
  key::Named,
  Modifiers
};

use super::{
  OutMsg,
  State
};

#[derive(Debug,Copy,Clone)]
pub enum Cursor{
  Ins(usize),
  Sel(usize,isize)
}

impl Cursor {
  //this translates a cursor to a start and end point
  pub fn range(&self) -> (usize,usize) {
    match self {
      Self::Ins(pt) => (*pt,*pt),
      Self::Sel(pt,len) => {
        if *len < 0 {
          let st = *pt as isize + *len;
          (st as usize,*pt)
        }
        else {
          let end = *pt as isize + *len;
          (*pt,end as usize)
        }
      }
    }
  }
}

impl Default for Cursor{
  fn default() -> Self {
    Self::Ins(0)
  }
}

impl State {
  pub fn named_keys(&mut self,k:Named,mods:Modifiers) -> OutMsg {
    match self.cursor {
      Cursor::Ins(c) => {
        match k {
          Named::Space => {
            self.content.insert(c,' ');
            self.cursor = Cursor::Ins(c+1);
          },

          Named::Home if mods.shift() => { 
            if c != 0 {
              self.cursor=Cursor::Sel(c,-(c as isize));
            }
          }

          Named::Home => { 
            self.cursor = Cursor::Ins(0);
          },


          Named::ArrowLeft if mods.shift() && mods.command() => { 
            if c != 0 {
              self.cursor=Cursor::Sel(c,-(c as isize));
            }
          }

          Named::ArrowLeft if mods.command() => { 
            self.cursor = Cursor::Ins(0);
          },

          Named::ArrowLeft if mods.shift() => { 
            if c != 0 {
              self.cursor = Cursor::Sel(c,-1);
            }
          },

          Named::ArrowLeft => {
            self.cursor = Cursor::Ins(c.saturating_sub(1));
          },

          Named::End if mods.shift() => {
            if c != self.content.len() {
              let remains = self.content.len() - c;
              self.cursor = Cursor::Sel(c,remains as isize);
            }
          }

          Named::End => {
            self.cursor = Cursor::Ins(self.content.len());
          }

          Named::ArrowRight if mods.shift() && mods.command() => {
            if c != self.content.len() {
              let remains = self.content.len() - c;
              self.cursor = Cursor::Sel(c,remains as isize);
            }
          }

          Named::ArrowRight if mods.command() => {
            self.cursor = Cursor::Ins(self.content.len());
          }

          Named::ArrowRight if mods.shift() => {
            if c != self.content.len() {
              self.cursor = Cursor::Sel(c,1);
            }
          }

          Named::ArrowRight => {
            let cpos = (c+1).min(self.content.len());
            self.cursor = Cursor::Ins(cpos);
          },

          Named::Backspace => {
            if c != 0 {
              let cpos = c.saturating_sub(1);
              self.content.remove(cpos);
              self.cursor = Cursor::Ins(cpos)
            }
          },

          Named::Enter => {
            return OutMsg::Submit;
          },

          Named::Escape => {
            return OutMsg::Cancel;
          },

          _=>()
        };
      },

      Cursor::Sel(pt,len) => {
        let (start,end) = self.cursor.range();

        match k  {
          Named::Space => {
            self.content.replace_range(start..end," ");
            self.cursor = Cursor::Ins(start+1);
          },

          Named::Home if mods.shift() => {
            if pt != 0 {
              self.cursor = Cursor::Sel(pt,-(pt as isize))
            }
          },

          Named::Home => {
            self.cursor = Cursor::Ins(0);
          },

          /*
          we will need these when we do words
          Named::ArrowLeft if mods.shift() && mods.command() => { 
            self.cursor=Cursor::Ins(start);
          }

          Named::ArrowLeft if mods.command() => { 
            self.cursor = Cursor::Ins(start);
          },
          */

          Named::ArrowLeft if mods.command() && mods.shift() => {
            if pt != 0 {
              self.cursor = Cursor::Sel(pt,-(pt as isize))
            }
          },

          Named::ArrowLeft if mods.shift() => {
            if len != 1 {
              let new_len = (len-1).max(-(pt as isize));
              self.cursor = Cursor::Sel(pt,new_len);
            }
            else {
              self.cursor = Cursor::Ins(pt);
            }
          },

          Named::ArrowLeft => {
            self.cursor = Cursor::Ins(start)
          },

          Named::End if mods.shift() => {
            if pt != self.content.len() {
              self.cursor = Cursor::Sel(pt,(self.content.len()-pt) as isize)
            }
          }

          Named::End => {
            self.cursor = Cursor::Ins(self.content.len());
          }

          Named::ArrowRight if mods.shift() && mods.command() => {
            if pt != self.content.len() {
              self.cursor = Cursor::Sel(pt,(self.content.len()-pt) as isize)
            }
          }

          Named::ArrowRight if mods.command() => {
            self.cursor = Cursor::Ins(self.content.len());
          }

          Named::ArrowRight if mods.shift() => {
            if len != -1 {
              let new_len = (len + 1).min((self.content.len() - pt) as isize);
              self.cursor = Cursor::Sel(pt,new_len);
            }
            else {
              self.cursor = Cursor::Ins(pt)
            }
          }

          Named::ArrowRight => {
            self.cursor = Cursor::Ins(end);
          },

          Named::Backspace => {
            self.content.replace_range(start..end,"");
            self.cursor = Cursor::Ins(start)
          },

          Named::Enter => {
            return OutMsg::Submit;
          },

          Named::Escape => {
            return OutMsg::Cancel;
          },

          _=>()
        }
      }
    };

    OutMsg::None
  }

  pub fn char_key(&mut self,k:&str) -> OutMsg {
    match self.cursor {
      Cursor::Ins(c) => {
        k.chars().for_each(|chr|{
          if chr.is_ascii() {
            self.content.insert(c,chr);
            self.cursor = Cursor::Ins(c + 1);
          }
          else { //this is cause lua don't play nice with unicode
            let esc = chr.escape_unicode();
            self.content.extend(esc.clone());
            self.cursor = Cursor::Ins(c + esc.len());
          }
        });
      },

      Cursor::Sel(_,_) => {
        let (start,end) = self.cursor.range();
        self.content.replace_range(start..end,k);
        self.cursor = Cursor::Ins(start+k.len());
      }
    }
    
    OutMsg::None
  }
}
