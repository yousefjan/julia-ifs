use crate::constants::{BGCOLOR, PALSIZE, WIDTH, HEIGHT, ZDEPTH};
use crate::palette::Palette;
use crate::transforms;
use crate::camera::Camera;
use crate::light::LightCam;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Copy, Clone)]
pub enum Mode { Julia2D, IFS2D, IFS3D }

pub fn render(framebuffer: &mut [u32], width: usize, height: usize, palette: &Palette, tick: u32, mode: Mode, mut zbuffer: Option<&mut [i32]>, camera: Option<&Camera>, light: Option<&LightCam>, mut lightbuf: Option<&mut [i32]>) {
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
        Mode::IFS3D => {
            let zbuf = match zbuffer.as_deref_mut() { Some(z) => z, None => {
                // fallback: plot without z if none provided
                let mut rng = StdRng::seed_from_u64(tick as u64 + 12345);
                let mut x = 0.1f32; let mut y = 0.0f32; let mut z = 0.0f32;
                for i in 0..(width * height * 6) {
                    iterate_point(&mut x, &mut y, &mut z, &mut rng);
                    let Some((sx, sy, _zc)) = camera.and_then(|cam| cam.view_project(x, y, z, width, height, 240.0)) else { continue };
                    if sx>=0 && sy>=0 && (sx as usize) < width && (sy as usize) < height {
                        let idx = ((i as u32 * 11) & (PALSIZE as u32 - 1)) as usize;
                        framebuffer[sy as usize * width + sx as usize] = palette.colors[idx];
                    }
                }
                return;
            }};
            // clear zbuffer to far depth
            for z in zbuf.iter_mut() { *z = ZDEPTH; }
            let mut rng_light = StdRng::seed_from_u64(tick as u64 + 9999);
            let mut rng_view = StdRng::seed_from_u64(tick as u64 + 9999);

            // build light map first if requested
            if let (Some(light_cam), Some(lbuf)) = (light, lightbuf.as_deref_mut()) {
                lbuf.fill(ZDEPTH);
                let mut lx = 0.1f32; let mut ly = 0.0f32; let mut lz = 0.0f32;
                for _ in 0..(width * height * 10) {
                    iterate_point(&mut lx, &mut ly, &mut lz, &mut rng_light);
                    let Some((lsX, lsY, lzc)) = light_cam.project(lx, ly, lz, width, height, 280.0) else { continue };
                    if lsX<0 || lsY<0 || (lsX as usize) >= width || (lsY as usize) >= height { continue; }
                    let ldepth = ((lzc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
                    let lidx = lsY as usize * width + lsX as usize;
                    if ldepth < lbuf[lidx] { lbuf[lidx] = ldepth; }
                }
            }

            let mut x = 0.1f32; let mut y = 0.0f32; let mut z = 0.0f32;
            for i in 0..(width * height * 8) {
                iterate_point(&mut x, &mut y, &mut z, &mut rng_view);
                let Some((sx, sy, zc)) = camera.and_then(|cam| cam.view_project(x, y, z, width, height, 260.0)) else { continue };
                if sx<0 || sy<0 || (sx as usize) >= width || (sy as usize) >= height { continue; }
                let depth = ((zc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
                let idxp = sy as usize * width + sx as usize;
                if depth < zbuf[idxp] {
                    zbuf[idxp] = depth;
                    let idx = ((i as u32 * 13) & (PALSIZE as u32 - 1)) as usize;
                    let base = palette.colors[idx];
                    // shadowing: compare against precomputed light map
                    let lshade = if let (Some(light_cam), Some(lbuf)) = (light, lightbuf.as_deref_mut()) {
                        if let Some((lx, ly, lzc)) = light_cam.project(x, y, z, width, height, 280.0) {
                            if lx>=0 && ly>=0 && (lx as usize) < width && (ly as usize) < height {
                                let lidx = ly as usize * width + lx as usize;
                                let ld = ((lzc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
                                // bias to reduce acne
                                let bias = 16;
                                if ld <= lbuf[lidx] + bias { 1.0 } else { 0.35 }
                            } else { 1.0 }
                        } else { 1.0 }
                    } else { 1.0 };
                    let shade = (((ZDEPTH - depth).max(0) as f32 / ZDEPTH as f32) * lshade).clamp(0.0, 1.0);
                    let r = (((base >> 16) & 0xFF) as f32 * shade) as u32;
                    let g = (((base >> 8) & 0xFF) as f32 * shade) as u32;
                    let b = ((base & 0xFF) as f32 * shade) as u32;
                    framebuffer[idxp] = (r << 16) | (g << 8) | b;
                }
            }
        }
    }
}

#[inline]
fn iterate_point(x: &mut f32, y: &mut f32, z: &mut f32, rng: &mut StdRng) {
    // choose transform
    match rng.gen_range(0..6) {
        0 => { let r = transforms::set3_a(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        1 => { let r = transforms::set3_b(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        2 => { let r = transforms::set3_c(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        3 => { let r = transforms::set3_d(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        4 => { let r = transforms::set3_e(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        _ => { let r = transforms::set3_d3(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
    }
    // symmetry mods similar to MOD8X / 6X
    if rng.gen_bool(0.33) {
        // 8X flips
        let mask = rng.gen_range(0u8..8);
        if (mask & 0x4) != 0 { *x = -*x; }
        if (mask & 0x2) != 0 { *y = -*y; }
        if (mask & 0x1) != 0 { *z = -*z; }
    } else if rng.gen_bool(0.2) {
        // 6X rotation (x,y,z) -> (z,x,y)
        let tx = -*y; *y = *z; *z = *x; *x = tx;
    }
}


