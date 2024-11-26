use iced::{
  Element,
  widget::{
    column,
    combo_box,
    row,
    button,
    combo_box::State as Cbs
  }
};

use wrappers::*;

#[derive(Debug,Clone)]
pub enum Msg {
  HostSelect(Apw),
  OutSelect(Diw),
  SrSelect(u32),
  BuffSelect(u32),
  OutStart(u32),
  OutChans(u32),
  SubmitBtn,
  CancelBtn
}

#[derive(Debug)]
pub enum OutMsg {
  None,
  Submit(StreamReq),
  Cancel
}

pub struct State {
  hosts:combo_box::State<Apw>,
  selected_host:Option<Apw>,

  out_devs:Option<combo_box::State<Diw>>,
  out_dev:Option<Diw>,
  out_starts:combo_box::State<u32>,
  out_start:u32,
  out_nums:combo_box::State<u32>,
  out_channels:u32,

  sample_rates:Option<combo_box::State<u32>>,
  sample_rate:Option<u32>,
  buff_sizes: combo_box::State<u32>,
  buff_size: u32
}

impl Default for State {
  fn default() -> Self {
    let buff_opts : Vec<u32> = (4..=16).map(|s|2u32.saturating_pow(s) as u32).collect();
    Self {
      hosts:combo_box::State::new(wrapped_apis()),
      selected_host:None,

      out_devs:None,
      out_dev:None,
      out_starts:Default::default(),
      out_start:0,
      out_nums:Default::default(),
      out_channels:u32::MAX,

      sample_rates:None,
      sample_rate:None,
      buff_sizes:combo_box::State::new(buff_opts),
      buff_size:256
    }
  }
}

impl State {
  fn setup_sample_rates(&mut self) {
    match self.out_dev.as_ref() {
      None => {
        self.sample_rates = None;
        self.sample_rate = None;
      },

      Some(od) => {
        self.sample_rates = Some(combo_box::State::new(od.inf.sample_rates.clone()));
      }
    }
  }

  fn setup_out_channels(&mut self) {
    if let Some(d) = &self.out_dev {
      self.out_starts = Cbs::new((0..d.inf.output_channels).collect());
      self.out_start = self.out_start.min(d.inf.output_channels-1);
      let max_channels = d.inf.output_channels - self.out_start;
      self.out_channels = self.out_channels.min(max_channels);
      
      self.out_nums = Cbs::new((1..=max_channels).collect());
    }
  }

  pub fn update(&mut self,msg:Msg) -> OutMsg {
    match msg {
      Msg::HostSelect(apw) =>  {
        let api = apw.api;
        self.selected_host = Some(apw);
        if let Ok(host) = rtaudio::Host::new(api) {
          let odvs : Vec<Diw> = host.iter_output_devices().map(Into::into).collect();
          self.out_devs= Some(Cbs::new(odvs));
        }
      }

      Msg::OutSelect(d) => {
        self.out_dev = Some(d);
        self.setup_sample_rates();
        self.setup_out_channels();
      },

      Msg::OutStart(c) => {
        if self.out_dev.is_some() {
          self.out_start = c;
        }

        self.setup_out_channels();
      },

      Msg::OutChans(c) => {
        self.out_channels = c;
      },

      Msg::SrSelect(r) => {
        self.sample_rate = Some(r)
      },

      Msg::BuffSelect(b) => {
        self.buff_size = b
      },

      Msg::SubmitBtn => {
        if let Some(req) = self.stream_request() {
          return OutMsg::Submit(req)
        }
      },

      Msg::CancelBtn => {
        return OutMsg::Cancel
      },
    };

    OutMsg::None
  }

  pub fn view(&self) -> Element<Msg> {
    let vert = iced::widget::vertical_space().height(20);
    let cbx = combo_box(&self.hosts,"pick a host",self.selected_host.as_ref(),Msg::HostSelect);
    let col = column![vert,cbx];

    let col = if let Some(s) = self.out_devs.as_ref() {
      let out_box = combo_box(s,"",self.out_dev.as_ref(),Msg::OutSelect);
      let out_box = tbox("output device",out_box);

      let start_box = if self.out_dev.is_some() && self.out_starts.options().len()>1 {
        let cb = combo_box(&self.out_starts,"",Some(&self.out_start),Msg::OutStart);
        Some(tbox("start channel",cb))
      }
      else {
        None
      };

      let num_box = if self.out_dev.is_some() && self.out_nums.options().len()>1 {
        let cb = combo_box(&self.out_nums,"",Some(&self.out_channels),Msg::OutChans);
        Some(tbox("number of channels",cb))
      }
      else {
        None
      };

      col.push("Output Device")
      .push(out_box)
      .push_maybe(start_box)
      .push_maybe(num_box)
      .push(" ")
    }
    else {
      col
    };

    let col = if let Some(s) = self.sample_rates.as_ref() {
      col.push(combo_box(s,"sample_rate",self.sample_rate.as_ref(),Msg::SrSelect))
    }
    else {
      col
    };

    let bsx = combo_box(&self.buff_sizes,"buffer_size",Some(&self.buff_size),Msg::BuffSelect);
    let col = col.push(bsx);

    let canceler = button("cancel").on_press(Msg::CancelBtn);

    let btns = if self.test() {
      row![button("submit").on_press(Msg::SubmitBtn),canceler].spacing(10)
    }
    else {
      row![canceler]
    };
    
    col.push(btns).padding(10).spacing(10).into()
  }

  pub fn output_params(&self) -> Option<rtaudio::DeviceParams> {
    self.out_dev.as_ref().map(|dev|{
      rtaudio::DeviceParams{
        device_id:dev.inf.id,
        num_channels:self.out_channels,
        first_channel:self.out_start
      }
    })
  }

  pub fn test(&self) -> bool {
    let one_dev = self.out_dev.is_some();
    let sr = self.sample_rate.is_some();

    self.selected_host.is_some() && one_dev && sr
  }

  pub fn stream_request(&self) -> Option<StreamReq> {
    self.selected_host.as_ref().and_then(|a| {
      let api = a.api;
      self.sample_rate.map(|sr|StreamReq {
        host_api:api,
        output_dev:self.output_params(),
        sample_rate:sr,
        buff_size:self.buff_size
      })
    })
  }
}

#[derive(Debug)]
pub struct StreamReq {
  pub host_api:rtaudio::Api,
  pub output_dev: Option<rtaudio::DeviceParams>,
  pub sample_rate:u32,
  pub buff_size:u32,
}

pub fn tbox<'a,T,Msg>(title:&'a str,cbox:iced::widget::ComboBox<'a,T,Msg>) -> Element<'a,Msg>
where
  T:Clone+std::fmt::Display + 'static,
  Msg:Clone + 'static
{
  row![title,cbox].spacing(10).into()
}


//A Bunch of wrappers to implement std::fmt::display
mod wrappers {
  use std::fmt::{
    Display,
    Formatter,
    Error
  };

  use rtaudio::{
    Api,
    DeviceInfo
  };

  #[derive(Debug,Copy,Clone)]
  pub struct Apw {
    pub api:Api
  }

  impl Display for Apw {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
      write!(f,"{}",self.api.get_display_name())
    }
  }

  impl From<Api> for Apw {
    fn from(api:Api) -> Self { Self{api} }
  }

  pub fn wrapped_apis() -> Vec<Apw> {
    rtaudio::compiled_apis().into_iter().map(Into::into).collect()
  }

  #[derive(Debug,Clone)]
  pub struct Diw {
    pub inf:DeviceInfo
  }

  impl From<DeviceInfo> for Diw {
    fn from(inf:DeviceInfo) -> Self { Self {inf} }
  }

  impl Display for Diw {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
      write!(f,"{}",self.inf.name)
    }
  }
}
