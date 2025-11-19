use crate::constants::{BGCOLOR, PALSIZE, ZDEPTH, BOTTOM_COLORS, FRACTAL_COLORS, PRESETS};
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
    pub show_background: bool,
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
            show_background: false,
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
    
    let (ifs_cx, ifs_cy, ifs_cz) = if ifs.freeze_c {
        if ifs.use_presets {
            PRESETS[ifs.preset_idx.min(8)]
        } else {
            (ifs.c_x, ifs.c_y, ifs.c_z)
        }
    } else {
        (anim_cx, anim_cy, 0.0)
    };
    
    // If preset 9 (random) is selected, logic is handled dynamically or via inputs. 
    // For now, use calculated if preset is random/dynamic.

    match mode {
        Mode::Julia2D => render_julia_2d(framebuffer, width, height, palette, ifs_cx, ifs_cy, aspect, tick),
        Mode::IFS2D => render_ifs_2d(framebuffer, width, height, palette, ifs, ifs_cx, ifs_cy, aspect, samples_view, tick),
        Mode::IFS3D => {
            framebuffer.fill(BGCOLOR);
            
            if let Some(zbuf) = zbuffer.as_deref_mut() {
                 for z in zbuf.iter_mut() { *z = ZDEPTH; }
            }

            let seed_tick = if ifs.freeze_sets { 0 } else { (tick as u64) / 90 };
            
            // 1. Shadow Map Pass
            let mut shadow_rng = StdRng::seed_from_u64(0x5AD0_5AD0 + seed_tick);
            if let (Some(light_cam), Some(lbuf)) = (light, lightbuf.as_deref_mut()) {
                lbuf.fill(ZDEPTH);
                // Bottom Plane Shadows
                if !ifs.show_background { // "showbackground == 0" means background is ON in C++ (inverted logic name maybe? No, C++ says "Show background? if (showbackground == 0)")
                     // Actually C++: "Background is visible: showbackground = 0;"
                     render_bottom_layer(None, None, None, Some(lbuf), Some(light_cam), width, height, ifs, samples_light / 4, &mut shadow_rng, true);
                }
                // Fractal Shadows
                render_fractal_layer(None, None, None, Some(lbuf), Some(light_cam), width, height, ifs, samples_light, &mut shadow_rng, true, palette, ifs_cx, ifs_cy, ifs_cz);
            }

            // 2. View Pass
            let mut view_rng = StdRng::seed_from_u64(0xF00D_F00D + seed_tick);
            if let Some(cam) = camera {
                 // Bottom Plane View
                 if !ifs.show_background {
                     render_bottom_layer(Some(framebuffer), zbuffer.as_deref_mut(), Some(cam), lightbuf.as_deref(), light, width, height, ifs, samples_view / 4, &mut view_rng, false);
                 }
                 // Fractal View
                 render_fractal_layer(Some(framebuffer), zbuffer.as_deref_mut(), Some(cam), lightbuf.as_deref(), light, width, height, ifs, samples_view, &mut view_rng, false, palette, ifs_cx, ifs_cy, ifs_cz);
            }
        }
    }
}

fn render_julia_2d(fb: &mut [u32], width: usize, height: usize, palette: &Palette, cx: f32, cy: f32, aspect: f32, _tick: u32) {
    // ... existing Julia2D implementation ...
    // Simplified for brevity, using previous logic
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
        // Simple symmetry for 2D mode
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
    lbuf: Option<&[i32]>, // Read-only shadow map
    light: Option<&LightCam>,
    width: usize, height: usize,
    ifs: &IfsControl,
    samples: usize,
    rng: &mut StdRng,
    is_shadow_pass: bool
) {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut z = 0.0f32;
    
    // Bottom colors (initially tcr[0])
    let mut bcr = BOTTOM_COLORS[0].0 as u32;
    let mut bcg = BOTTOM_COLORS[0].1 as u32;
    let mut bcb = BOTTOM_COLORS[0].2 as u32;

    let set_idx = ifs.set3d_index.unwrap_or(0);
    let vectors = transforms::get_bottom_vectors(set_idx);
    // sc is always 0.5
    
    // The loop goes for 128 iterations in C++, but here we sample many points
    // We need to iterate similarly to fractal: iterative function system.
    // C++ iterates 128 times per frame? No, it iterates 128 times per "dot"?
    // Wait, the C++ loop: "for ( pti = 128; pti >= 0; pti-- )" -> This is VERY small if it's per frame.
    // Ah, DoMyStuff is called repeatedly?
    // "if ( ( programMode == 0 ) && renderactive ) DoMyStuff ( );"
    // Yes, it's called in a loop.
    // So we should run 'samples' iterations.
    
    for i in 0..samples {
        let bi = rng.gen_range(0..4);
        let tx = vectors[bi];
        
        // btx = ( btx - tx [ bi ] ) * sc [ bi ];
        x = (x - tx.0) * 0.5;
        y = (y - tx.1) * 0.5;
        z = (z - tx.2) * 0.5;
        
        // Glow logic skipped for now
        
        // btx += tx [ bi ];
        x += tx.0;
        y += tx.1;
        z += tx.2;
        
        // Color update
        let tc = BOTTOM_COLORS[bi];
        bcr = ((bcr + tc.0 as u32) >> 1) & 0xFF;
        bcg = ((bcg + tc.1 as u32) >> 1) & 0xFF;
        bcb = ((bcb + tc.2 as u32) >> 1) & 0xFF;
        
        if i < 20 { continue; } // Warmup
        
        // Plot
        let color = (bcr << 16) | (bcg << 8) | bcb;
        
        if is_shadow_pass {
            // Plot to light buffer
            if let Some(l_cam) = light {
                 if let Some((lx, ly, lzc)) = l_cam.project(x, y, z, width, height, 280.0) {
                     plot_z(lx, ly, lzc, None::<&mut [u32]>, zbuf.as_deref_mut(), width, height, 0);
                 }
            }
        } else {
            // Plot to view buffer
            if let Some(c) = cam {
                if let Some((sx, sy, zc)) = c.view_project(x, y, z, width, height, 260.0) {
                    // Shadow check
                    let shade = calculate_shadow(x, y, z, light, lbuf, width, height);
                    let r = ((color >> 16) as f32 * shade) as u32;
                    let g = (((color >> 8) & 0xFF) as f32 * shade) as u32;
                    let b = ((color & 0xFF) as f32 * shade) as u32;
                    let final_col = (r << 16) | (g << 8) | b;
                    
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
    samples: usize,
    rng: &mut StdRng,
    is_shadow_pass: bool,
    palette: &Palette,
    cx: f32, cy: f32, cz: f32
) {
    let mut state = RenderState {
        x: ifs.c_x, y: ifs.c_y, z: ifs.c_z, // Reset to constant
        dcr: FRACTAL_COLORS[0].0, dcg: FRACTAL_COLORS[0].1, dcb: FRACTAL_COLORS[0].2,
        pali: 0, pali2: 0, palupflag: false,
    };
    // Initialize state from presets if needed
    // In C++: dtx = xbuf[ui]; ...
    // Since we iterate many times, we rely on the attractor behavior. 
    // But we should start random to fill space?
    state.x = rng.gen_range(-1.0..1.0);
    state.y = rng.gen_range(-1.0..1.0);
    state.z = rng.gen_range(-1.0..1.0);

    let set_idx = ifs.set3d_index.unwrap_or(0); // Default A
    
    // Loop variables mirroring C++
    let mut repti = 0;
    let mut maxrepti = 0;
    let mut duoi;
    let mut sduoi = false; // storage for duoi
    let mut multi = 0usize;
    let mut smulti = 0usize;
    let mut indxn = 0;
    let mut indxuse = 0;
    let mut indxs = 0;
    let mut swapflag = false;
    let mut probability = 0.0;
    let mut useswap = false;

    for i in 0..samples {
        // C++: dtx -= pcx[ui]...
        state.x -= cx;
        state.y -= cy;
        state.z -= cz;

        // Secret Ingredient
        if ifs.secret_ingredient {
             if ifs.secret_extra_coord && (rng.gen_bool(0.5)) { // "secretextracoord & int(RND*2)"
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

        // Set Transform
        let randu = rng.gen::<f32>();
        match set_idx {
            0 => { let r = transforms::set3_a(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            1 => { let r = transforms::set3_b(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            2 => { let r = transforms::set3_c(state.x, state.y, state.z); state.x=r.0; state.y=r.1; state.z=r.2; }
            3 => { let r = transforms::set3_d(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            4 => { let r = transforms::set3_e(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            5 => { let r = transforms::set3_d3(state.x, state.y, state.z, randu); state.x=r.0; state.y=r.1; state.z=r.2; }
            6 => { let r = transforms::set2d(state.x, state.y); state.x=r.0; state.y=r.1; state.z=0.0; } // SET2D
            _ => { let r = transforms::set2d3(state.x, state.y); state.x=r.0; state.y=r.1; state.z=0.0; } // SET2D3
        }

        // X-mode Selector Logic
        repti -= 1;
        if repti <= 0 {
            maxrepti = (rng.gen::<f32>() * rng.gen::<f32>() * 128.0) as i32;
            repti = maxrepti;
            sduoi = rng.gen_bool(0.5);
            smulti = rng.gen_range(0..8);
            probability = rng.gen::<f32>();
        }
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

        // Apply Symmetry (X-mode)
        // returns (pmodi, coli, palupflag_override)
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

        // Palette Index Update
        if state.palupflag {
            state.pali = state.pali.wrapping_add((PALSIZE - state.pali) >> 2);
            state.pali2 = state.pali.wrapping_sub(state.pali >> 2);
        } else {
            state.pali = state.pali.wrapping_sub(state.pali >> 2);
            state.pali2 = state.pali.wrapping_add((PALSIZE - state.pali) >> 2);
        }

        // Color Update
        if pmodi == 1 {
             // COLMOD
             let c = FRACTAL_COLORS[coli_idx % 8];
             state.dcr = ((state.dcr as u32 + c.0 as u32) >> 1) as u8;
             state.dcg = ((state.dcg as u32 + c.1 as u32) >> 1) as u8;
             state.dcb = ((state.dcb as u32 + c.2 as u32) >> 1) as u8;
        } else {
             // COLPAL
             let idx = state.pali & (PALSIZE - 1);
             let tcolor = palette.colors[idx];
             let tr = ((tcolor >> 16) & 0xFF) as u32;
             let tg = ((tcolor >> 8) & 0xFF) as u32;
             let tb = (tcolor & 0xFF) as u32;
             
             // C++: dcr = ( ( dcr + ( tRed * 3 ) )   >> 2 ) & 0xFF;
             state.dcr = ((state.dcr as u32 + tr * 3) >> 2) as u8;
             state.dcg = ((state.dcg as u32 + tg * 3) >> 2) as u8;
             state.dcb = ((state.dcb as u32 + tb * 3) >> 2) as u8;
        }

        if i < 50 { continue; } // Warmup

        // Plotting
        // If 2X, 3X, 4X (nxi == 0, 1, 2) -> C++ has logic to write "Second Root" pixel sometimes? 
        // See lines 1322 in C++.
        // "Write the second root... only used for some of the x-modes"
        // I'll skip the second root logic for now to keep it simple, focusing on the main point.
        
        let color = ((state.dcr as u32) << 16) | ((state.dcg as u32) << 8) | (state.dcb as u32);

        if is_shadow_pass {
             if let Some(l_cam) = light {
                 // Main pixel shadow
                 if let Some((lx, ly, lzc)) = l_cam.project(state.x, state.y, state.z, width, height, 280.0) {
                     plot_z(lx, ly, lzc, None::<&mut [u32]>, zbuf.as_deref_mut(), width, height, 0);
                 }
                 // Backside shadow pixel? (lines 1272-1297 in C++)
                 // It rotates light, moves z, unrotates... basically adds a pixel behind.
                 // I'll skip for now.
            }
        } else {
            if let Some(c) = cam {
                if let Some((sx, sy, zc)) = c.view_project(state.x, state.y, state.z, width, height, 260.0) {
                    let shade = calculate_shadow(state.x, state.y, state.z, light, lbuf, width, height);
                    let r = ((color >> 16) as f32 * shade) as u32;
                    let g = (((color >> 8) & 0xFF) as f32 * shade) as u32;
                    let b = ((color & 0xFF) as f32 * shade) as u32;
                    let final_col = (r << 16) | (g << 8) | b;
                    plot_z(sx, sy, zc, fb.as_deref_mut(), zbuf.as_deref_mut(), width, height, final_col);
                }
            }
        }
    }
}

#[inline]
fn calculate_shadow(x: f32, y: f32, z: f32, light: Option<&LightCam>, lbuf: Option<&[i32]>, w: usize, h: usize) -> f32 {
    if let (Some(l_cam), Some(lb)) = (light, lbuf) {
        if let Some((lx, ly, lzc)) = l_cam.project(x, y, z, w, h, 280.0) {
            if lx >= 0 && ly >= 0 && (lx as usize) < w && (ly as usize) < h {
                 let lidx = ly as usize * w + lx as usize;
                 let ld = ((lzc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
                 if ld <= lb[lidx] + 16 { 1.0 } else { 0.4 }
            } else { 1.0 }
        } else { 1.0 }
    } else { 1.0 }
}

#[inline]
fn plot_z(x: i32, y: i32, zc: f32, fb: Option<&mut [u32]>, zbuf: Option<&mut [i32]>, w: usize, h: usize, col: u32) {
    if x < 0 || y < 0 || (x as usize) >= w || (y as usize) >= h { return; }
    let depth = ((zc * 2048.0) as i32).clamp(0, ZDEPTH - 1);
    let idx = y as usize * w + x as usize;
    
    if let Some(zb) = zbuf {
        if depth < zb[idx] {
            zb[idx] = depth;
            if let Some(f) = fb {
                f[idx] = col;
            }
        }
    } else if let Some(f) = fb {
        f[idx] = col;
    }
}
