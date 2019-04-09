use std::f64;
extern crate rand;

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Noise,
}

pub struct Oscillator {
    waveform: Waveform,
    frequency: f64,
    sample_rate: f64,
    phase: f64,
    phase_step: f64,
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency: f64, sample_rate: f64) -> Self {
        Self {
            waveform: waveform,
            frequency: frequency,
            sample_rate: sample_rate,
            phase: 0.0,
            phase_step: frequency / sample_rate,
        }
    }

    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calc_phase_step();
    }

    pub fn get_frequency(&self) -> f64 {
        self.frequency
    }

    pub fn set_sample_rate(&mut self, frequency: f64) {
        self.sample_rate = frequency;
        self.calc_phase_step();
    }

    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }

    pub fn get_amplitude(&mut self) -> f64 {
        let output = match self.waveform {
            Waveform::Sine => sine_sample(&self),
            Waveform::Square => square_sample(&self),
            Waveform::Saw => saw_sample(&self),
            Waveform::Noise => noise_sample(&self),
        };

        self.phase = (self.phase + self.phase_step).fract();

        output
    }

    fn calc_phase_step(&mut self) {
        self.phase_step = self.frequency / self.sample_rate;
    }
}

fn sine_sample(osc: &Oscillator) -> f64 {
    (2.0 * f64::consts::PI * osc.phase).sin()
}

fn square_sample(osc: &Oscillator) -> f64 {
    let sine = sine_sample(&osc);
    if sine > 0.0 {
        1.0
    } else {
        -1.0
    }
}

fn saw_sample(osc: &Oscillator) -> f64 {
    fmod(osc.phase, 1.0) * -2.0 + 1.0
}

fn noise_sample(_osc: &Oscillator) -> f64 {
    rand::random::<f64>() * 2.0 - 1.0
}

pub fn fmod(numer: f64, denom: f64) -> f64 {
    let rquot: f64 = (numer / denom).floor();
    numer - rquot * denom
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sine() {}
}
