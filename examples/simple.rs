extern crate hound;
extern crate waveforms;

use std::i16;
use waveforms::{Oscillator, Waveform};

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("example.wav", spec).unwrap();

    let mut osc = Oscillator::new(Waveform::Sine, 440.0, 44100.0);

    for _t in (0..44100 / 24).map(|x| x as f32 / 44100.0) {
        let sample = osc.get_amplitude() as f32;
        let amplitude = (i16::MAX / 2) as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
