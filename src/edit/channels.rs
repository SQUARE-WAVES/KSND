use super::Ctx;
use crate::Snd;

pub fn solo(ctx:&Ctx,channel:usize) -> Ctx {
  let seqs = ctx.seqs().filter(|(i,_,_)| *i == channel).map(|(_,_,seq)|seq.clone());
  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),seqs);
  let mut nc = ctx.flip(new_snd.into());
  nc.channels.solo();
  nc
}

pub fn delete(ctx:&Ctx,channel:usize) -> Ctx {
  let seqs = ctx.seqs().filter(|(i,_,_)| *i != channel).map(|(_,_,seq)|seq.clone());
  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),seqs);
  let mut nc = ctx.flip(new_snd.into());
  nc.channels.shift_after(channel);
  nc
}

pub fn insert(ctx:&Ctx) -> Ctx {
  use crate::blocks;
  let silence : blocks::BlockSequence = blocks::Block::silence(ctx.snd.len()).into();
  let seqs = ctx.seqs().map(|(_,_,s)|s.clone()).chain(std::iter::once(silence));

  let new_snd = Snd::from_iter(ctx.snd.sample_rate(),seqs);
  ctx.flip(new_snd.into())
}
