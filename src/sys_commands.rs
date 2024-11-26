use std::sync::Arc;
use iced::window::Id;

use crate::{
  snd::Snd,
  audio_sys::{ConfAction,StreamReq},
  editor_window::Action as EdAction,

};

pub enum SysCommand {
  Play(Id,Arc<Snd>,f64,f64,f64,bool),
  Stop(Id),
  SetupAudio(StreamReq),
  OpenEditor(Arc<Snd>,Option<String>),
  OpenAudioConfig,
}

impl SysCommand {
  pub fn from_editor_window(id:Id,act:EdAction) -> Option<Self> {
    match act {
      EdAction::None => None,
      EdAction::Play(snd,s,e,pt,lp) => Some(Self::Play(id,snd,s,e,pt,lp)),
      EdAction::Stop => Some(Self::Stop(id)),
      EdAction::OpenNew(snd,strn) => Some(Self::OpenEditor(snd,strn)),
      EdAction::ConfigAudio => Some(Self::OpenAudioConfig)
    }
  }

  pub fn from_conf_window(act:ConfAction) -> Option<Self> {
    match act {
      ConfAction::None => None,
      ConfAction::Cancel => None,
      ConfAction::Submit(sr) => Some(Self::SetupAudio(sr))
    }
  }
}
