# waveforms
Simple audio synthesis waveforms

### Support waveforms:
Sine, square, sawtooth, noise

### Usage
```
extern crate waveforms;

use waveforms::{Oscillator, Waveform};

fn main() {
    let mut osc = Oscillator::new(Waveform::Sine, 440.0, 44100.0);

    for _ in 0..10 {
        println!("{}", osc.get_amplitude());
    }
}
```