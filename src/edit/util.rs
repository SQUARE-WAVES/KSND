use crate::snd::Snd;
use super::Ctx;

pub fn insert_multichannel(ctx:&Ctx,to_insert:&Snd,point:f64) -> Snd {
  //inserts need to occur at sample bounds
  let insert_point = point.floor() as usize;
  let insert_point = insert_point.min(ctx.snd.len());

  let new_seqs = ctx.seqs().map(|(n,active,main_seq)|{
    if active {
      //if the insert sound doesn't have enough channels, just use the closest
      //this might get weird for 2 sounds with more than 2 channels. I haven't really
      //thought about it
      let insert_idx = n.min(to_insert.channels()-1);
      let insert_sequence = to_insert.channel(insert_idx).map(|iseq|{
        main_seq.insert(insert_point,iseq)
      });

      insert_sequence.unwrap_or(main_seq.clone())
    }
    else {
      //don't change anything
      main_seq.clone()
    }
  });

  Snd::from_iter(ctx.snd.sample_rate(),new_seqs)
}

pub fn replace_multichannel(ctx:&Ctx,to_insert:&Snd,region:(f64,f64)) -> Snd {
  //the region needs to be set to sample boundaries and trimmed to the length
  //of the main sound
  let (start,end) = region;
  let (start,end) = (start.min(end),end.max(start));
  let (start,end) = (start as usize,end as usize);
  let (start,end) = (start.min(ctx.snd.len()),end.min(ctx.snd.len()));

  let new_seqs = ctx.seqs().map(|(n,active,main_seq)|{
    if active {
      //if the insert sound doesn't have enough channels, just use the closest
      //this might get weird for 2 sounds with more than 2 channels. I haven't really
      //thought about it
      let insert_idx = n.min(to_insert.channels() - 1);
      let insert_sequence = to_insert.channel(insert_idx).map(|iseq|{
        main_seq.replace(start,end,iseq)
      });

      insert_sequence.unwrap_or(main_seq.clone())
    }
    else {
      //don't change anything
      main_seq.clone()
    }
  });

  Snd::from_iter(ctx.snd.sample_rate(),new_seqs)
}

