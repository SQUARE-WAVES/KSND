use std::{
  rc::Rc,
  cell::RefCell,
  sync::Arc
};

use anyhow::Result;

use iced::{
  Element,
  widget::{
    column,
    row,
    container
  }
};

use crate::{
  snd::Snd,
  edit::Editor,
  lua,
  skin::Skin,
  widgets::{
    snd_viewer::{
      State as SndView,
      Msg as SvMsg,
      Action as SndAct
    },
    text_input::{
      State as CmdLine,
      Msg as CmdMsg,
      OutMsg
    },
    console,
    info_panel
  }
};

#[derive(Clone,Debug)]
pub enum Msg {
  SndView(SvMsg),
  CmdLine(CmdMsg),
  None //this is dumb but I'll figure a better way later
}

pub enum Action {
  Play(Arc<Snd>,f64,f64,f64,bool),
  Stop,
  OpenNew(Arc<Snd>,Option<String>),
  ConfigAudio,
  None
}

pub struct Win {
  editor:Rc<RefCell<Editor>>,
  snd_view:SndView,
  cmd_line:CmdLine,
  skin:Skin
}

impl Win {
  pub fn new<S:Into<Arc<Snd>>>(snd:S,file:Option<String>) -> Result<Self> {
    let editor = Rc::new(RefCell::new(Editor::new(snd.into(),file)));

    Ok(Self {
      editor,
      snd_view:SndView::new(),
      cmd_line:Default::default(),
      skin:Default::default()
    })
  }

  pub fn title(&self) -> String {
    self.editor.borrow().path().map(|s|s.to_string()).unwrap_or("New Snd".to_string())
  }
  
  pub fn update(&mut self,lua:&mut mlua::Lua,msg:Msg) -> Action {
    match msg {
      Msg::SndView(m) => {
        let r = self.snd_view.update(self.editor.clone(),lua,m);

        if let SndAct::Chord(x,y,w,h,txt) = r {
          let out = lua::run_chord(lua,&txt,self.editor.clone(),x,y,w,h);

          match out {
            Ok(act) => {
              self.lua_action(act)
            },
          
            Err(e) => {
              let s = e.to_string();
              //tabs mess up the text formatting
              let s = s.replace("\t","    ");
              self.editor.borrow_mut().print_err(s);
              Action::None
            }
          }
        }
        else {
          Action::None
        }
      },

      Msg::CmdLine(m) => {
        let r = self.cmd_line.update(m);

        match r {
          OutMsg::Cancel => {
            self.cmd_line.reset();
            self.snd_view.focus();
            Action::None
          },

          OutMsg::Submit => {
            let txt = self.cmd_line.content(); 
            let out = lua::run_cmd(lua,txt,self.editor.clone());

            let cmd_act = match out {
              Ok(act) => self.lua_action(act),
            
              Err(e) => {
                let s = e.to_string();
                //tabs mess up the text formatting
                let s = s.replace("\t","    ");
                self.editor.borrow_mut().print_err(s);
                Action::None
              }
            };

            self.cmd_line.reset();
            self.snd_view.focus();
            cmd_act
          },

          OutMsg::TakeFocus => {
            self.cmd_line.focus();
            self.snd_view.unfocus();
            Action::None
          }

          OutMsg::None => Action::None 
        }
      },

      _ => Action::None
    }
  }

  fn lua_action(&mut self,act:Option<lua::Action>) -> Action {
    //handle the action
    match act {
      Some(lua::Action::ActivateCmdLine) => {
        self.snd_view.unfocus();
        self.cmd_line.focus();
        Action::None
      },

      Some(lua::Action::Play) => {
        if self.snd_view.playing() {
          Action::Stop
        }
        else {
          let (snd,s,e,pt,lp) = self.editor.borrow().playback_settings();
          Action::Play(snd,s,e,pt,lp)
        }
      },

      Some(lua::Action::NewWindow(s,f)) => {
        Action::OpenNew(s,f)
      },

      Some(lua::Action::ConfigAudio) => {
        Action::ConfigAudio
      }

      _ => Action::None
    }
  }

  pub fn play(&mut self,pos:usize) {
    self.snd_view.playback(pos)
  }
  
  pub fn stop(&mut self) {
    self.snd_view.stop()
  }

  pub fn view(&self) -> Element<Msg> {
    let waveform : Element<SvMsg> = self.snd_view.view(&mut self.editor.borrow_mut());
    
    let cmd_input : Element<CmdMsg> = container(self.cmd_line.view())
    .align_y(iced::alignment::Vertical::Center)
    .width(600)
    .height(30)
    .into();
    
    let console : Element<()> = container(console::view(self.editor.borrow().con_txt()))
    .width(600)
    .height(iced::Length::Fill)
    .style(|_|self.skin.console_box())
    .into();
  
    let ipv = info_panel::view(&self.editor.borrow());
    
    let info_panel : Element<()> = container(ipv)
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .padding(10)
    .style(|_|self.skin.info_box())
    .into();

    column![
      iced::widget::vertical_space().height(25),
      container(waveform.map(Msg::SndView)).style(|_|self.skin.waveform_box()),
      row![
        column![
          cmd_input.map(Msg::CmdLine),
          //this is what None is for, it's kind of annoying
          console.map(|_|Msg::None) 
        ],
        info_panel.map(|_|Msg::None)
      ].spacing(5)
    ]
    .spacing(5)
    .padding(5)
    .into()
  }
}
