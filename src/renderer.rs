use crate::constants::{BGCOLOR, PALSIZE, WIDTH, HEIGHT, ZDEPTH};
use crate::palette::Palette;
use crate::transforms;
use crate::camera::Camera;
use crate::light::LightCam;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Copy, Clone)]
pub enum Mode { Julia2D, IFS2D, IFS3D }

#[derive(Copy, Clone)]
pub struct IfsControl {
    pub time_scale: f32,        // scales animation speed for c_x/c_y
    pub freeze_sets: bool,      // if true, seeds and set selections are fixed
    pub set2d_variant: Option<i32>, // Some(0)=SET2D, Some(1)=SET2D3, None=random
    pub set3d_index: Option<i32>,   // 0..=5 selects 3D family, None=random
    pub freeze_c: bool,         // if true, c_x/c_y are static for IFS modes
    pub c_x: f32,               // fixed Julia constant x for IFS modes
    pub c_y: f32,               // fixed Julia constant y for IFS modes
}

impl Default for IfsControl {
    fn default() -> Self {
        Self { time_scale: 1.0, freeze_sets: false, set2d_variant: None, set3d_index: None, freeze_c: true, c_x: 0.285, c_y: 0.01 }
    }
}

pub fn render(
    framebuffer: &mut [u32],
    width: usize,
    height: usize,
    palette: &Palette,
    tick: u32,
    mode: Mode,
    mut zbuffer: Option<&mut [i32]>,
    camera: Option<&Camera>,
    light: Option<&LightCam>,
    mut lightbuf: Option<&mut [i32]>,
    samples_view: usize,
    samples_light: usize,
    ifs: &IfsControl,
) {
    let w = width as f32;
    let h = height as f32;
    let aspect = w / h.max(1.0);

    let t = tick as f32 * 0.015 * ifs.time_scale;
    let anim_cx = 0.285 + 0.25 * (t * 0.91).sin();
    let anim_cy = 0.01 + 0.25 * (t * 1.13).cos();
    let (ifs_cx, ifs_cy) = if ifs.freeze_c { (ifs.c_x, ifs.c_y) } else { (anim_cx, anim_cy) };

    match mode {
        Mode::Julia2D => {
            // Adaptive budget: probe complexity, then adjust iterations and stride
            let mut probe_sum = 0u32;
            let probe_iter_max: u32 = 160;
            let probes = 64usize;
            for py in 0..8 {
                let sy = (py * (height.max(1) / 8)).min(height.saturating_sub(1));
                let ny = (sy as f32 / h) * 2.0 - 1.0;
                let zy0 = ny * 1.6;
                for px in 0..8 {
                    let sx = (px * (width.max(1) / 8)).min(width.saturating_sub(1));
                    let nx = (sx as f32 / w) * 2.0 - 1.0;
                    let zx0 = nx * 1.6 * aspect;
                    let mut zx = zx0; let mut zy = zy0; let mut i = 0u32;
                    while i < probe_iter_max {
                        let zx2 = zx * zx; let zy2 = zy * zy;
                        if zx2 + zy2 > 4.0 { break; }
                        let two_zx_zy = 2.0 * zx * zy;
                        zx = zx2 - zy2 + anim_cx; zy = two_zx_zy + anim_cy; i += 1;
                    }
                    probe_sum += i;
                }
            }
            let avg_probe = probe_sum as f32 / probes as f32;
            let heavy = avg_probe > (probe_iter_max as f32 * 0.55);
            let max_iter: u32 = if heavy { 128 } else { 256 };
            let stride: usize = if heavy { 2 } else { 1 };
            let phase = (tick as usize) & 1;
            let y_start = if stride == 1 { 0 } else { phase };
            let x_start = if stride == 1 { 0 } else { phase };

            let mut y = y_start;
            while y < height {
                let ny = (y as f32 / h) * 2.0 - 1.0;
                let zy0 = ny * 1.6;
                let mut x = x_start;
                while x < width {
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
                        zx = zx2 - zy2 + anim_cx;
                        zy = two_zx_zy + anim_cy;
                        i += 1;
                    }

                    let idx = (((i * 32) as usize) & (PALSIZE - 1)) as usize;
                    let col = palette.colors[idx];
                    // write pixel(s)
                    framebuffer[y * width + x] = col;
                    if stride == 2 {
                        if x + 1 < width { framebuffer[y * width + (x + 1)] = col; }
                        if y + 1 < height { framebuffer[(y + 1) * width + x] = col; }
                        if x + 1 < width && y + 1 < height { framebuffer[(y + 1) * width + (x + 1)] = col; }
                    }
                    x += stride;
                }
                y += stride;
            }
        }
        Mode::IFS2D => {
            // Reversed Julia IFS in 2D, mirroring C++ SET2D/SET2D3
            framebuffer.fill(BGCOLOR);
            let seed_tick = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
            let mut rng = StdRng::seed_from_u64(0x2D_2D + seed_tick);
            let mut x = rng.gen_range(-0.5..0.5);
            let mut y = rng.gen_range(-0.5..0.5);
            // choose 2D transform family (SET2D or SET2D3)
            let set_idx_2d = if let Some(s) = ifs.set2d_variant { s } else if rng.gen_bool(0.5) { 0 } else { 1 };
            // animated Julia constant reused from 2D Julia view
            let c2 = (ifs_cx as f32, ifs_cy as f32);
            // burn-in iterations to converge to attractor
            let burn_in = (samples_view / 10).max(300).min(5000);
            for _ in 0..burn_in { iterate_point_2d(&mut x, &mut y, &mut rng, set_idx_2d, c2); }
            // draw iterations
            for i in 0..samples_view {
                iterate_point_2d(&mut x, &mut y, &mut rng, set_idx_2d, c2);
                let sx = (((x * 1.6 * aspect) + 1.0) * 0.5 * w) as i32;
                let sy = (((y * 1.6) + 1.0) * 0.5 * h) as i32;
                if sx >= 0 && sy >= 0 && (sx as usize) < width && (sy as usize) < height {
                    let idx = (((i as u32) * 13) & (PALSIZE as u32 - 1)) as usize;
                    framebuffer[sy as usize * width + sx as usize] = palette.colors[idx];
                }
            }
        }
        Mode::IFS3D => {
            // Clear previous visuals each frame for clean IFS rendering
            framebuffer.fill(BGCOLOR);
            let zbuf = match zbuffer.as_deref_mut() { Some(z) => z, None => {
                // fallback: plot without z if none provided
                let seed_tick = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
                let mut rng = StdRng::seed_from_u64(12345 + seed_tick);
                let mut x = rng.gen_range(-0.5..0.5); let mut y = rng.gen_range(-0.5..0.5); let mut z = rng.gen_range(-0.5..0.5);
                // choose single transform family for this frame
                let set_idx = if let Some(s) = ifs.set3d_index { s } else { rng.gen_range(0..6) };
                let c3 = (ifs_cx as f32, ifs_cy as f32, 0.0f32);
                // burn-in iterations to converge to attractor
                let burn_in = (samples_view / 10).max(300).min(4000);
                for _ in 0..burn_in { iterate_point_fixed(&mut x, &mut y, &mut z, &mut rng, set_idx, c3); }
                for i in 0..(samples_view.max(width * height / 8)) {
                    iterate_point_fixed(&mut x, &mut y, &mut z, &mut rng, set_idx, c3);
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
            let seed_tick = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
            let mut rng_light = StdRng::seed_from_u64(9999 + seed_tick);
            let mut rng_view = StdRng::seed_from_u64(9999 + seed_tick);
            // choose single transform family and julia constant
            let set_idx = if let Some(s) = ifs.set3d_index { s } else { rng_view.gen_range(0..6) };
            let c3 = (ifs_cx as f32, ifs_cy as f32, 0.0f32);

            // build light map first if requested
            if let (Some(light_cam), Some(lbuf)) = (light, lightbuf.as_deref_mut()) {
                lbuf.fill(ZDEPTH);
                let mut lx = rng_light.gen_range(-0.5..0.5); let mut ly = rng_light.gen_range(-0.5..0.5); let mut lz = rng_light.gen_range(-0.5..0.5);
                let burn_in_l = (samples_light / 10).max(200).min(3000);
                for _ in 0..burn_in_l { iterate_point_fixed(&mut lx, &mut ly, &mut lz, &mut rng_light, set_idx, c3); }
                for _ in 0..samples_light {
                    iterate_point_fixed(&mut lx, &mut ly, &mut lz, &mut rng_light, set_idx, c3);
                    let Some((lsX, lsY, lzc)) = light_cam.project(lx, ly, lz, width, height, 280.0) else { continue };
                    if lsX<0 || lsY<0 || (lsX as usize) >= width || (lsY as usize) >= height { continue; }
                    let ldepth = ((lzc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
                    let lidx = lsY as usize * width + lsX as usize;
                    if ldepth < lbuf[lidx] { lbuf[lidx] = ldepth; }
                }
            }

            let mut x = rng_view.gen_range(-0.5..0.5); let mut y = rng_view.gen_range(-0.5..0.5); let mut z = rng_view.gen_range(-0.5..0.5);
            let burn_in_v = (samples_view / 10).max(300).min(4000);
            for _ in 0..burn_in_v { iterate_point_fixed(&mut x, &mut y, &mut z, &mut rng_view, set_idx, c3); }
            for i in 0..samples_view {
                iterate_point_fixed(&mut x, &mut y, &mut z, &mut rng_view, set_idx, c3);
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
fn iterate_point_fixed(x: &mut f32, y: &mut f32, z: &mut f32, rng: &mut StdRng, set_idx: i32, c3: (f32,f32,f32)) {
    // subtract Julia constant each iteration
    *x -= c3.0; *y -= c3.1; *z -= c3.2;
    // apply selected transform family
    match set_idx {
        0 => { let r = transforms::set3_a(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        1 => { let r = transforms::set3_b(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        2 => { let r = transforms::set3_c(*x,*y,*z); *x=r.0; *y=r.1; *z=r.2; }
        3 => { let r = transforms::set3_d(*x,*y,*z, rng.gen()); *x=r.0; *y=r.1; *z=r.2; }
        4 => { let r = transforms::set3_e(*x,*y,*z, rng.gen()); *x=r.0; *y=r.1; *z=r.2; }
        _ => { let r = transforms::set3_d3(*x,*y,*z, rng.gen()); *x=r.0; *y=r.1; *z=r.2; }
    }
    // symmetry: pick either 8X flips or 6X rotation with small probability
    if rng.gen_bool(0.3) {
        let mask = rng.gen_range(0u8..8);
        if (mask & 0x4) != 0 { *x = -*x; }
        if (mask & 0x2) != 0 { *y = -*y; }
        if (mask & 0x1) != 0 { *z = -*z; }
    } else if rng.gen_bool(0.15) {
        let tx = -*y; *y = *z; *z = *x; *x = tx;
    }
}

#[inline]
fn iterate_point_2d(x: &mut f32, y: &mut f32, rng: &mut StdRng, set_idx: i32, c2: (f32,f32)) {
    // subtract Julia constant
    *x -= c2.0; *y -= c2.1;
    // apply selected 2D family
    match set_idx {
        0 => { let r = transforms::set2d(*x, *y); *x = r.0; *y = r.1; }
        _ => { let r = transforms::set2d3(*x, *y); *x = r.0; *y = r.1; }
    }
    // symmetry similar to MOD8X or MOD4X randomly
    if rng.gen_bool(0.4) {
        let mask = rng.gen_range(0u8..4);
        if (mask & 0x2) != 0 { *x = -*x; }
        if (mask & 0x1) != 0 { *y = -*y; }
    } else if rng.gen_bool(0.2) {
        let tx = -*y; *y = *x; *x = tx; // 90 deg rotate equivalent
    }
}


