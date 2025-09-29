use crate::constants::{BHEIGHT, BWIDTH, HEIGHT, LHEIGHT, LWIDTH, PALSIZE, WIDTH, ZDEPTH};

pub struct Framebuffers {
    pub screen: Vec<u32>,
    pub pict: Vec<u32>,
    pub zbuf: Vec<i32>,
    pub light: Vec<i32>,
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
            zbuf: vec![ZDEPTH; pict_w * pict_h],
            light: vec![ZDEPTH; light_w * light_h],
            width,
            height,
        }
    }

    pub fn clear_screen(&mut self, rgb: u32) {
        self.screen.fill(rgb);
    }

    pub fn clear_z(&mut self) { self.zbuf.fill(ZDEPTH); }
}


