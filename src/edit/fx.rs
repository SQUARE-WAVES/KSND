use super::Ctx;
use crate::{
  snd::Snd,
  blocks::Block
};

pub fn reverse(ctx:&Ctx) -> Ctx {
  let (s,e) = ctx.sample_region();

  let rev_seqs = ctx.seqs().map(|(_,active,seq)|{
    if active {
      let rev_section = seq.chunks(s..e).rev().map(|c|{
        let rev_samps = c.samples().rev().collect();
        Block::data(rev_samps)
      }).collect();

      seq.replace(s,e,&rev_section)
    }
    else {
      seq.clone()
    }
  });

  let rev_snd = Snd::from_iter(ctx.snd.sample_rate(),rev_seqs);
  ctx.flip(rev_snd.into())
}

