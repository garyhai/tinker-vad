//! A basic example of using the asr library.
//!
//! Run with `cargo run --example segment -- <path-to-wav-file>`.
//! `cargo run --example segment -- "/Users/gary/Desktop/clips/english.wav"`

use std::{fs::File, io::BufReader, path::PathBuf};

use rodio::{
  buffer::SamplesBuffer, source::UniformSourceIterator, Decoder, OutputStream,
  Sink, Source as _,
};
use tinker_vad::{Vad, VadConfig};

fn main() {
  env_logger::init();
  let mut args = std::env::args();
  args.next();
  let arg1 = args.next().expect("first argument should be path to WAV file");
  let input_file: PathBuf = arg1.into();
  if !input_file.exists() {
    panic!("audio file doesn't exist");
  }
  // let filename = input_file.file_stem().unwrap().to_string_lossy();
  let stream = BufReader::new(File::open(&input_file).unwrap());
  let source = Decoder::new(stream).unwrap();
  let audio: Vec<f32> = UniformSourceIterator::<_, f32>::new(source, 1, 16000)
    .convert_samples()
    .collect();

  let config = VadConfig::new(500, 16000);

  let mut vad = Vad::new(config);
  vad.load("").expect("failed to load VAD model");
  vad.segment_audio(&audio).expect("failed to segment audio");
  for segment in vad.get_segments().iter() {
    println!("Peek: {:?}", segment);
  }
  while let Some(segement) = vad.yield_segment() {
    println!("Yield: {:?}", segement);
  }

  let segments = vad.flush().expect("failed to flush segments");
  for segment in segments {
    println!("Flush: {:?}", segment);
  }

  vad.reset().expect("failed to reset VAD");
  vad.segment_audio(&audio).expect("failed to segment audio");
  for segment in vad.flush().unwrap() {
    println!("Flush: {:?}", segment);
    // play_audio(&audio[segment.0..segment.1]);
  }

}

#[allow(dead_code)]
fn play_audio(audio: impl Into<Vec<f32>>) {
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();
  let source = SamplesBuffer::new(1, 16000, audio);
  sink.append(source);
  sink.sleep_until_end();
}
