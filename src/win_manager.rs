use std::{
  sync::Arc,
  collections::HashMap
};

use anyhow::Result;

use iced::{
  Element,
  window
};

use crate::{
  snd::Snd,
  
  editor_window::{
    Win as EdWin,
    Msg as EdMsg,
  },
  
  audio_sys::{
    ConfWin,
    ConfMsg,
  },

  sys_commands::SysCommand
};

pub enum ProgramWindow {
  Editor(EdWin),
  AudioConf(ConfWin)
}

impl ProgramWindow {
  pub fn title(&self) -> String {
    match self {
      Self::Editor(e) => e.title(),
      Self::AudioConf(_) => "audio configuration".to_string()
    }
  }

  pub fn view(&self) -> Element<WindowMsg> {
    match self {
      Self::Editor(e) => e.view().map(WindowMsg::Editor),
      Self::AudioConf(c) => c.view().map(WindowMsg::AudioConf)
    }
  }
}

#[derive(Clone,Debug)]
pub enum WindowMsg {
  Editor(EdMsg),
  AudioConf(ConfMsg)
}

#[derive(Default)]
pub struct State {
  wins:HashMap<window::Id,ProgramWindow>
}

impl State {
  pub fn title(&self,id:window::Id) -> String {
    if let Some(win) = self.wins.get(&id) {
      win.title()
    }
    else {
      "unknown window".to_string()
    }
  }

  pub fn view(&self,id: window::Id) -> Element<(window::Id,WindowMsg)> {
    if let Some(win) = self.wins.get(&id) {
      win.view().map(move|m|(id,m))
    }
    else {
      iced::widget::horizontal_space().into()
    }
  }

  pub fn open_editor(&mut self,id:window::Id,snd:Arc<Snd>,title:Option<String>) -> Result<()> {
    let win = EdWin::new(snd,title)?;
    self.wins.insert(id,ProgramWindow::Editor(win));
    Ok(())
  }

  pub fn open_conf(&mut self,id:window::Id) {
    let win = ConfWin::default();
    self.wins.insert(id,ProgramWindow::AudioConf(win));
  }

  pub fn close_window(&mut self,id:window::Id) -> bool {
    self.wins.remove(&id);
    self.wins.is_empty()
  }

  pub fn update(&mut self,id: window::Id, msg:WindowMsg,lua:&mut mlua::Lua) -> Option<SysCommand> {
    let win = self.wins.get_mut(&id);

    //this feels dumb, like we shouldn't have to match both
    match (win,msg) {
      (Some(ProgramWindow::Editor(e)),WindowMsg::Editor(msg)) => { 
        let action = e.update(lua,msg);
        SysCommand::from_editor_window(id,action)
      },

      (Some(ProgramWindow::AudioConf(e)),WindowMsg::AudioConf(msg)) => { 
        let action = e.update(msg);
        SysCommand::from_conf_window(action)
      },

      (None,_) => panic!("message for lost window"),
      (Some(_),_) => panic!("message type and window type don't match")
    }
  }

  pub fn stop_window(&mut self,id: window::Id) {
    let win = self.wins.get_mut(&id);
      
    if let Some(ProgramWindow::Editor(e)) = win {
      e.stop()
    }
  }

  pub fn play_window(&mut self,id: window::Id,pos:usize) {
    let win = self.wins.get_mut(&id);
      
    if let Some(ProgramWindow::Editor(e)) = win {
      e.play(pos)
    }
  }

  pub fn stop_all_editors(&mut self) {
    self.wins.iter_mut().for_each(|(_id,win)|{
      if let ProgramWindow::Editor(e) = win {
        e.stop();
      }
    });
  }
}
