use crate::constants::{BHEIGHT, BOTTOM_COLORS, BWIDTH, FRACTAL_COLORS, HEIGHT, LHEIGHT, LWIDTH, WIDTH};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub struct LightCalibration {
    pub minbright: f32,
    pub maxbright: f32,
}

pub struct BottomState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub bglow: f32,
    pub blargel: f32,
    pub bcr: u32,
    pub bcg: u32,
    pub bcb: u32,
    pub rng: SmallRng,
}

impl BottomState {
    pub fn new(seed: u64) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            bglow: 1.0,
            blargel: 0.0001,
            bcr: BOTTOM_COLORS[0].0 as u32,
            bcg: BOTTOM_COLORS[0].1 as u32,
            bcb: BOTTOM_COLORS[0].2 as u32,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn reseed(&mut self, seed: u64) {
        *self = Self::new(seed);
    }
}

pub struct FractalState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub dcr: u8,
    pub dcg: u8,
    pub dcb: u8,
    pub pali: usize,
    pub pali2: usize,
    pub repti: i32,
    pub maxrepti: i32,
    pub sduoi: bool,
    pub smulti: usize,
    pub indxn: i32,
    pub indxuse: usize,
    pub indxs: usize,
    pub swapflag: bool,
    pub probability: f32,
    pub useswap: bool,
    pub last_pmodi: u32,
    pub last_coli: usize,
    pub rng: SmallRng,
}

impl FractalState {
    pub fn new(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let maxrepti = (rng.gen::<f32>() * rng.gen::<f32>() * 128.0) as i32;
        let indxn = if rng.gen_bool(0.5) { -1 } else { 0 };
        let useswap = rng.gen_bool(0.5);
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            dcr: FRACTAL_COLORS[0].0,
            dcg: FRACTAL_COLORS[0].1,
            dcb: FRACTAL_COLORS[0].2,
            pali: 0,
            pali2: 0,
            repti: -1,
            maxrepti,
            sduoi: false,
            smulti: 0,
            indxn,
            indxuse: 0,
            indxs: 0,
            swapflag: false,
            probability: 0.0,
            useswap,
            last_pmodi: 0,
            last_coli: 0,
            rng,
        }
    }

    pub fn reseed(&mut self, seed: u64) {
        *self = Self::new(seed);
    }
}

pub struct Framebuffers {
    pub screen: Vec<u32>,
    pub pict: Vec<u32>,
    pub zbuf: Vec<i32>,
    pub light: Vec<i32>,
    pub lighting: LightCalibration,
    pub bottom_view: BottomState,
    pub bottom_shadow: BottomState,
    pub fractal_view: FractalState,
    pub fractal_shadow: FractalState,
    pub width: usize,
    pub height: usize,
}

impl Framebuffers {
    pub fn new() -> Self {
        let width = WIDTH as usize;
        let height = HEIGHT as usize;
        let pict_w = (BWIDTH as usize).max(width);
        let pict_h = (BHEIGHT as usize).max(height);
        let light_w = LWIDTH as usize;
        let light_h = LHEIGHT as usize;

        Self {
            screen: vec![0; width * height],
            pict: vec![0; pict_w * pict_h],
            zbuf: vec![0; pict_w * pict_h],
            light: vec![0; light_w * light_h],
            lighting: LightCalibration {
                minbright: 1.0,
                maxbright: 0.0001,
            },
            bottom_view: BottomState::new(0xB077_0001),
            bottom_shadow: BottomState::new(0xB077_0002),
            fractal_view: FractalState::new(0xF4AC_0001),
            fractal_shadow: FractalState::new(0xF4AC_0002),
            width,
            height,
        }
    }

    pub fn clear_view(&mut self, rgb: u32) {
        self.pict.fill(rgb);
        self.zbuf.fill(0);
    }

    pub fn clear_light(&mut self) { self.light.fill(0); }

    pub fn reset_lighting(&mut self, lightness: i32) {
        if lightness == 1 {
            self.lighting.minbright = 1.0 / 3.0;
            self.lighting.maxbright = 1.0 / 2.0;
        } else {
            self.lighting.minbright = 1.0;
            self.lighting.maxbright = 0.0001;
        }
    }

    pub fn reset_all_state(&mut self, lightness: i32) {
        self.bottom_view.reseed(0xB077_0001);
        self.bottom_shadow.reseed(0xB077_0002);
        self.fractal_view.reseed(0xF4AC_0001);
        self.fractal_shadow.reseed(0xF4AC_0002);
        self.reset_lighting(lightness);
    }

    pub fn resolve_2x2_to_screen(&mut self) {
        let pict_w = BWIDTH as usize;
        for sy in 0..self.height {
            let py = sy << 1;
            let row0 = py * pict_w;
            let row1 = (py + 1) * pict_w;
            for sx in 0..self.width {
                let px = sx << 1;
                let c0 = self.pict[row0 + px];
                let c1 = self.pict[row0 + px + 1];
                let c2 = self.pict[row1 + px];
                let c3 = self.pict[row1 + px + 1];

                let r = (((c0 >> 16) & 0xFF)
                    + ((c1 >> 16) & 0xFF)
                    + ((c2 >> 16) & 0xFF)
                    + ((c3 >> 16) & 0xFF))
                    >> 2;
                let g = (((c0 >> 8) & 0xFF)
                    + ((c1 >> 8) & 0xFF)
                    + ((c2 >> 8) & 0xFF)
                    + ((c3 >> 8) & 0xFF))
                    >> 2;
                let b = ((c0 & 0xFF) + (c1 & 0xFF) + (c2 & 0xFF) + (c3 & 0xFF)) >> 2;

                self.screen[sy * self.width + sx] = (r << 16) | (g << 8) | b;
            }
        }
    }
}
