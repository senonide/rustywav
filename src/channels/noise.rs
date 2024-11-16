use rodio::Source;
use std::time::Duration;

pub struct NoiseChannel {
    sample_rate: u32,
    t: u32,
    seed: u32,
    metallic: bool,
    amplitude: f32,
    ramp_up_duration: f32,
    ramp_up_progress: f32,
    is_ramping_up: bool,
}

impl NoiseChannel {
    pub fn new(seed: u32, metallic: bool) -> NoiseChannel {
        NoiseChannel {
            sample_rate: 44100,
            t: 0,
            seed,
            metallic,
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

    fn generate_noise(&mut self) -> f32 {
        self.apply_ramp_up();

        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 9;
        self.seed ^= self.seed >> 13;

        let value = if self.metallic {
            ((self.seed & 1) as f32 * 2.0 - 1.0) * 0.8
        } else {
            (self.seed & 1) as f32 * 2.0 - 1.0
        };

        value * self.amplitude
    }
}

impl Iterator for NoiseChannel {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let waveform = self.generate_noise();
        self.t += 1;
        Some(waveform)
    }
}

impl Source for NoiseChannel {
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
