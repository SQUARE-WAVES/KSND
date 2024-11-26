use std::path::Path;

use dasp::Sample;
use anyhow::{anyhow,Result};

use super::Snd;
use crate::blocks;
use blocks::BlockSequence as Seq;

fn extract_channels<T,R>(r:&mut hound::WavReader<R>) -> Vec<f32> 
where
  T:hound::Sample + dasp::Sample + dasp::sample::ToSample<f32>,
  R:std::io::Read
{
  r.samples::<T>().map(|s|s.unwrap().to_sample::<f32>()).collect()
}

pub fn load_wav<P:AsRef<Path>>(p:P) -> Result<Snd> {
  let mut r = hound::WavReader::open(p)?;
  let spec = r.spec();
  let channels = spec.channels as usize;

  let samples = match(spec.sample_format,spec.bits_per_sample) {
    (hound::SampleFormat::Float,_) => { 
      extract_channels::<f32,_>(&mut r)
    },

    (hound::SampleFormat::Int,i) if i <= 16 => {
      extract_channels::<i16,_>(&mut r)
    },

    (hound::SampleFormat::Int,i) if i <= 32 => { 
      extract_channels::<i32,_>(&mut r)
    },

    _ => { Err(anyhow!("some kinda weird wav format here"))? }
  };

  let mut seqs :Vec<Seq> = vec![];
    
  for chan in 0..channels {
    let chan_smps : Vec<f32> = samples.iter().copied().skip(chan).step_by(channels).collect();
    let block = blocks::Block::data(chan_smps);
    seqs.push(block.into());
  }

  Ok(Snd::new(spec.sample_rate as usize,seqs))
}

pub fn save_wav<P:AsRef<Path>>(snd:&Snd,p:P) -> Result<()> {

  let spec = hound::WavSpec {
    channels: snd.as_ref().len() as u16,
    sample_rate: snd.sample_rate() as u32,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
  };

  let mut writer = hound::WavWriter::create(p, spec)?;

  for smp in snd.interleaved_audio() {
    writer.write_sample(smp.to_sample::<i16>())?;
  }

  Ok(())
}

