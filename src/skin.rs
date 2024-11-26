use iced::{
  Color,
  Border,
  widget::container
};

pub struct Skin {
  pub waveform_border:Color,
  pub console_border:Color,
  pub info_border:Color
}

fn sqr_border(color:Color,width:f32) -> Border {
  Border{
    color,
    width,
    radius:Default::default()
  }
}

impl Skin {
  pub fn waveform_box(&self) ->  container::Style {
    container::Style{
      text_color:None,
      background:None,
      border: sqr_border(self.waveform_border,1.0),
      shadow:Default::default()
    }
  }

  pub fn console_box(&self) -> container::Style {
    container::Style{
      text_color:None,
      background:None,
      border: sqr_border(self.console_border,1.0),
      shadow:Default::default()
    }
  }

  pub fn info_box(&self) -> container::Style {
    container::Style{
      text_color:None,
      background:None,
      border: sqr_border(self.info_border,1.0),
      shadow:Default::default()
    }
  }
}

impl Default for Skin {
  fn default() -> Self {
    Self {
      waveform_border:Color::from_rgb(1.0,0.0,1.0),
      console_border:Color::from_rgb(1.0,0.0,1.0),
      info_border:Color::from_rgb(1.0,0.0,1.0)
    }
  }
}
