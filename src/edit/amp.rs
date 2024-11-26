use super::Ctx;
use crate::Snd;

pub fn gain(ctx:&Ctx,amt:f32) -> Ctx {
  let (s,e) = ctx.sample_region();

  let gain_seqs = ctx.seqs().map(|(_,active,seq)|{
    if active {
      seq.map_rng(s..e,|sample| sample*amt)
    }
    else {
      seq.clone()
    }
  });

  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),gain_seqs);
  ctx.flip(new_snd.into())
}

pub fn lin_fade(ctx:&Ctx,start:f32,end:f32) -> Ctx {
  let (s,e) = ctx.sample_region();
  let step = (end-start)/(e - s) as f32;

  let gain_seqs = ctx.seqs().map(|(_,active,seq)|{
    if active {
      let mut amt = start;
      seq.map_rng(s..e,|sample| {
        let out = sample * amt;
        amt += step;
        out
      })
    }
    else {
      seq.clone()
    }
  });

  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),gain_seqs);
  ctx.flip(new_snd.into())
}

pub fn normalize(ctx:&Ctx,level:Option<f32>) -> Ctx {
  let level = level.unwrap_or(1.0);
  let (s,e) = ctx.sample_region();
  
  let max = ctx.seqs().fold(0.0f32,|accum,(_,active,seq)|{
    if active {
      let (seq_min,seq_max) = seq.summary(s,e);
      accum.max(seq_max.abs()).max(seq_min.abs())
    }
    else {
      accum
    }
  });

  let amt = level/max;
  gain(ctx,amt)
}
