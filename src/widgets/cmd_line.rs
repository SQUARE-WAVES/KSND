use iced::{
  widget::text_input,
  Element
};

#[derive(Default)]
pub struct State {
  pub active:bool,
  pub text:String
}

#[derive(Clone,Debug)]
pub enum Msg {
  TxtChange(String),
  Submit,
  Quit
}

pub enum Action {
  None,
  RunCmd(String),
}

pub fn update(st:&mut State,msg:Msg) -> Action {
  match msg {
    Msg::TxtChange(s) => {
      st.text = s;
      Action::None
    }
    
    Msg::Submit => {
      let mut s = String::new();
      std::mem::swap(&mut st.text,&mut s);
      st.active = false;

      Action::RunCmd(s)
    },

    Msg::Quit => {
      st.text.clear();
      st.active = false;

      Action::None
    }
  }
}

pub fn view(st:&State) -> Element<Msg> {
  if st.active {
    text_input("",&st.text).on_input(Msg::TxtChange).on_submit(Msg::Submit).into()
  }
  else {
    text_input("","").into()
  }
}
