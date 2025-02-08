extern crate rtaudio;

use std::sync::Arc;

use iced::{
  Element,
  Task,
  window
};

mod util;
mod blocks;
mod dsp;
mod snd;
mod edit;
mod editor_window;
mod widgets;
mod audio_sys;
mod lua;
mod win_manager;
mod sys_commands;
mod skin;

use snd::Snd;
use sys_commands::SysCommand;

fn main() {
  iced::daemon(Main::title,Main::update,Main::view)
  .antialiasing(false)
  .subscription(Main::subs)
  .style(|_,_|{
    iced::theme::Style{
      background_color: iced::Color::BLACK,
      text_color: iced::Color::from_rgb(0.0,1.0,0.0)
    }
  })
  .run_with(Main::new)
  .expect("daemon couldn't hack it");
}

use audio_sys::{
  AudioSystemInterface,
  OutMsg as AudioOut
};

struct Main {
  mgr:win_manager::State,
  lua:mlua::Lua,
  audio:AudioSystemInterface,
}

#[derive(Clone,Debug)]
pub enum Msg {
  WinMsg(window::Id,win_manager::WindowMsg),
  WinOpen(window::Id,Arc<Snd>,Option<String>),
  ConfOpen(window::Id),
  WinClosed(window::Id),
  Poll
}

impl Main {
  pub fn new() -> (Self,Task<Msg>) {
    let audio = AudioSystemInterface::new_default().expect("bad audio");
    let lua = crate::lua::setup().expect("bad lua");

    //ok we need a sound to have a sound window, it's just easier that way
    let silent_second = blocks::Block::silence(44100);
    let snd = Snd::new(44100,vec![silent_second.into()]);
    let snd : Arc<Snd> = snd.into();
    let (_id,open) = window::open(WINSET);
    (
      Self { audio,lua,mgr:Default::default()},
      open.map( move |id|Msg::WinOpen(id,snd.clone(),None))
    )
  }

  fn view(&self, window_id: window::Id) -> Element<Msg> {
    self.mgr.view(window_id).map(|(id,msg)|Msg::WinMsg(id,msg))
  }

  pub fn title(&self,id:window::Id) -> String {
    self.mgr.title(id)
  }

  fn handle_sys_cmd(&mut self,cmd:Option<SysCommand>) -> Task<Msg> {
    match cmd {
      Some(SysCommand::Play(id,snd,s,e,pt,lp)) => {
        self.audio.play(id,snd,s,e,pt,lp);
        Task::none()
      },

      Some(SysCommand::Stop(id)) => {
        self.audio.stop(id);
        Task::none()
      },

      Some(SysCommand::OpenEditor(snd,file))=>{
        let (_id,open) = window::open(WINSET);
        //it's annoying we have to copy the string
        open.map(move|id|Msg::WinOpen(id,snd.clone(),file.clone()))
      },

      Some(SysCommand::OpenAudioConfig) => {
        let (_id,open) = window::open(WINSET);
        open.map(Msg::ConfOpen)
      },

      Some(SysCommand::SetupAudio(strm_req)) => {
        self.mgr.stop_all_editors();
        self.audio.change_stream(strm_req).expect("bad stream change");
        Task::none()
      }

      _=> Task::none()
    }
  }

  pub fn update(&mut self,msg:Msg) -> Task<Msg> {
    match msg {
      Msg::WinMsg(id,x) => {
        let cmd = self.mgr.update(id,x,&mut self.lua);
        self.handle_sys_cmd(cmd)
      },

      Msg::WinOpen(id,snd,file) => {
        self.mgr.open_editor(id,snd,file).expect("couldn't open editor");
        Task::none()
      }

      Msg::ConfOpen(id) => {
        self.mgr.open_conf(id);
        Task::none()
      }

      Msg::Poll => {
        self.audio.poll(|msg| {
          match msg {
            AudioOut::Stop(id) => {
              self.mgr.stop_window(id);
            },

            AudioOut::Playback(id,pos) => {
              self.mgr.play_window(id,pos);
            }
          };

        });

        Task::none()
      }

      Msg::WinClosed(id) => {
        if self.mgr.close_window(id) {
          iced::exit()
        }
        else {
          Task::none()
        }
      }
    }
  }

  pub fn subs(&self) -> iced::Subscription<Msg> {
    //so we could dynamically slow or speed this up when we know something is playing
    let audio = iced::time::every(iced::time::Duration::from_millis(50)).map(|_|Msg::Poll);
    let window_closings = window::close_events().map(Msg::WinClosed);

    iced::Subscription::batch([audio,window_closings])
  }
}

const WINSET : iced::window::Settings = iced::window::Settings {
  size: iced::Size{width:1400.0,height:800.0},
  position: iced::window::Position::Default,
  min_size: Some(iced::Size{width:120.0,height:60.0}),
  max_size: Some(iced::Size{width:1920.0,height:1080.0}),
  fullscreen:false,
  maximized:false,
  visible: true,
  resizable: true,
  decorations: true,
  transparent: false,
  level: iced::window::Level::Normal,
  icon: None,
  platform_specific: iced::window::settings::PlatformSpecific{
    title_hidden: false,
    titlebar_transparent: false,
    fullsize_content_view: false 
  },
  exit_on_close_request: true
};
