use rand::Rng;

use crate::constants::PALSIZE;

pub struct Palette {
    pub colors: Vec<u32>,
    phase_r: f32,
    phase_g: f32,
    phase_b: f32,
    freq: f32,
    invert: bool,
    offset_accum: u32,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: vec![0; PALSIZE],
            phase_r: 0.0,
            phase_g: 0.0,
            phase_b: 0.0,
            freq: 1.0,
            invert: false,
            offset_accum: 0,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.freq = 1.0 + rng.gen::<f32>().powf(3.0) * 256.0;
        self.phase_r = self.freq * rng.gen::<f32>() * std::f32::consts::PI;
        self.phase_g = self.freq * rng.gen::<f32>() * std::f32::consts::PI;
        self.phase_b = self.freq * rng.gen::<f32>() * std::f32::consts::PI;
        self.invert = rng.gen::<f32>() > 0.75;
        self.build();
    }

    fn build(&mut self) {
        for i in 0..PALSIZE {
            let t = i as f32 / PALSIZE as f32;
            let r = (0.5 + 0.5 * (self.phase_r + t * self.freq).sin()).clamp(0.0, 1.0);
            let g = (0.5 + 0.5 * (self.phase_g + t * self.freq).sin()).clamp(0.0, 1.0);
            let b = (0.5 + 0.5 * (self.phase_b + t * self.freq).sin()).clamp(0.0, 1.0);
            let (r, g, b) = if self.invert { (1.0 - r, 1.0 - g, 1.0 - b) } else { (r, g, b) };
            self.colors[i] = ((r * 255.0) as u32) << 16
                | ((g * 255.0) as u32) << 8
                | ((b * 255.0) as u32);
        }
    }

    pub fn offset(&mut self) -> u32 {
        self.offset_accum = self.offset_accum.wrapping_add(3);
        self.offset_accum & (PALSIZE as u32 - 1)
    }
}


