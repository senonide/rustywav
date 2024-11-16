use rodio::Source;
use std::time::Duration;

pub struct PulseWave {
    freq: f32,
    duty_cycle: f32,
    sample_rate: u32,
    t: f32,
    amplitude: f32,
    ramp_up_duration: f32,
    ramp_up_progress: f32,
    is_ramping_up: bool,
}

impl PulseWave {
    pub fn new(freq: f32, duty_cycle: f32) -> PulseWave {
        PulseWave {
            freq,
            duty_cycle,
            sample_rate: 44100,
            t: 0.0,
            amplitude: 0.0,
            ramp_up_duration: 0.02,
            ramp_up_progress: 0.0,
            is_ramping_up: true,
        }
    }

    fn apply_ramp_up(&mut self) {
        if self.is_ramping_up {
            self.ramp_up_progress += 1.0 / self.sample_rate as f32;
            if self.ramp_up_progress >= self.ramp_up_duration {
                self.is_ramping_up = false;
                self.amplitude = 1.0;
            } else {
                self.amplitude = self.ramp_up_progress / self.ramp_up_duration;
            }
        }
    }

    fn pulse_wave(&mut self) -> f32 {
        self.apply_ramp_up();

        let period = 1.0 / self.freq;
        let phase = self.t % period;

        if phase < self.duty_cycle * period {
            1.0 * self.amplitude
        } else {
            -1.0 * self.amplitude
        }
    }
}

impl Iterator for PulseWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let waveform = self.pulse_wave();
        self.t += 1.0 / self.sample_rate as f32;
        Some(waveform)
    }
}

impl Source for PulseWave {
    fn current_frame_len(&self) -> Option<usize> {
        None // Continuous audio
    }

    fn channels(&self) -> u16 {
        1 // Mono output
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate // Standard sample rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None // Indefinite duration
    }
}
