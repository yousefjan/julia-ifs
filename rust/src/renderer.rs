use crate::constants::{BGCOLOR, PALSIZE, WIDTH, HEIGHT, ZDEPTH};
use crate::palette::Palette;
use crate::transforms;

#[derive(Copy, Clone)]
pub enum Mode { Julia2D, IFS2D }

pub fn render(framebuffer: &mut [u32], width: usize, height: usize, palette: &Palette, tick: u32, mode: Mode) {
    let w = width as f32;
    let h = height as f32;
    let aspect = w / h.max(1.0);

    let t = tick as f32 * 0.015;
    let c_x = 0.285 + 0.25 * (t * 0.91).sin();
    let c_y = 0.01 + 0.25 * (t * 1.13).cos();

    match mode {
        Mode::Julia2D => {
            let max_iter: u32 = 256;
            for y in 0..height {
                let ny = (y as f32 / h) * 2.0 - 1.0;
                let zy0 = ny * 1.6;
                for x in 0..width {
                    let nx = (x as f32 / w) * 2.0 - 1.0;
                    let zx0 = nx * 1.6 * aspect;

                    let mut zx = zx0;
                    let mut zy = zy0;
                    let mut i = 0u32;
                    while i < max_iter {
                        let zx2 = zx * zx;
                        let zy2 = zy * zy;
                        if zx2 + zy2 > 4.0 { break; }
                        let two_zx_zy = 2.0 * zx * zy;
                        zx = zx2 - zy2 + c_x;
                        zy = two_zx_zy + c_y;
                        i += 1;
                    }

                    let idx = (((i * 32) as usize) & (PALSIZE - 1)) as usize;
                    framebuffer[y * width + x] = palette.colors[idx];
                }
            }
        }
        Mode::IFS2D => {
            // Simple 2D IFS using set2d/set2d3 alternation
            let mut x = 0.01f32;
            let mut y = 0.0f32;
            for i in 0..(width * height * 10) {
                if (i & 1) == 0 { let r = transforms::set2d(x, y); x = r.0; y = r.1; }
                else { let r = transforms::set2d3(x, y); x = r.0; y = r.1; }

                let sx = ((x * 0.45 + 0.5) * w) as i32;
                let sy = ((y * 0.45 + 0.5) * h) as i32;
                if sx >= 0 && sy >= 0 && (sx as usize) < width && (sy as usize) < height {
                    let idx = ((i as u32 * 7) & (PALSIZE as u32 - 1)) as usize;
                    framebuffer[sy as usize * width + sx as usize] = palette.colors[idx];
                }
            }
        }
    }
}


