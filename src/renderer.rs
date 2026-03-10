use crate::buffers::{BottomState, Framebuffers, FractalState, LightCalibration};
use crate::constants::{
    BGCOLORS, BHEIGHT, BOTTOM_COLORS, BWIDTH, FRACTAL_COLORS, LHEIGHT, LWIDTH, PALSIZE, PRESETS,
    ZDEPTH,
};
use crate::palette::Palette;
use crate::transforms;
use crate::camera::Camera;
use crate::light::LightCam;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum Mode { Julia2D, IFS2D, IFS3D }

#[derive(Clone)]
pub struct IfsControl {
    pub time_scale: f32,
    pub freeze_sets: bool,
    pub set2d_variant: Option<i32>,
    pub set3d_index: Option<i32>,
    pub x_mode: i32,
    pub freeze_c: bool,
    pub c_x: f32,
    pub c_y: f32,
    pub c_z: f32,
    pub preset_idx: usize,
    pub use_presets: bool,
    pub secret_ingredient: bool,
    pub secret_square: bool,
    pub secret_extra_coord: bool,
    pub secret_size: f32,
    pub background_mode: usize,
    pub whitershade: i32,
    pub lightness: i32,
}

impl Default for IfsControl {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            freeze_sets: false,
            set2d_variant: None,
            set3d_index: Some(3),
            x_mode: 0,
            freeze_c: true,
            c_x: 0.0, c_y: 0.0, c_z: 0.0,
            preset_idx: 9,
            use_presets: true,
            secret_ingredient: false,
            secret_square: false,
            secret_extra_coord: false,
            secret_size: 1.0,
            background_mode: 0,
            whitershade: 0,
            lightness: 0,
        }
    }
}

const VIEW_SCALE: f32 = 520.0;
const LIGHT_SCALE: f32 = 1040.0;
const EXTRA_SHADOW_OFFSET: f32 = 1.0 / 2500.0;

const FRACTAL_ITERS_PER_FRAME: usize = 512;
const BOTTOM_ITERS_PER_FRAME: usize = 128;

pub fn render(
    palette: &Palette,
    tick: u32,
    mode: Mode,
    buffers: &mut Framebuffers,
    camera: Option<&Camera>,
    light: Option<&LightCam>,
    ifs: &IfsControl,
    needs_reset: bool,
) {
    let width = buffers.width;
    let height = buffers.height;
    let w = width as f32;
    let h = height as f32;
    let aspect = w / h.max(1.0);

    let t = tick as f32 * 0.015 * ifs.time_scale;
    let anim_cx = 0.285 + 0.25 * (t * 0.91).sin();
    let anim_cy = 0.01 + 0.25 * (t * 1.13).cos();

    let (ifs_cx, ifs_cy, ifs_cz) = if ifs.freeze_c {
        if ifs.use_presets {
            if ifs.preset_idx < PRESETS.len() {
                PRESETS[ifs.preset_idx]
            } else {
                (ifs.c_x, ifs.c_y, ifs.c_z)
            }
        } else {
            (ifs.c_x, ifs.c_y, ifs.c_z)
        }
    } else {
        (anim_cx, anim_cy, 0.0)
    };

    match mode {
        Mode::Julia2D => render_julia_2d(
            &mut buffers.screen, width, height, palette,
            ifs_cx, ifs_cy, aspect, tick,
        ),
        Mode::IFS2D => render_ifs_2d(
            &mut buffers.screen, width, height, palette, ifs,
            ifs_cx, ifs_cy, aspect, 512, tick,
        ),
        Mode::IFS3D => {
            let bg_color = BGCOLORS[ifs.background_mode.min(BGCOLORS.len() - 1)];
            if needs_reset {
                buffers.clear_view(bg_color);
                buffers.clear_light();
                buffers.reset_all_state(ifs.lightness);
            }

            let lw = LWIDTH as usize;
            let lh = LHEIGHT as usize;
            let bw = BWIDTH as usize;
            let bh = BHEIGHT as usize;

            // Shadow pass
            if light.is_some() {
                if ifs.background_mode == 0 {
                    render_bottom_shadow(
                        &mut buffers.bottom_shadow, &mut buffers.light,
                        light.unwrap(), lw, lh, ifs, BOTTOM_ITERS_PER_FRAME,
                    );
                }
                render_fractal_shadow(
                    &mut buffers.fractal_shadow, &mut buffers.light,
                    light.unwrap(), lw, lh, ifs, FRACTAL_ITERS_PER_FRAME,
                    palette, ifs_cx, ifs_cy, ifs_cz,
                );
            }

            // View pass
            if let Some(cam) = camera {
                if ifs.background_mode == 0 {
                    render_bottom_view(
                        &mut buffers.bottom_view,
                        &mut buffers.pict, &mut buffers.zbuf,
                        &buffers.light,
                        cam, light, bw, bh, ifs,
                        &mut buffers.lighting,
                        BOTTOM_ITERS_PER_FRAME,
                    );
                }
                render_fractal_view(
                    &mut buffers.fractal_view,
                    &mut buffers.pict, &mut buffers.zbuf,
                    &buffers.light,
                    cam, light, bw, bh, ifs,
                    &mut buffers.lighting,
                    FRACTAL_ITERS_PER_FRAME,
                    palette, ifs_cx, ifs_cy, ifs_cz,
                );
            }

            buffers.resolve_2x2_to_screen();
        }
    }
}

fn render_julia_2d(fb: &mut [u32], width: usize, height: usize, palette: &Palette, cx: f32, cy: f32, aspect: f32, _tick: u32) {
    let w = width as f32;
    let h = height as f32;
    let max_iter = 256;
    for y in 0..height {
        let ny = (y as f32 / h) * 2.0 - 1.0;
        let zy0 = ny * 1.6;
        for x in 0..width {
            let nx = (x as f32 / w) * 2.0 - 1.0;
            let zx0 = nx * 1.6 * aspect;
            let mut zx = zx0;
            let mut zy = zy0;
            let mut i = 0;
            while i < max_iter {
                let zx2 = zx * zx;
                let zy2 = zy * zy;
                if zx2 + zy2 > 4.0 { break; }
                let two_zx_zy = 2.0 * zx * zy;
                zx = zx2 - zy2 + cx;
                zy = two_zx_zy + cy;
                i += 1;
            }
            let idx = ((i * 32) as usize) & (PALSIZE - 1);
            fb[y * width + x] = palette.colors[idx];
        }
    }
}

fn render_ifs_2d(fb: &mut [u32], width: usize, height: usize, palette: &Palette, ifs: &IfsControl, cx: f32, cy: f32, aspect: f32, samples: usize, _tick: u32) {
    use crate::constants::BGCOLOR;
    fb.fill(BGCOLOR);
    let mut rng = rand::thread_rng();
    let mut x: f32 = rng.gen_range(-0.5..0.5);
    let mut y: f32 = rng.gen_range(-0.5..0.5);
    let set_idx = ifs.set2d_variant.unwrap_or_else(|| if rng.gen_bool(0.5) { 0 } else { 1 });
    for i in 0..samples {
        x -= cx; y -= cy;
        match set_idx {
            0 => { let r = transforms::set2d(x, y); x = r.0; y = r.1; }
            _ => { let r = transforms::set2d3(x, y); x = r.0; y = r.1; }
        }
        if rng.gen_bool(0.5) { x = -x; }
        if rng.gen_bool(0.5) { y = -y; }
        if i > 20 {
            let sx = (((x * 1.6 * aspect) + 1.0) * 0.5 * width as f32) as i32;
            let sy = (((y * 1.6) + 1.0) * 0.5 * height as f32) as i32;
            if sx >= 0 && sy >= 0 && (sx as usize) < width && (sy as usize) < height {
                let idx = ((i as u32 * 13) & (PALSIZE as u32 - 1)) as usize;
                fb[sy as usize * width + sx as usize] = palette.colors[idx];
            }
        }
    }
}

// ── Bottom plane: shadow pass ───────────────────────────────────────────
fn render_bottom_shadow(
    s: &mut BottomState, light_zbuf: &mut [i32],
    light_cam: &LightCam, lw: usize, lh: usize,
    ifs: &IfsControl, iters: usize,
) {
    let vectors = transforms::get_bottom_vectors(ifs.set3d_index.unwrap_or(0));
    for _ in 0..iters {
        bottom_step(s, &vectors);
        if let Some((lx, ly, lzc)) = light_cam.project(s.x, s.y, s.z, lw, lh, LIGHT_SCALE) {
            plot_z(lx, ly, lzc, light_zbuf, lw, lh);
        }
    }
}

// ── Bottom plane: view pass ─────────────────────────────────────────────
fn render_bottom_view(
    s: &mut BottomState,
    pict: &mut [u32], zbuf: &mut [i32],
    light_zbuf: &[i32],
    cam: &Camera, light: Option<&LightCam>,
    bw: usize, bh: usize,
    ifs: &IfsControl,
    lighting: &mut LightCalibration,
    iters: usize,
) {
    let vectors = transforms::get_bottom_vectors(ifs.set3d_index.unwrap_or(0));
    for _ in 0..iters {
        bottom_step(s, &vectors);
        if let Some((sx, sy, zc)) = cam.view_project(s.x, s.y, s.z, bw, bh, VIEW_SCALE) {
            let (bright, luma, overexpose) =
                calculate_lighting(s.x, s.y, s.z, light, Some(light_zbuf), lighting, s.bglow, true);
            let (fr, fg, fb) = apply_color_mode(
                s.bcr, s.bcg, s.bcb, bright, luma, overexpose, ifs.whitershade,
            );
            let col = (fr << 16) | (fg << 8) | fb;
            plot_z_col(sx, sy, zc, pict, zbuf, bw, bh, col);
        }
    }
}

#[inline]
fn bottom_step(s: &mut BottomState, vectors: &[(f32, f32, f32); 4]) {
    let bi = s.rng.gen_range(0..4);
    let tx = vectors[bi];
    s.x = (s.x - tx.0) * 0.5;
    s.y = (s.y - tx.1) * 0.5;
    s.z = (s.z - tx.2) * 0.5;

    let dist = (s.x * s.x + s.y * s.y + s.z * s.z).sqrt();
    if dist > s.blargel { s.blargel = dist; }
    let t_val = (1.0 - dist / s.blargel).powf(16.0);
    s.bglow = (s.bglow + t_val) / 2.0;

    s.x += tx.0;
    s.y += tx.1;
    s.z += tx.2;

    let tc = BOTTOM_COLORS[bi];
    s.bcr = ((s.bcr + tc.0 as u32) >> 1) & 0xFF;
    s.bcg = ((s.bcg + tc.1 as u32) >> 1) & 0xFF;
    s.bcb = ((s.bcb + tc.2 as u32) >> 1) & 0xFF;
}

// ── Fractal: shadow pass ────────────────────────────────────────────────
fn render_fractal_shadow(
    s: &mut FractalState, light_zbuf: &mut [i32],
    light_cam: &LightCam, lw: usize, lh: usize,
    ifs: &IfsControl, iters: usize,
    palette: &Palette,
    cx: f32, cy: f32, cz: f32,
) {
    for _ in 0..iters {
        fractal_step(s, ifs, palette, cx, cy, cz);
        if let Some((lx, ly, lzc)) = light_cam.project(s.x, s.y, s.z, lw, lh, LIGHT_SCALE) {
            plot_z(lx, ly, lzc, light_zbuf, lw, lh);
        }
    }
}

// ── Fractal: view pass ──────────────────────────────────────────────────
fn render_fractal_view(
    s: &mut FractalState,
    pict: &mut [u32], zbuf: &mut [i32],
    light_zbuf: &[i32],
    cam: &Camera, light: Option<&LightCam>,
    bw: usize, bh: usize,
    ifs: &IfsControl,
    lighting: &mut LightCalibration,
    iters: usize,
    palette: &Palette,
    cx: f32, cy: f32, cz: f32,
) {
    for _ in 0..iters {
        fractal_step(s, ifs, palette, cx, cy, cz);
        let dcr = s.dcr as u32;
        let dcg = s.dcg as u32;
        let dcb = s.dcb as u32;

        // Shadow pixel behind main point
        if let Some((sx2, sy2, sz2)) = backstep_from_light(s.x, s.y, s.z, light) {
            plot_lit(cam, light, Some(light_zbuf), lighting, ifs,
                pict, zbuf, bw, bh,
                sx2, sy2, sz2, dcr, dcg, dcb, 1.0, false, 0.5);
        }
        // Main pixel
        plot_lit(cam, light, Some(light_zbuf), lighting, ifs,
            pict, zbuf, bw, bh,
            s.x, s.y, s.z, dcr, dcg, dcb, 1.0, false, 1.0);

        // Second root for x-modes 0, 1, 2
        if ifs.x_mode <= 2 {
            let (mr, mg, mb) = next_root_color(
                s.dcr, s.dcg, s.dcb, s.pali2,
                s.last_pmodi, s.last_coli, palette,
            );
            let (mx, my, mz) = (-s.x, -s.y, -s.z);

            if let Some((sx2, sy2, sz2)) = backstep_from_light(mx, my, mz, light) {
                plot_lit(cam, light, Some(light_zbuf), lighting, ifs,
                    pict, zbuf, bw, bh,
                    sx2, sy2, sz2, mr, mg, mb, 1.0, false, 0.5);
            }
            plot_lit(cam, light, Some(light_zbuf), lighting, ifs,
                pict, zbuf, bw, bh,
                mx, my, mz, mr, mg, mb, 1.0, false, 1.0);
        }
    }
}

fn fractal_step(s: &mut FractalState, ifs: &IfsControl, palette: &Palette, cx: f32, cy: f32, cz: f32) {
    let set_idx = ifs.set3d_index.unwrap_or(0);

    s.x -= cx;
    s.y -= cy;
    s.z -= cz;

    if ifs.secret_ingredient {
        if ifs.secret_extra_coord && s.rng.gen_bool(0.5) {
            let r3 = s.rng.gen_range(0..3);
            let sv = ifs.secret_size * if s.rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            match r3 { 0 => s.x += sv, 1 => s.y += sv, _ => s.z += sv }
        }
        if ifs.secret_square {
            let t = s.y;
            s.y = -s.z;
            s.z = -t;
        }
    }

    let randu = s.rng.gen::<f32>();
    match set_idx {
        0 => { let r = transforms::set3_a(s.x, s.y, s.z); s.x=r.0; s.y=r.1; s.z=r.2; }
        1 => { let r = transforms::set3_b(s.x, s.y, s.z); s.x=r.0; s.y=r.1; s.z=r.2; }
        2 => { let r = transforms::set3_c(s.x, s.y, s.z); s.x=r.0; s.y=r.1; s.z=r.2; }
        3 => { let r = transforms::set3_d(s.x, s.y, s.z, randu); s.x=r.0; s.y=r.1; s.z=r.2; }
        4 => { let r = transforms::set3_e(s.x, s.y, s.z, randu); s.x=r.0; s.y=r.1; s.z=r.2; }
        5 => { let r = transforms::set3_d3(s.x, s.y, s.z, randu); s.x=r.0; s.y=r.1; s.z=r.2; }
        6 => { let r = transforms::set2d(s.x, s.y); s.x=r.0; s.y=r.1; s.z=0.0; }
        _ => { let r = transforms::set2d3(s.x, s.y); s.x=r.0; s.y=r.1; s.z=0.0; }
    }

    s.repti -= 1;
    if s.repti <= 0 {
        s.maxrepti = (s.rng.gen::<f32>() * s.rng.gen::<f32>() * 128.0) as i32;
        s.repti = s.maxrepti;
        s.sduoi = s.rng.gen_bool(0.5);
        s.smulti = s.rng.gen_range(0..8);
        s.probability = s.rng.gen::<f32>();
    }
    let mut duoi = s.sduoi;
    let mut palupflag = false;

    if s.indxn < 0 {
        duoi = s.rng.gen::<f32>() < s.probability;
    } else {
        if s.indxn == 0 {
            s.indxs += 1;
            s.indxuse = s.indxs;
            s.indxn = 24;
        }
        if s.useswap {
            s.swapflag = !s.swapflag;
            duoi = if s.swapflag { (s.indxuse & 1) == 0 } else { (s.indxuse & 1) != 0 };
        } else {
            duoi = (s.indxuse & 1) != 0;
        }
        s.indxuse >>= 1;
        s.indxn -= 1;
    }

    let (pmodi, coli_idx, p_flag) = match ifs.x_mode {
        1 => transforms::mod3x(&mut s.x, &mut s.y, &mut s.z, s.rng.gen_range(0..3)),
        2 => transforms::mod4x(&mut s.x, &mut s.y, &mut s.z, duoi),
        3 => transforms::mod6x(&mut s.x, &mut s.y, &mut s.z, duoi),
        4 => transforms::mod6xx(&mut s.x, &mut s.y, &mut s.z, s.smulti),
        5 => transforms::mod8x(&mut s.x, &mut s.y, &mut s.z, s.smulti),
        6 => transforms::mod2x6x(&mut s.x, &mut s.y, &mut s.z, duoi, s.rng.gen_bool(0.5)),
        _ => transforms::mod2x(&mut s.x, &mut s.y, &mut s.z, duoi),
    };
    if p_flag { palupflag = true; }

    if palupflag {
        s.pali = s.pali.wrapping_add((PALSIZE - s.pali) >> 2);
        s.pali2 = s.pali.wrapping_sub(s.pali >> 2);
    } else {
        s.pali = s.pali.wrapping_sub(s.pali >> 2);
        s.pali2 = s.pali2.wrapping_add((PALSIZE - s.pali) >> 2);
    }

    if pmodi == 1 {
        let c = FRACTAL_COLORS[coli_idx % 8];
        s.dcr = ((s.dcr as u32 + c.0 as u32) >> 1) as u8;
        s.dcg = ((s.dcg as u32 + c.1 as u32) >> 1) as u8;
        s.dcb = ((s.dcb as u32 + c.2 as u32) >> 1) as u8;
    } else {
        let idx = s.pali & (PALSIZE - 1);
        let tcolor = palette.colors[idx];
        let tr = ((tcolor >> 16) & 0xFF) as u32;
        let tg = ((tcolor >> 8) & 0xFF) as u32;
        let tb = (tcolor & 0xFF) as u32;
        s.dcr = ((s.dcr as u32 + tr * 3) >> 2) as u8;
        s.dcg = ((s.dcg as u32 + tg * 3) >> 2) as u8;
        s.dcb = ((s.dcb as u32 + tb * 3) >> 2) as u8;
    }

    s.last_pmodi = pmodi;
    s.last_coli = coli_idx;
}

// ── Plot helpers ────────────────────────────────────────────────────────
#[inline]
fn plot_lit(
    cam: &Camera, light: Option<&LightCam>, lbuf: Option<&[i32]>,
    lighting: &mut LightCalibration, ifs: &IfsControl,
    pict: &mut [u32], zbuf: &mut [i32], w: usize, h: usize,
    x: f32, y: f32, z: f32,
    r: u32, g: u32, b: u32,
    glow: f32, use_glow: bool, brightness_scale: f32,
) {
    if let Some((sx, sy, zc)) = cam.view_project(x, y, z, w, h, VIEW_SCALE) {
        let (mut bright, mut luma, mut overexpose) =
            calculate_lighting(x, y, z, light, lbuf, lighting, glow, use_glow);
        if brightness_scale != 1.0 {
            bright = ((bright as f32) * brightness_scale).min(255.0) as u32;
            luma = 1.0;
            overexpose = 0;
        }
        let (fr, fg, fb) = apply_color_mode(r, g, b, bright, luma, overexpose, ifs.whitershade);
        let col = (fr << 16) | (fg << 8) | fb;
        plot_z_col(sx, sy, zc, pict, zbuf, w, h, col);
    }
}

#[inline]
fn projected_depth(zc: f32) -> i32 {
    if zc <= 0.0 { return 0; }
    ((1.0 / zc) * ZDEPTH as f32).clamp(0.0, (ZDEPTH - 1) as f32) as i32
}

#[inline]
fn backstep_from_light(x: f32, y: f32, z: f32, light: Option<&LightCam>) -> Option<(f32, f32, f32)> {
    let lc = light?;
    let (lx, ly, lz) = lc.rotate_point(x, y, z);
    Some(lc.unrotate_point(lx, ly, lz + EXTRA_SHADOW_OFFSET))
}

#[inline]
fn next_root_color(dcr: u8, dcg: u8, dcb: u8, pali2: usize, pmodi: u32, coli_idx: usize, palette: &Palette) -> (u32, u32, u32) {
    if pmodi == 1 {
        let c = FRACTAL_COLORS[coli_idx % 8];
        (
            ((dcr as u32 + c.0 as u32) >> 1) & 0xFF,
            ((dcg as u32 + c.1 as u32) >> 1) & 0xFF,
            ((dcb as u32 + c.2 as u32) >> 1) & 0xFF,
        )
    } else {
        let idx = pali2 & (PALSIZE - 1);
        let tcolor = palette.colors[idx];
        let tr = (tcolor >> 16) & 0xFF;
        let tg = (tcolor >> 8) & 0xFF;
        let tb = tcolor & 0xFF;
        (
            ((dcr as u32 + tr * 3) >> 2) & 0xFF,
            ((dcg as u32 + tg * 3) >> 2) & 0xFF,
            ((dcb as u32 + tb * 3) >> 2) & 0xFF,
        )
    }
}

#[inline]
fn calculate_lighting(
    x: f32, y: f32, z: f32,
    light: Option<&LightCam>, lbuf: Option<&[i32]>,
    lighting: &mut LightCalibration,
    glow: f32, use_glow: bool,
) -> (u32, f32, u32) {
    let mut bright = 0u32;
    let mut luma = 1.0f32;
    let mut overexpose = 0u32;

    if let Some(l_cam) = light {
        let (_, _, light_z) = l_cam.rotate_point(x, y, z);
        if let Some((lx, ly, lzc)) =
            l_cam.project(x, y, z, LWIDTH as usize, LHEIGHT as usize, LIGHT_SCALE)
        {
            let size = (3.0 + light_z) / 2.0;
            let mut t = 2.0 - size;
            t = (1.0 + t) / 2.0;
            if t < lighting.minbright { lighting.minbright = t; }
            t -= lighting.minbright;
            if t > lighting.maxbright { lighting.maxbright = t; }
            t /= lighting.maxbright.max(0.0001);

            t *= 2.0;
            luma = (t - 1.0).max(0.0);
            luma = 1.0 + luma.powf(8.0);
            if t > 1.0 { t = 1.0; }
            overexpose = ((255.0 * luma) as i32 - 0xFF).max(0) as u32;

            if let Some(lb) = lbuf {
                if lx >= 0 && ly >= 0
                    && (lx as usize) < LWIDTH as usize
                    && (ly as usize) < LHEIGHT as usize
                {
                    let lidx = ly as usize * LWIDTH as usize + lx as usize;
                    let ld = projected_depth(lzc);
                    if lb[lidx] > ld + 32 {
                        bright = if use_glow {
                            ((48.0 * glow + 208.0 * t) as u32).min(255)
                        } else {
                            ((255.0 * t) as u32).min(255)
                        };
                        bright >>= 1;
                        return (bright, 1.0, 0);
                    }
                }
            }

            bright = if use_glow {
                ((48.0 * glow + 208.0 * t) as u32).min(255)
            } else {
                ((255.0 * t) as u32).min(255)
            };
        }
    }

    if bright == 0 {
        bright = if use_glow {
            (48.0 * glow + 128.0).clamp(32.0, 255.0) as u32
        } else { 192 };
        luma = 1.0;
        overexpose = 0;
    }
    (bright, luma, overexpose)
}

#[inline]
fn apply_color_mode(r: u32, g: u32, b: u32, bright: u32, luma: f32, overexpose: u32, mode: i32) -> (u32, u32, u32) {
    let mut cr = r;
    let mut cg = g;
    let mut cb = b;
    if overexpose != 0 {
        cr = (((cr + overexpose) as f32) / luma).min(255.0) as u32;
        cg = (((cg + overexpose) as f32) / luma).min(255.0) as u32;
        cb = (((cb + overexpose) as f32) / luma).min(255.0) as u32;
    }
    if mode != 0 {
        if mode == 1 {
            cr = ((cr * 3) + bright) >> 2;
            cg = ((cg << 1) + bright) / 3;
            cb = (cb + bright) >> 1;
        } else {
            cr = (cr + bright) >> 1;
            cg = ((cg << 1) + bright) / 3;
            cb = ((cb * 3) + bright) >> 2;
        }
    }
    cr = (cr * bright) >> 8;
    cg = (cg * bright) >> 8;
    cb = (cb * bright) >> 8;
    (cr.min(255), cg.min(255), cb.min(255))
}

#[inline]
fn plot_z(x: i32, y: i32, zc: f32, zbuf: &mut [i32], w: usize, h: usize) {
    if x < 0 || y < 0 || (x as usize) >= w || (y as usize) >= h { return; }
    let depth = projected_depth(zc);
    let idx = y as usize * w + x as usize;
    if depth > zbuf[idx] { zbuf[idx] = depth; }
}

#[inline]
fn plot_z_col(x: i32, y: i32, zc: f32, pict: &mut [u32], zbuf: &mut [i32], w: usize, h: usize, col: u32) {
    if x < 0 || y < 0 || (x as usize) >= w || (y as usize) >= h { return; }
    let depth = projected_depth(zc);
    let idx = y as usize * w + x as usize;
    if depth > zbuf[idx] {
        zbuf[idx] = depth;
        pict[idx] = col;
    }
}
