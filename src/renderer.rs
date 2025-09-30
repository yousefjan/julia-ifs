use crate::constants::{BGCOLOR, PALSIZE, WIDTH, HEIGHT, ZDEPTH};
use crate::palette::Palette;
use crate::transforms;
use crate::camera::Camera;
use crate::light::LightCam;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Copy, Clone)]
pub enum Mode { Julia2D, IFS2D, IFS3D }

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
) {
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
            // Reversed Julia IFS in 2D, mirroring C++ SET2D/SET2D3
            framebuffer.fill(BGCOLOR);
            let mut rng = StdRng::seed_from_u64(tick as u64 + 0x2D_2D);
            let mut x = rng.gen_range(-0.5..0.5);
            let mut y = rng.gen_range(-0.5..0.5);
            // choose 2D transform family (SET2D or SET2D3)
            let set_idx_2d = if rng.gen_bool(0.5) { 0 } else { 1 };
            // animated Julia constant reused from 2D Julia view
            let c2 = (c_x as f32, c_y as f32);
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
                let mut rng = StdRng::seed_from_u64(tick as u64 + 12345);
                let mut x = rng.gen_range(-0.5..0.5); let mut y = rng.gen_range(-0.5..0.5); let mut z = rng.gen_range(-0.5..0.5);
                // choose single transform family for this frame
                let set_idx = rng.gen_range(0..6);
                let c3 = (c_x as f32, c_y as f32, 0.0f32);
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
            let mut rng_light = StdRng::seed_from_u64(tick as u64 + 9999);
            let mut rng_view = StdRng::seed_from_u64(tick as u64 + 9999);
            // choose single transform family and julia constant
            let set_idx = rng_view.gen_range(0..6);
            let c3 = (c_x as f32, c_y as f32, 0.0f32);

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


