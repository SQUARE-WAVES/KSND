use iced::keyboard::{Key,key::Named};

#[derive(Default)]
pub struct Keystate {
  keys:Vec<Key>
}

impl Keystate {
  pub fn key_down(&mut self,k:Key) -> bool {
    if !self.keys.contains(&k) {
      self.keys.push(k);
      true
    }
    else {
      false
    }
  }

  pub fn key_up(&mut self,k:Key) {
    if !self.keys.contains(&k) {
      self.keys.retain(|dk|*dk != k);
    }
  }
}


/*
fn translate(k:&Key) -> &str{
  match k {
    Key::Character(c) => c.as_str(),
    Key::Named(
    _=> ""
  }
}
*/
