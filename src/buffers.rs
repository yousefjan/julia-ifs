use crate::constants::{BHEIGHT, BWIDTH, HEIGHT, LHEIGHT, LWIDTH, WIDTH};

pub struct LightCalibration {
    pub minbright: f32,
    pub maxbright: f32,
}

pub struct Framebuffers {
    pub screen: Vec<u32>,
    pub pict: Vec<u32>,
    pub zbuf: Vec<i32>,
    pub light: Vec<i32>,
    pub lighting: LightCalibration,
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
            width,
            height,
        }
    }

    pub fn clear_screen(&mut self, rgb: u32) {
        self.screen.fill(rgb);
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


