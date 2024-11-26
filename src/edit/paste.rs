use crate::snd::Snd;
use super::Ctx;
use super::util;

//this is sorta the default "intellegent paste" that looks at your
//selections and cursors and does things based on that.
pub fn insert_or_replace(target:&Ctx,to_insert:&Snd) -> Ctx {

  if let Some(r) = target.selected_region() {
    let (s,e) = r.into();
    let new_snd = util::replace_multichannel(target,to_insert,(s,e));
    let mut new_ctx = target.flip(new_snd.into());
    new_ctx.cursor = Some(s);
    new_ctx.selection = Some(to_insert.len() as f64);
    return new_ctx;
  };

  let pt = target.cursor.unwrap_or(target.len());
  let new_snd = util::insert_multichannel(target,to_insert,pt);
  target.flip(new_snd.into())
}


pub fn mix_in(target:&Ctx,src:&Snd,src_gain:f32,target_gain:f32) -> Ctx {
  let (s,e) = match (target.cursor,target.selection) {
    (Some(pt),None) => (pt.ceil() as usize,target.snd.len()),
    _ => target.sample_region()
  };
  
  let mix_seqs = target.seqs().map(|(i,active,seq)|{
    if active {
      let src_channel = i.min(src.channels()-1);
      let mut src_samples = src.seqs()[src_channel].samples(..);
      seq.map_rng(s..e,|t_sample|{
        let src = src_samples.next().unwrap_or(0.0) * src_gain;
        t_sample.mul_add(target_gain,src)
      })
    }
    else {
      seq.clone()
    }
  });

  let new_snd = Snd::from_iter(target.snd.sample_rate(),mix_seqs);
  target.flip(new_snd.into())
}
