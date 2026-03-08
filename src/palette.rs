use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f32::consts::PI;

use crate::constants::PALSIZE;

pub struct Palette {
    pub colors: Vec<u32>,
    // Store parameters to rebuild if needed, but randomization overwrites them
    pub offset_accum: u32,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colors: vec![0; PALSIZE],
            offset_accum: 0,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.generate(&mut rng);
    }

    pub fn randomize_with_seed(&mut self, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        self.generate(&mut rng);
    }

    // Ported from C++ CreatePalette (lines 2420-2534)
    fn generate<R: Rng>(&mut self, rng: &mut R) {
        let freq = 1.0 + rng.gen::<f32>().powf(3.0) * 256.0;
        let rf = freq * rng.gen::<f32>() * PI;
        let gf = freq * rng.gen::<f32>() * PI;
        let bf = freq * rng.gen::<f32>() * PI;

        let invert = rng.gen::<f32>() > 0.75;
        let lightobject = rng.gen::<f32>() > 0.75;
        let vertin = rng.gen::<f32>() > 0.75;
        let heatvawe = rng.gen::<f32>() > 0.95;
        let sinvawe = rng.gen::<f32>() > 0.5;
        let bakwrds = rng.gen::<f32>() > 0.5;
        
        // C++: fade = int(RND * 2) - unused in logic? It says "fade = int(RND*2); for..."
        // Ah, fade isn't used inside the loop in the provided snippet?
        // wait, "fade" is local variable, not used. "bakwrds" controls direction.

        for i in 0..PALSIZE {
            let mut fdout = (PALSIZE - i) as f32 / PALSIZE as f32;
            if bakwrds {
                fdout = i as f32 / PALSIZE as f32;
            }

            let fdout2 = fdout * fdout;
            let fdouts = fdout.sqrt();

            let fdin = 1.0 - fdout;
            let fdin2 = 1.0 - fdouts;
            let fdins = 1.0 - fdout2;

            let ufade0 = fdout;
            // let ufade1 = fdins; // Unused in C++ snippet provided directly?
            let mut ufade2 = fdout2;
            // let ufade3 = fdouts;

            if vertin {
                ufade2 = 1.0 - ufade2;
            }
            // Wait, C++: "freq = rf * ufade0 * sqrt(ufade0);"
            // The snippet uses `ufade0`.

            // Red
            let mut freq_val = rf * ufade0 * ufade0.sqrt();
            let mut rl = (1.0 + freq_val.cos()) * 0.5;
            
            // Green
            freq_val = gf * ufade0 * ufade0.sqrt();
            let mut gl = (1.0 + freq_val.cos()) * 0.5;
            
            // Blue
            freq_val = bf * ufade0 * ufade0.sqrt();
            let mut bl = (1.0 + freq_val.cos()) * 0.5;

            if lightobject {
                if vertin {
                    let length = (rl * rl + gl * gl + bl * bl).sqrt() * 2.0;
                    if length > 0.0001 { // avoid div zero
                         rl = ((1.0 + rl) / length) * fdin;
                         gl = ((1.0 + gl) / length) * fdin;
                         bl = ((1.0 + bl) / length) * fdin;
                    }
                } else {
                    rl = fdin2;
                    gl = fdin2;
                    bl = fdin2;
                }
            }

            if heatvawe {
                let f = fdin2 * (2.0 * PI);
                // rad = 2pi/360. 240*rad = 240/360 * 2pi = 4/3 pi
                let rad240 = (240.0 / 360.0) * 2.0 * PI;
                let rad120 = (120.0 / 360.0) * 2.0 * PI;
                
                rl = (1.0 - ((1.0 + (f + rad240).sin()) * 0.5 * fdout)) * fdins;
                gl = (1.0 - ((1.0 + (f + rad120).sin()) * 0.5 * fdout)) * fdins;
                bl = (1.0 - ((1.0 + f.sin()) * 0.5 * fdout)) * fdins;
            }

            if sinvawe {
                 rl *= (1.0 + (fdout * PI * 4.1 * 2.0).sin()) / 2.0; // C++ uses `pii` which is 2*pi? 
                 // C++: "rl *= ( 1.0f + sin( fdout * pii * 4.1f ) ) / 2.0f;"
                 // pii is 2*pi.
                 gl *= (1.0 + (fdout * PI * 2.0 * 4.2).sin()) / 2.0;
                 bl *= (1.0 + (fdout * PI * 2.0 * 4.3).sin()) / 2.0;
            }

            if invert {
                if vertin {
                    rl = (2.0 - rl) / 2.0;
                    gl = (2.0 - gl) / 2.0;
                    bl = (2.0 - bl) / 2.0;
                } else {
                    rl = 1.0 - rl;
                    gl = 1.0 - gl;
                    bl = 1.0 - bl;
                }
            }

            // Clamp and convert
            let r = (rl.clamp(0.0, 1.0) * 255.0) as u32;
            let g = (gl.clamp(0.0, 1.0) * 255.0) as u32;
            let b = (bl.clamp(0.0, 1.0) * 255.0) as u32;

            self.colors[i] = (r << 16) | (g << 8) | b;
        }
    }
    
    pub fn offset(&mut self) -> u32 {
        self.offset_accum = self.offset_accum.wrapping_add(3);
        self.offset_accum & (PALSIZE as u32 - 1)
    }
}
