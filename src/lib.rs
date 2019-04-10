use std::f64;
extern crate rand;

pub trait Waveform {
    fn get_oscillator(&mut self) -> &mut Oscillator;

    fn process(&mut self) -> f64;

    fn get_frequency(&mut self) -> f64 {
        self.get_oscillator().get_frequency()
    }

    fn set_frequency(&mut self, frequency: f64) {
        self.get_oscillator().set_frequency(frequency);
    }
}

pub struct Sine {
    pub osc: Oscillator,
}

impl Sine {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            osc: Oscillator::new(frequency, sample_rate),
        }
    }
}

impl Waveform for Sine {
    fn get_oscillator(&mut self) -> &mut Oscillator {
        &mut self.osc
    }

    fn process(&mut self) -> f64 {
        let output = (2.0 * f64::consts::PI * self.osc.phase).sin();

        self.osc.increment_phase();

        output
    }
}

pub struct Square {
    osc: Oscillator,
}

impl Square {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            osc: Oscillator::new(frequency, sample_rate),
        }
    }
}

impl Waveform for Square {
    fn get_oscillator(&mut self) -> &mut Oscillator {
        &mut self.osc
    }

    fn process(&mut self) -> f64 {
        let sine = (2.0 * f64::consts::PI * self.osc.phase).sin();

        self.osc.increment_phase();

        if sine > 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}

pub struct Saw {
    osc: Oscillator,
}

impl Saw {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            osc: Oscillator::new(frequency, sample_rate),
        }
    }
}

impl Waveform for Saw {
    fn get_oscillator(&mut self) -> &mut Oscillator {
        &mut self.osc
    }

    fn process(&mut self) -> f64 {
        fmod(self.osc.phase, 1.0) * -2.0 + 1.0
    }
}

pub struct Noise {
    osc: Oscillator,
}

impl Noise {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            osc: Oscillator::new(frequency, sample_rate),
        }
    }
}

impl Waveform for Noise {
    fn get_oscillator(&mut self) -> &mut Oscillator {
        &mut self.osc
    }

    fn process(&mut self) -> f64 {
        rand::random::<f64>() * 2.0 - 1.0
    }
}

pub struct Oscillator {
    frequency: f64,
    sample_rate: f64,
    phase: f64,
    phase_step: f64,
}

impl Oscillator {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            frequency: frequency,
            sample_rate: sample_rate,
            phase: 0.0,
            phase_step: frequency / sample_rate,
        }
    }

    fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.calc_phase_step();
    }

    fn get_frequency(&self) -> f64 {
        self.frequency
    }

    fn increment_phase(&mut self) {
        self.phase = (self.phase + self.phase_step).fract();
    }

    fn calc_phase_step(&mut self) {
        self.phase_step = self.frequency / self.sample_rate;
    }
}

fn fmod(numer: f64, denom: f64) -> f64 {
    let rquot: f64 = (numer / denom).floor();
    numer - rquot * denom
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sine() {}
}
