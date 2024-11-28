use crate::snd::Snd;
use super::Ctx;


//ok this is our default delete, that looks at the selection and the channels and such
pub fn remove_selected(ctx:&Ctx) -> Option<Ctx> {
  if let Some(r) = ctx.selected_region() {
    let (s,e) = r.sample_range();
    let deld = ctx.seqs().map(|(_,active,seq)|{
      if active {
        seq.delete(s,e)
      }
      else {
        seq.clone()
      }
    });

    let mut new_ctx = ctx.flip(Snd::from_iter(ctx.snd.sample_rate(),deld).into());
    new_ctx.cursor=Some(s as f64);
    new_ctx.selection = None;

    Some(new_ctx)
  }
  else {
    None
  }
}

//aka "Crop"
pub fn remove_non_selected(ctx:&Ctx) -> Option<Ctx> {
  if let Some(r) = ctx.selected_region() {
    let (s,e) = r.sample_range();
    let deld = ctx.seqs().map(|(_,active,seq)|{
      if active {
        seq.sub_seq(s..e)
      }
      else {
        seq.clone()
      }
    });

    let mut new_ctx = ctx.flip(Snd::from_iter(ctx.snd.sample_rate(),deld).into());
    new_ctx.cursor=None;
    new_ctx.selection = None;

    Some(new_ctx)
  }
  else {
    None
  }

}
