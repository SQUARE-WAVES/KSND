use std::sync::Arc;
use std::sync::mpsc::{Sender,Receiver,channel};

use anyhow::{anyhow,Result};
use iced::window::Id;

pub mod config_window;

pub use config_window::{
  State as ConfWin,
  Msg as ConfMsg,
  OutMsg as ConfAction,
  StreamReq
};

use rtaudio::{
  Api,
  Buffers,
  StreamStatus,
  StreamInfo,
  StreamHandle,
  StreamOptions,
  SampleFormat
};

use crate::snd::Snd;

mod player;
use player::Player;

pub enum Cmd {
  Play(Id,Arc<Snd>,f64,f64,f64,bool),
  Stop(Id)
}

#[derive(Debug,Copy,Clone)]
pub enum OutMsg {
  Playback(Id,usize),
  Stop(Id)
}

struct AudioStreamConnection {
  stream:StreamHandle,
  tx:Sender<Cmd>,
  rx:Receiver<OutMsg>
}

pub struct AudioSystemInterface {
  connection:Option<AudioStreamConnection>,
}

impl AudioSystemInterface {
  //not sure about the timing here
  pub fn poll<F:FnMut(OutMsg)>(&self,handler:F) {
    if let Some(conn) = self.connection.as_ref() {
      conn.rx.try_iter().for_each(handler)
    }
  }

  pub fn new_default() -> Result<Self> {
    let host = rtaudio::Host::new(Api::Unspecified)?;
    let out_device = host.default_output_device()?;
    let sample_rate = out_device.preferred_sample_rate;

    let stream_results = host.open_stream(
      Some(rtaudio::DeviceParams {
        device_id:out_device.id,
        num_channels:out_device.output_channels,
        first_channel:0
      }),
      None,
      SampleFormat::Float32,
      sample_rate,
      256,
      StreamOptions::default(),
      |err| eprintln!("{}",err)
    );

    let mut handle = match stream_results {
      Ok(h) => h,
      Err((_,err)) => {
        return Err(anyhow!("failed to open the default stream: {}",err));
      }
    };

    let (tx,rx) = channel::<Cmd>();
    let (sys_tx,sys_rx) = channel::<OutMsg>();

    let stream_cb = {
      let mut current_play : Option<player::Player> = None;

      move |buffers: Buffers<'_>,nfo:&StreamInfo,_stat: StreamStatus| {
        if let Buffers::Float32 {output,..} = buffers {
          
          clear_buffs(output);

          if let Some(ref mut player) = current_play.as_mut() {
            let keep_on= fill_buffs(output,player);

            if !keep_on {
              let id = player.id();
              current_play = None;
              sys_tx.send(OutMsg::Stop(id)).expect("sending failed");
            }
            else {
              sys_tx.send(OutMsg::Playback(player.id(),player.play_pos())).expect("send_failed");
            }
          }
        }

        let req = rx.try_recv();

        match (req,current_play.as_mut()) {
          (Ok(Cmd::Play(id,snd,rs,re,pt,lp)),Some(p)) if p.id() != id => {
            sys_tx.send(OutMsg::Stop(p.id())).expect("sending failed");
            current_play = Some(player::Player::new(id,snd,nfo.sample_rate as f64,rs,re,pt,lp));
          },

          (Ok(Cmd::Play(id,snd,rs,re,pt,lp)),_) => {
            current_play = Some(player::Player::new(id,snd,nfo.sample_rate as f64,rs,re,pt,lp));
          },
          
          (Ok(Cmd::Stop(id)),Some(p)) if id == p.id() => {
            current_play = None;
            sys_tx.send(OutMsg::Stop(id)).expect("sending failed");
          },

          _ => ()
        }
      }
    };

    handle.start(stream_cb)?;

    let connection = AudioStreamConnection {
      stream:handle,
      tx,
      rx:sys_rx
    };

    Ok(Self {
      connection:Some(connection)
    })
  }

  pub fn change_stream(&mut self,srq:StreamReq) -> Result<()> {
    let old_connection = self.connection.take();
    old_connection.unwrap().stream.close();

    let host = rtaudio::Host::new(srq.host_api)?;

    let stream_results = host.open_stream(
      srq.output_dev,
      None,
      SampleFormat::Float32,
      srq.sample_rate,
      srq.buff_size,
      StreamOptions::default(),
      |err| eprintln!("{}",err)
    );

    let mut handle = match stream_results {
      Ok(h) => h,
      Err((_,err)) => {
        return Err(anyhow!("failed to open the default stream: {}",err));
      }
    };

    let (tx,rx) = channel::<Cmd>();
    let (sys_tx,sys_rx) = channel::<OutMsg>();

    let stream_cb = {
      let mut current_play : Option<player::Player> = None;

      move |buffers: Buffers<'_>,nfo:&StreamInfo,_stat: StreamStatus| {
        if let Buffers::Float32 {output,..} = buffers {
          
          clear_buffs(output);

          if let Some(ref mut player) = current_play.as_mut() {
            let keep_on= fill_buffs(output,player);

            if !keep_on {
              let id = player.id();
              current_play = None;
              sys_tx.send(OutMsg::Stop(id)).expect("sending failed");
            }
            else {
              sys_tx.send(OutMsg::Playback(player.id(),player.play_pos())).expect("send_failed");
            }
          }
        }

        let req = rx.try_recv();

        match (req,current_play.as_mut()) {
          (Ok(Cmd::Play(id,snd,rs,re,pt,lp)),Some(p)) if p.id() != id => {
            sys_tx.send(OutMsg::Stop(p.id())).expect("sending failed");
            current_play = Some(player::Player::new(id,snd,nfo.sample_rate as f64,rs,re,pt,lp));
          },

          (Ok(Cmd::Play(id,snd,rs,re,pt,lp)),_) => {
            current_play = Some(player::Player::new(id,snd,nfo.sample_rate as f64,rs,re,pt,lp));
          },
          
          (Ok(Cmd::Stop(id)),Some(p)) if id == p.id() => {
            current_play = None;
            sys_tx.send(OutMsg::Stop(id)).expect("sending failed");
          },

          _ => ()
        }
      }
    };

    handle.start(stream_cb)?;

    let connection = AudioStreamConnection {
      stream:handle,
      tx,
      rx:sys_rx
    };

    self.connection = Some(connection);
    
    Ok(())
  }

  pub fn play(&self,id:Id,snd:Arc<Snd>,s:f64,e:f64,pt:f64,lp:bool) {
    if let Some(conn) = self.connection.as_ref() {
      conn.tx.send(Cmd::Play(id,snd.clone(),s,e,pt,lp)).expect("audio send broke");
    }
    else {
      println!("no connection");
    }
  }

  pub fn stop(&self,id:Id) {
    if let Some(conn) = self.connection.as_ref() {
      conn.tx.send(Cmd::Stop(id)).expect("audio send broke");
    }
  }
}

fn clear_buffs(buffs:&mut [f32]) {
  buffs.fill(0.0)
}
  
fn fill_buffs(buffs:&mut [f32],player:&mut Player) -> bool {
  let mut keep_on = true;

  for i in 0..buffs.len()/2 {
    let data_frame = &mut buffs[2*i..=(2*i)+1];
    keep_on = player.tick(data_frame);
  }
  
  keep_on
}
