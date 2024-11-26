use std::ops::RangeBounds;

use crate::util::range_bounds;
use super::Block;

#[derive(Clone)]
pub struct BlockSequence {
  blocks:Vec<(usize,Block)>
}

impl BlockSequence {
  pub fn len(&self) -> usize {
    self.blocks.last().map(|(i,b)|i+b.len()).unwrap_or(0)
  }

  //how many blocks
  pub fn blocks_len(&self) -> usize {
    self.blocks.len()
  }

  pub fn get_block(&self,block_idx:usize) -> Option<&(usize,Block)> {
    self.blocks.get(block_idx)
  }

  pub fn get_containing_block_index(&self,idx:&usize) -> Option<usize> {
    let search = self.blocks.binary_search_by(|(i,b)|{
      //remember you are returning whenther the block is greater than the index
      match idx {
        _ if *idx < *i => std::cmp::Ordering::Greater,
        _ if *idx >= *i+b.len() => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal
      }
    });

    search.ok()
  }

  fn get_containing_block(&self,idx:&usize) -> Option<&(usize,Block)> {
    self.get_containing_block_index(idx).map(|i|&self.blocks[i])
  }

  pub fn get_sample(&self,idx:&usize) -> Option<&f32> {
    self.get_containing_block(idx).and_then(|(i,b)|{
      b.get_sample(idx-i)
    })
  }

  pub fn summary(&self,st:usize,end:usize) -> (f32,f32) {
    if st > end {
      let val = self.get_sample(&st).copied().unwrap();
      return (val,val);
    }

    match end-st {
      0 if st < self.len() => {
        let val = self.get_sample(&st).copied().unwrap();
        (val,val)
      },

      _ if st >= self.len() => {
        (0.0,0.0)
      },

      _=> self.chunks(st..end).fold((f32::MAX,f32::MIN),|(min,max),c| {
        let (cmin,cmax) = c.summary();
        (min.min(cmin),max.max(cmax))
      })
    }
  }

  pub fn chunks<R:RangeBounds<usize>>(&self,rng:R) -> Chunker {
    let (start,end) = range_bounds(rng,self.len());
    Chunker::new(self,start,end)
  }

  pub fn samples<R:RangeBounds<usize>>(&self,rng:R) -> impl Iterator<Item=f32> + '_ {
    self.chunks(rng).flat_map(move |c|c.into_samples())
  }

  pub fn sub_seq<R:RangeBounds<usize>>(&self,rng:R) -> BlockSequence {
    Self::from_iter(self.chunks(rng))
  }

  pub fn map_rng<R,F>(&self,rng:R,mut proc:F) -> BlockSequence
  where
      R:RangeBounds<usize>,
      F:FnMut(f32)->f32
  {
    let (start,end) = range_bounds(rng,self.len());
    let mid = self.chunks(start..end);
    let prev = self.chunks(..start);
    let post = self.chunks(end..);

    let mid_map = mid.map(|c|c.map(&mut proc));

    BlockSequence::from_iter(prev.chain(mid_map).chain(post))
  }

  pub fn delete(&self,start:usize,end:usize) -> BlockSequence {
    let prev = self.chunks(..start);
    let post = self.chunks(end..);

    BlockSequence::from_iter(prev.chain(post))
  }

  pub fn insert(&self,at:usize,datums:&BlockSequence) -> BlockSequence {
    if at > self.len() {
      let pad_len = at - self.len();
      let pad = Block::silence(pad_len);

      let pad = std::iter::once(pad);
      let post = datums.chunks(..);

      self.chunks(..).chain(pad).chain(post).collect()
    }
    else {
      let mid = datums.chunks(..);
      let prev = self.chunks(..at);
      let post = self.chunks(at..);
      prev.chain(mid).chain(post).collect()
    }
  }

  pub fn replace(&self,start:usize,end:usize,datums:&BlockSequence) -> BlockSequence {
    let mid = datums.chunks(..);
    let prev = self.chunks(..start);
    let post = self.chunks(end..);

    BlockSequence::from_iter(prev.chain(mid).chain(post))
  }
}

impl FromIterator<Block> for BlockSequence {
  // Required method
  fn from_iter<T:IntoIterator<Item=Block>>(iterable: T) -> Self {
    let mut count = 0;

    let blocks = iterable.into_iter().map(move|block|{
      let start = count;
      count += block.len();
      (start,block)

    }).collect();

    Self{blocks}
  }
}

impl From<Block> for BlockSequence {
  fn from(block:Block) -> Self {
    std::iter::once(block).collect()
  }
}

//WOW its the chunker, its an iterator that lets you 
//iterate by blocks. So you can do things 1 block at a time
pub struct Chunker<'a> {
  seq:&'a BlockSequence,
  start:usize,
  si:usize,
  end:usize,
  ei:usize
}

impl<'a> Chunker<'a> {
  pub fn new(seq:&'a BlockSequence,start:usize,end:usize) -> Self {
    let si = seq.get_containing_block_index(&start).unwrap_or(seq.blocks_len());
    let ei = seq.get_containing_block_index(&(end.max(1) - 1)).unwrap_or(seq.blocks_len());

    Self{ seq, start,si, end,ei }
  }
}

impl<'a> Iterator for Chunker<'a> {
  type Item = Block;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }

    self.seq.get_block(self.si).map(|(i,b)| {
      let here = self.start;

      if self.end < i+b.len() {
        self.start = self.end;
        b.rng((here - i)..(self.end -i))
      }
      else {
        self.start = i+b.len();
        self.si += 1;
        b.rng(here-i..)
      }
    })
  }
}

impl<'a> DoubleEndedIterator for Chunker<'a> {
  fn next_back(&mut self) -> Option<Self::Item> {

    if self.end == self.start {
      return None
    }

    self.seq.get_block(self.ei).map(|(i,b)|{
      let here = self.end;

      if self.start >= *i {
        self.end = self.start;
        b.rng((self.start - i)..(here-i))
      }
      else {
        self.end = *i;
        self.ei -= 1;
        b.rng(..(here - i))
      }
    })
  }
}
