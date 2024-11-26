use super::Ctx;
use crate::snd::Snd;
use crate::dsp;
use crate::blocks::Block;

pub fn resample(ctx:&Ctx,rate:f64,q:usize) -> Ctx {
  let sr = ctx.snd.sample_rate() as f64;
  let ratio = sr/rate;

  let new_channels = ctx.snd.seqs().iter().map(|sq|{
    crate::blocks::Block::data(dsp::window_resample(sq.samples(..),ratio,q)).into()
  });

  let new_snd = Snd::from_iter(rate as usize,new_channels);

  ctx.flip(new_snd.into())
}

pub fn pitch(ctx:&Ctx,ratio:f64,q:usize) -> Ctx {
  let mut out_sel_len = 0.0f64;

  let (start,end) = ctx.sample_region();
  
  let new_seqs = ctx.seqs().map(|(_,active,seq)| {
    if active {
      let p = Block::data(dsp::window_resample(seq.sub_seq(start..end).samples(..),ratio,q));
      out_sel_len = p.len() as f64;
      seq.replace(start,end,&p.into())
    }
    else {
      seq.clone()
    }
  });

  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),new_seqs);
  let mut new_ctx = ctx.flip(new_snd.into());

  match (ctx.cursor,ctx.selection) {
    (Some(_),Some(len)) if len >=0.0  => {
      new_ctx.selection = Some(out_sel_len);
    }
    (Some(pt),Some(len)) if len < 0.0 => {
      new_ctx.cursor = Some(pt+(out_sel_len+len));
      new_ctx.selection = Some(-out_sel_len);
    },
    (_,_) => ()
  }

  new_ctx
}
