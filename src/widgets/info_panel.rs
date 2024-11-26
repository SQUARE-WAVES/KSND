
use iced::{
  Element,
  widget::{row,column},
  advanced::widget::Text
};

use crate::{
  edit::Editor,
  util::formatters
};

fn format_region(title:&str,r:crate::util::Region,sample_rate:f64) -> String {
  let (s,e) = r.into();
  let start_secs = s/sample_rate;
  let end_secs = e/sample_rate;
  format!("{}: {} - {}",title,formatters::seconds(start_secs),formatters::seconds(end_secs))
}

//you need this lifetime to prevent the element
//from holding onto the editor reference
pub fn view<'b>(ed:&Editor) -> Element<'b,()> {
  let ctx = ed.ctx();
  let sr = format!("sample rate: {}",ctx.snd.sample_rate());
  let lpm = format!("loop: {}",if ctx.loop_mode { "on" } else {"off"});

  let fsr = ctx.snd.sample_rate() as f64;

  let view_region = format_region("view",ctx.region(),fsr);

  let path = ed.path().map(|p|p.to_string()).unwrap_or("<No File>".to_string());

  let sel_region = ctx.selected_region()
  .map(|r|format_region("selection",r,fsr))
  .unwrap_or("<No Selection>".to_string());

  let cursor = ctx.cursor
  .map(|c|format!("cursor: {}",formatters::seconds(c/fsr)))
  .unwrap_or("<No Cursor>".to_string());

  row![
    column![Text::new(path),Text::new(sr),Text::new(view_region)].spacing(5),
    column![Text::new(lpm),Text::new(sel_region),Text::new(cursor)].spacing(5),
  ]
  .spacing(10)
  .into()
}
