use iced::{
  Element,
  widget::{
    scrollable,
    Column
  },
  advanced::widget::Text,
  Color
};

#[derive(Clone)]
pub enum Ptype {
  Nfo(String),
  Err(String)
}

pub fn view(history:Vec<Ptype>) -> Element<'static,()> {
  let col = Column::new();
  let col = col.extend(history.into_iter().map(|p| {
    match p {
      Ptype::Nfo(s) => Text::new(s).into(),
      Ptype::Err(s) => Text::new(s).color(Color::from_rgb(1.0,0.0,0.0)).into()
    }
  }));

  scrollable(col.width(600)).into()
}
