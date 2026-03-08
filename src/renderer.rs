use crate::buffers::{Framebuffers, LightCalibration};
use crate::constants::{
    BGCOLOR, BGCOLORS, BHEIGHT, BOTTOM_COLORS, BWIDTH, FRACTAL_COLORS, LHEIGHT, LWIDTH, PALSIZE, PRESETS,
    ZDEPTH,
};
use crate::palette::Palette;
use crate::transforms;
use crate::camera::Camera;
use crate::light::LightCam;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Copy, Clone, PartialEq)]
pub enum Mode { Julia2D, IFS2D, IFS3D }

#[derive(Copy, Clone)]
pub struct IfsControl {
    pub time_scale: f32,
    pub freeze_sets: bool,
    pub set2d_variant: Option<i32>,
    pub set3d_index: Option<i32>,   // 0..=7 (A, B, C, D, E, D3, 2D, 2D3)
    pub x_mode: i32, // 0=2X, 1=3X, 2=4X, 3=6X, 4=6XX, 5=8X, 6=2X6X
    pub freeze_c: bool,
    pub c_x: f32,
    pub c_y: f32,
    pub c_z: f32,
    pub preset_idx: usize, // 0..9
    pub use_presets: bool,
    pub secret_ingredient: bool,
    pub secret_square: bool,
    pub secret_extra_coord: bool,
    pub secret_size: f32,
    pub background_mode: usize,
    pub whitershade: i32, // 0 = Normal, 1 = Fluorescent, 2 = Filament
    pub lightness: i32, // 0 = Dark, 1 = Light
    pub orbit_x: [f32; 10],
    pub orbit_y: [f32; 10],
    pub orbit_z: [f32; 10],
}

impl Default for IfsControl {
    fn default() -> Self {
        Self {
            time_scale: 1.0,
            freeze_sets: false,
            set2d_variant: None,
            set3d_index: Some(3), // Default Set D
            x_mode: 0, // Default 2X
            freeze_c: true,
            c_x: 0.0, c_y: 0.0, c_z: 0.0,
            preset_idx: 9, // Random
            use_presets: true,
            secret_ingredient: false,
            secret_square: false,
            secret_extra_coord: false,
            secret_size: 1.0,
            background_mode: 0,
            whitershade: 0,
            lightness: 0,
            orbit_x: [1.0; 10],
            orbit_y: [1.0; 10],
            orbit_z: [1.0; 10],
        }
    }
}

struct RenderState {
    x: f32, y: f32, z: f32,
    dcr: u8, dcg: u8, dcb: u8, // Current RGB color
    pali: usize, // Palette index
    pali2: usize, // Second palette index
    palupflag: bool,
}

const VIEW_SCALE: f32 = 520.0;
const LIGHT_SCALE: f32 = 1040.0;
const EXTRA_SHADOW_OFFSET: f32 = 1.0 / 2500.0;

pub fn render(
    palette: &Palette,
    tick: u32,
    mode: Mode,
    buffers: &mut Framebuffers,
    camera: Option<&Camera>,
    light: Option<&LightCam>,
    samples_view: usize,
    samples_light: usize,
    ifs: &mut IfsControl,
    clear_view_buffer: bool,
    clear_shadow_map: bool,
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
            &mut buffers.screen,
            width,
            height,
            palette,
            ifs_cx,
            ifs_cy,
            aspect,
            tick,
        ),
        Mode::IFS2D => render_ifs_2d(
            &mut buffers.screen,
            width,
            height,
            palette,
            ifs,
            ifs_cx,
            ifs_cy,
            aspect,
            samples_view,
            tick,
        ),
        Mode::IFS3D => {
            let bg_color = BGCOLORS[ifs.background_mode.min(BGCOLORS.len() - 1)];
            if clear_view_buffer {
                buffers.clear_view(bg_color);
            }

            if clear_shadow_map {
                buffers.clear_light();
            }
            if clear_shadow_map || clear_view_buffer {
                buffers.reset_lighting(ifs.lightness);
            }

            let seed_tick = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
            let mut shadow_lighting = LightCalibration {
                minbright: buffers.lighting.minbright,
                maxbright: buffers.lighting.maxbright,
            };
            let orbit_slot = if ifs.use_presets {
                ifs.preset_idx.min(9)
            } else {
                9
            };
            let orbit_start = (
                ifs.orbit_x[orbit_slot],
                ifs.orbit_y[orbit_slot],
                ifs.orbit_z[orbit_slot],
            );
            
            // 1. Shadow Map Pass
            let mut shadow_rng = StdRng::seed_from_u64(0x5AD0_5AD0 + seed_tick);
            if let Some(light_cam) = light {
                // Bottom Plane Shadows
                if ifs.background_mode == 0 {
                    render_bottom_layer(
                        None,
                        Some(&mut buffers.light),
                        None,
                        None,
                        Some(light_cam),
                        LWIDTH as usize,
                        LHEIGHT as usize,
                        ifs,
                        &mut shadow_lighting,
                        samples_light / 4,
                        &mut shadow_rng,
                        true,
                    );
                }
                // Fractal Shadows
                let _ = render_fractal_layer(
                    None,
                    Some(&mut buffers.light),
                    None,
                    None,
                    Some(light_cam),
                    LWIDTH as usize,
                    LHEIGHT as usize,
                    ifs,
                    &mut shadow_lighting,
                    samples_light,
                    &mut shadow_rng,
                    true,
                    palette,
                    orbit_start,
                    ifs_cx,
                    ifs_cy,
                    ifs_cz,
                );
            }

            // 2. View Pass
            let mut view_rng = StdRng::seed_from_u64(0xF00D_F00D + seed_tick);
            if let Some(cam) = camera {
                // Bottom Plane View
                if ifs.background_mode == 0 {
                    render_bottom_layer(
                        Some(&mut buffers.pict),
                        Some(&mut buffers.zbuf),
                        Some(cam),
                        Some(&buffers.light),
                        light,
                        BWIDTH as usize,
                        BHEIGHT as usize,
                        ifs,
                        &mut buffers.lighting,
                        samples_view / 4,
                        &mut view_rng,
                        false,
                    );
                }
                // Fractal View
                let orbit_end = render_fractal_layer(
                    Some(&mut buffers.pict),
                    Some(&mut buffers.zbuf),
                    Some(cam),
                    Some(&buffers.light),
                    light,
                    BWIDTH as usize,
                    BHEIGHT as usize,
                    ifs,
                    &mut buffers.lighting,
                    samples_view,
                    &mut view_rng,
                    false,
                    palette,
                    orbit_start,
                    ifs_cx,
                    ifs_cy,
                    ifs_cz,
                );
                ifs.orbit_x[orbit_slot] = orbit_end.0;
                ifs.orbit_y[orbit_slot] = orbit_end.1;
                ifs.orbit_z[orbit_slot] = orbit_end.2;
            }

            buffers.resolve_2x2_to_screen();
        }
    }
}

fn render_julia_2d(fb: &mut [u32], width: usize, height: usize, palette: &Palette, cx: f32, cy: f32, aspect: f32, _tick: u32) {
    // ... existing Julia2D implementation ...
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

fn render_ifs_2d(fb: &mut [u32], width: usize, height: usize, palette: &Palette, ifs: &IfsControl, cx: f32, cy: f32, aspect: f32, samples: usize, tick: u32) {
    fb.fill(BGCOLOR);
    let seed = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
    let mut rng = StdRng::seed_from_u64(0x2D_2D + seed);
            let mut x = rng.gen_range(-0.5..0.5);
            let mut y = rng.gen_range(-0.5..0.5);
    
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

fn render_bottom_layer(
    mut fb: Option<&mut [u32]>,
    mut zbuf: Option<&mut [i32]>,
    cam: Option<&Camera>,
    lbuf: Option<&[i32]>, 
    light: Option<&LightCam>,
    width: usize, height: usize,
    ifs: &IfsControl,
    lighting: &mut LightCalibration,
    samples: usize,
    rng: &mut StdRng,
    is_shadow_pass: bool
) {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut z = 0.0f32;
    
    // Glow State
    let mut bglow = 1.0f32;
    let mut blargel = 0.0001f32;
    
    // Colors
    let mut bcr = BOTTOM_COLORS[0].0 as u32;
    let mut bcg = BOTTOM_COLORS[0].1 as u32;
    let mut bcb = BOTTOM_COLORS[0].2 as u32;

    let set_idx = ifs.set3d_index.unwrap_or(0);
    let vectors = transforms::get_bottom_vectors(set_idx);
    
    for i in 0..samples {
        let bi = rng.gen_range(0..4);
        let tx = vectors[bi];
        
        x = (x - tx.0) * 0.5;
        y = (y - tx.1) * 0.5;
        z = (z - tx.2) * 0.5;
        
        // Attractor Glow Logic
        let dist = (x*x + y*y + z*z).sqrt();
        if dist > blargel {
            blargel = dist;
        }
        let t_val = (1.0 - dist / blargel).powf(16.0);
        bglow = (bglow + t_val) / 2.0;
        
        x += tx.0;
        y += tx.1;
        z += tx.2;
        
        let tc = BOTTOM_COLORS[bi];
        bcr = ((bcr + tc.0 as u32) >> 1) & 0xFF;
        bcg = ((bcg + tc.1 as u32) >> 1) & 0xFF;
        bcb = ((bcb + tc.2 as u32) >> 1) & 0xFF;
        
        if i < 20 { continue; }
        
        if is_shadow_pass {
            if let Some(l_cam) = light {
                 if let Some((lx, ly, lzc)) = l_cam.project(x, y, z, width, height, LIGHT_SCALE) {
                     plot_z(lx, ly, lzc, None::<&mut [u32]>, zbuf.as_deref_mut(), width, height, 0);
                 }
            }
        } else {
            if let Some(c) = cam {
                if let Some((sx, sy, zc)) = c.view_project(x, y, z, width, height, VIEW_SCALE) {
                    let (bright, luma, overexpose) =
                        calculate_lighting(x, y, z, light, lbuf, lighting, bglow, true);
                    
                    // Apply Color & Whitershade
                    let (final_r, final_g, final_b) = apply_color_mode(
                        bcr, bcg, bcb,
                        bright, luma, overexpose,
                        ifs.whitershade
                    );
                    
                    let final_col = (final_r << 16) | (final_g << 8) | final_b;
                    plot_z(sx, sy, zc, fb.as_deref_mut(), zbuf.as_deref_mut(), width, height, final_col);
                }
            }
        }
    }
}

fn render_fractal_layer(
    mut fb: Option<&mut [u32]>,
    mut zbuf: Option<&mut [i32]>,
    cam: Option<&Camera>,
    lbuf: Option<&[i32]>,
    light: Option<&LightCam>,
    width: usize, height: usize,
    ifs: &IfsControl,
    lighting: &mut LightCalibration,
    samples: usize,
    rng: &mut StdRng,
    is_shadow_pass: bool,
    palette: &Palette,
    start: (f32, f32, f32),
    cx: f32, cy: f32, cz: f32
) -> (f32, f32, f32) {
    let mut state = RenderState {
        x: start.0,
        y: start.1,
        z: start.2,
        dcr: FRACTAL_COLORS[0].0, dcg: FRACTAL_COLORS[0].1, dcb: FRACTAL_COLORS[0].2,
        pali: 0, pali2: 0, palupflag: false,
    };

    let set_idx = ifs.set3d_index.unwrap_or(0);
    
    let mut maxrepti = (rng.gen::<f32>() * rng.gen::<f32>() * 128.0) as i32;
    let mut repti = -1;
    let mut duoi;
    let mut sduoi = false;
    let mut smulti = 0usize;
    let mut indxn = if rng.gen_bool(0.5) { -1 } else { 0 };
    let mut indxuse = 0;
    let mut indxs = 0;
    let mut swapflag = false;
    let mut probability = 0.0;
    let useswap = rng.gen_bool(0.5);

    for i in 0..samples {
        state.x -= cx;
        state.y -= cy;
        state.z -= cz;

        if ifs.secret_ingredient {
             if ifs.secret_extra_coord && (rng.gen_bool(0.5)) {
                 let r3 = rng.gen_range(0..3);
                 let s = ifs.secret_size * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
                 if r3 == 0 { state.x += s; }
                 else if r3 == 1 { state.y += s; }
                 else { state.z += s; }
             }
             if ifs.secret_square {
                 let t = state.y;
                 state.y = -state.z;
                 state.z = -t;
             }
        }

        let randu = rng.gen::<f32>();
        match set_idx {
            0 => { let r = transforms::set3_a(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            1 => { let r = transforms::set3_b(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            2 => { let r = transforms::set3_c(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            3 => { let r = transforms::set3_d(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            4 => { let r = transforms::set3_e(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            5 => { let r = transforms::set3_d3(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            6 => { let r = transforms::set2d(state.x, state.y); state.x=r.0; state.y=r.1; state.z=0.0; }
            _ => { let r = transforms::set2d3(state.x, state.y); state.x=r.0; state.y=r.1; state.z=0.0; }
        }

        repti -= 1;
        if repti <= 0 {
            maxrepti = (rng.gen::<f32>() * rng.gen::<f32>() * 128.0) as i32;
            repti = maxrepti;
            sduoi = rng.gen_bool(0.5);
            smulti = rng.gen_range(0..8);
            probability = rng.gen::<f32>();
        }
        duoi = sduoi;
        state.palupflag = false;

        if indxn < 0 {
            duoi = rng.gen::<f32>() < probability;
        } else {
            if indxn == 0 {
                indxs += 1;
                indxuse = indxs;
                indxn = 24;
            }
            if useswap {
                swapflag = !swapflag;
                duoi = if swapflag { (indxuse & 1) == 0 } else { (indxuse & 1) != 0 };
            } else {
                duoi = (indxuse & 1) != 0;
            }
            indxuse >>= 1;
            indxn -= 1;
        }

        let (pmodi, coli_idx, p_flag) = match ifs.x_mode {
            1 => transforms::mod3x(&mut state.x, &mut state.y, &mut state.z, rng.gen_range(0..3)),
            2 => transforms::mod4x(&mut state.x, &mut state.y, &mut state.z, duoi),
            3 => transforms::mod6x(&mut state.x, &mut state.y, &mut state.z, duoi),
            4 => transforms::mod6xx(&mut state.x, &mut state.y, &mut state.z, smulti),
            5 => transforms::mod8x(&mut state.x, &mut state.y, &mut state.z, smulti),
            6 => transforms::mod2x6x(&mut state.x, &mut state.y, &mut state.z, duoi, rng.gen_bool(0.5)),
            _ => transforms::mod2x(&mut state.x, &mut state.y, &mut state.z, duoi),
        };
        
        if p_flag { state.palupflag = true; }

        if state.palupflag {
            state.pali = state.pali.wrapping_add((PALSIZE - state.pali) >> 2);
            state.pali2 = state.pali.wrapping_sub(state.pali >> 2);
        } else {
            state.pali = state.pali.wrapping_sub(state.pali >> 2);
            state.pali2 = state
                .pali2
                .wrapping_add((PALSIZE - state.pali) >> 2);
        }

        if pmodi == 1 {
             let c = FRACTAL_COLORS[coli_idx % 8];
             state.dcr = ((state.dcr as u32 + c.0 as u32) >> 1) as u8;
             state.dcg = ((state.dcg as u32 + c.1 as u32) >> 1) as u8;
             state.dcb = ((state.dcb as u32 + c.2 as u32) >> 1) as u8;
        } else {
             let idx = state.pali & (PALSIZE - 1);
             let tcolor = palette.colors[idx];
             let tr = ((tcolor >> 16) & 0xFF) as u32;
             let tg = ((tcolor >> 8) & 0xFF) as u32;
             let tb = (tcolor & 0xFF) as u32;
             
             state.dcr = ((state.dcr as u32 + tr * 3) >> 2) as u8;
             state.dcg = ((state.dcg as u32 + tg * 3) >> 2) as u8;
             state.dcb = ((state.dcb as u32 + tb * 3) >> 2) as u8;
        }

        if i < 50 { continue; }

        if is_shadow_pass {
             if let Some(l_cam) = light {
                 if let Some((lx, ly, lzc)) = l_cam.project(state.x, state.y, state.z, width, height, LIGHT_SCALE) {
                     plot_z(lx, ly, lzc, None::<&mut [u32]>, zbuf.as_deref_mut(), width, height, 0);
                 }
            }
        } else {
            if let (Some(c), Some(frame), Some(depth)) = (cam, fb.as_deref_mut(), zbuf.as_deref_mut()) {
                if let Some((shadow_x, shadow_y, shadow_z)) = backstep_from_light(state.x, state.y, state.z, light) {
                    plot_view_point(
                        frame,
                        depth,
                        c,
                        light,
                        lbuf,
                        width,
                        height,
                        ifs,
                        shadow_x,
                        shadow_y,
                        shadow_z,
                        state.dcr as u32,
                        state.dcg as u32,
                        state.dcb as u32,
                        1.0,
                        false,
                        0.5,
                        lighting,
                    );
                }

                plot_view_point(
                    frame,
                    depth,
                    c,
                    light,
                    lbuf,
                    width,
                    height,
                    ifs,
                    state.x,
                    state.y,
                    state.z,
                    state.dcr as u32,
                    state.dcg as u32,
                    state.dcb as u32,
                    1.0,
                    false,
                    1.0,
                    lighting,
                );

                if ifs.x_mode <= 2 {
                    let (mirror_r, mirror_g, mirror_b) =
                        next_root_color(&state, pmodi, coli_idx, palette);
                    let (mx, my, mz) = (-state.x, -state.y, -state.z);

                    if let Some((shadow_x, shadow_y, shadow_z)) = backstep_from_light(mx, my, mz, light) {
                        plot_view_point(
                            frame,
                            depth,
                            c,
                            light,
                            lbuf,
                            width,
                            height,
                            ifs,
                            shadow_x,
                            shadow_y,
                            shadow_z,
                            mirror_r,
                            mirror_g,
                            mirror_b,
                            1.0,
                            false,
                            0.5,
                            lighting,
                        );
                    }

                    plot_view_point(
                        frame,
                        depth,
                        c,
                        light,
                        lbuf,
                        width,
                        height,
                        ifs,
                        mx,
                        my,
                        mz,
                        mirror_r,
                        mirror_g,
                        mirror_b,
                        1.0,
                        false,
                        1.0,
                        lighting,
                    );
                }
            }
        }
    }

    (state.x, state.y, state.z)
}

#[inline]
fn projected_depth(zc: f32) -> i32 {
    if zc <= 0.0 {
        return 0;
    }
    ((1.0 / zc) * ZDEPTH as f32).clamp(0.0, (ZDEPTH - 1) as f32) as i32
}

#[inline]
fn backstep_from_light(x: f32, y: f32, z: f32, light: Option<&LightCam>) -> Option<(f32, f32, f32)> {
    let light_cam = light?;
    let (lx, ly, lz) = light_cam.rotate_point(x, y, z);
    Some(light_cam.unrotate_point(lx, ly, lz + EXTRA_SHADOW_OFFSET))
}

#[inline]
fn next_root_color(state: &RenderState, pmodi: u32, coli_idx: usize, palette: &Palette) -> (u32, u32, u32) {
    if pmodi == 1 {
        let c = FRACTAL_COLORS[coli_idx % 8];
        (
            ((state.dcr as u32 + c.0 as u32) >> 1) & 0xFF,
            ((state.dcg as u32 + c.1 as u32) >> 1) & 0xFF,
            ((state.dcb as u32 + c.2 as u32) >> 1) & 0xFF,
        )
    } else {
        let idx = state.pali2 & (PALSIZE - 1);
        let tcolor = palette.colors[idx];
        let tr = (tcolor >> 16) & 0xFF;
        let tg = (tcolor >> 8) & 0xFF;
        let tb = tcolor & 0xFF;
        (
            ((state.dcr as u32 + tr * 3) >> 2) & 0xFF,
            ((state.dcg as u32 + tg * 3) >> 2) & 0xFF,
            ((state.dcb as u32 + tb * 3) >> 2) & 0xFF,
        )
    }
}

#[inline]
fn plot_view_point(
    fb: &mut [u32],
    zbuf: &mut [i32],
    cam: &Camera,
    light: Option<&LightCam>,
    lbuf: Option<&[i32]>,
    width: usize,
    height: usize,
    ifs: &IfsControl,
    x: f32,
    y: f32,
    z: f32,
    r: u32,
    g: u32,
    b: u32,
    glow: f32,
    use_glow: bool,
    brightness_scale: f32,
    lighting: &mut LightCalibration,
) {
    if let Some((sx, sy, zc)) = cam.view_project(x, y, z, width, height, VIEW_SCALE) {
        let (mut bright, mut luma, mut overexpose) =
            calculate_lighting(x, y, z, light, lbuf, lighting, glow, use_glow);
        if brightness_scale != 1.0 {
            bright = ((bright as f32) * brightness_scale).min(255.0) as u32;
            luma = 1.0;
            overexpose = 0;
        }

        let (final_r, final_g, final_b) =
            apply_color_mode(r, g, b, bright, luma, overexpose, ifs.whitershade);
        let final_col = (final_r << 16) | (final_g << 8) | final_b;
        plot_z(sx, sy, zc, Some(fb), Some(zbuf), width, height, final_col);
    }
}

#[inline]
fn calculate_lighting(
    x: f32, y: f32, z: f32, 
    light: Option<&LightCam>, 
    lbuf: Option<&[i32]>, 
    lighting: &mut LightCalibration,
    glow: f32,
    use_glow: bool
) -> (u32, f32, u32) { // Returns (brightness 0..255, luma, overexpose)
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
            if t < lighting.minbright {
                lighting.minbright = t;
            }
            t -= lighting.minbright;
            if t > lighting.maxbright {
                lighting.maxbright = t;
            }
            t /= lighting.maxbright.max(0.0001);

            t *= 2.0;
            luma = (t - 1.0).max(0.0);
            luma = 1.0 + luma.powf(8.0);
            if t > 1.0 {
                t = 1.0;
            }
            overexpose = ((255.0 * luma) as i32 - 0xFF).max(0) as u32;
            
            // Shadows
            if let Some(lb) = lbuf {
                 if lx >= 0
                    && ly >= 0
                    && (lx as usize) < LWIDTH as usize
                    && (ly as usize) < LHEIGHT as usize
                {
                     let lidx = ly as usize * LWIDTH as usize + lx as usize;
                     let ld = projected_depth(lzc);
                     if lb[lidx] > ld + 32 {
                         if use_glow {
                             bright = ((48.0 * glow + 208.0 * t) as u32).min(255);
                         } else {
                             bright = ((255.0 * t) as u32).min(255);
                         }
                         bright >>= 1;
                         overexpose = 0;
                         luma = 1.0;
                         return (bright, luma, overexpose);
                     }
                 }
            }
            
            if use_glow {
                 bright = ((48.0 * glow + 208.0 * t) as u32).min(255);
            } else {
                 bright = ((255.0 * t) as u32).min(255);
            }
            
        }
    }

    // Fallback brightness if lighting calculation failed or light source not provided.
    if bright == 0 {
        bright = if use_glow {
            (48.0 * glow + 128.0).clamp(32.0, 255.0) as u32
        } else {
            192
        };
        luma = 1.0;
        overexpose = 0;
    }
    
    (bright, luma, overexpose)
}

#[inline]
fn apply_color_mode(
    r: u32, g: u32, b: u32, 
    bright: u32, 
    luma: f32,
    overexpose: u32,
    mode: i32
) -> (u32, u32, u32) {
    let mut cr = r;
    let mut cg = g;
    let mut cb = b;

    if overexpose != 0 {
        cr = (((cr + overexpose) as f32) / luma).min(255.0) as u32;
        cg = (((cg + overexpose) as f32) / luma).min(255.0) as u32;
        cb = (((cb + overexpose) as f32) / luma).min(255.0) as u32;
    }

    if mode != 0 {
        if mode == 1 { // Fluorescent / Cold
            cr = ((cr * 3) + bright) >> 2;
            cg = ((cg << 1) + bright) / 3;
            cb = (cb + bright) >> 1;
        } else { // Filament / Warm
            cr = (cr + bright) >> 1;
            cg = ((cg << 1) + bright) / 3;
            cb = ((cb * 3) + bright) >> 2;
        }
    }
    
    // Final brightness modulation
    cr = (cr * bright) >> 8;
    cg = (cg * bright) >> 8;
    cb = (cb * bright) >> 8;
    
    (cr.min(255), cg.min(255), cb.min(255))
}

#[inline]
fn plot_z(x: i32, y: i32, zc: f32, fb: Option<&mut [u32]>, zbuf: Option<&mut [i32]>, w: usize, h: usize, col: u32) {
    if x < 0 || y < 0 || (x as usize) >= w || (y as usize) >= h { return; }
    let depth = projected_depth(zc);
    let idx = y as usize * w + x as usize;
    
    if let Some(zb) = zbuf {
        if depth > zb[idx] {
            zb[idx] = depth;
            if let Some(f) = fb {
                f[idx] = col;
            }
        }
    } else if let Some(f) = fb {
        f[idx] = col;
    }
}
