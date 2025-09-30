mod palette;
mod constants;
mod buffers;
mod renderer;
mod transforms;
mod camera;
mod light;

use minifb::{Key, Window, WindowOptions};
use buffers::Framebuffers;

fn main() {
    let width = constants::WIDTH as usize;
    let height = constants::HEIGHT as usize;

    let mut window = Window::new(
        "3D IFS (Rust)",
        width,
        height,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    let mut frame = vec![0u32; width * height];
    let mut fb = Framebuffers::new();
    let mut cam = camera::Camera::new();
    let mut light = light::LightCam::new();
    let mut show_shadows = true;
    let mut samples_view: usize = (width * height) / 6; // default balanced
    let mut samples_light: usize = (width * height) / 8;
    let mut ifs = renderer::IfsControl::default();

    let mut pal = palette::Palette::new();
    pal.randomize_with_seed(0x5EED);

    let mut tick: u32 = 0;
    let mut mode = renderer::Mode::Julia2D;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            pal.randomize();
        }
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            mode = match mode {
                renderer::Mode::Julia2D => renderer::Mode::IFS2D,
                renderer::Mode::IFS2D => renderer::Mode::IFS3D,
                renderer::Mode::IFS3D => renderer::Mode::Julia2D,
            };
        }
        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) { show_shadows = !show_shadows; }
        // IFS controls
        if window.is_key_pressed(Key::Comma, minifb::KeyRepeat::No) { ifs.time_scale = (ifs.time_scale * 0.5).max(0.0); }
        if window.is_key_pressed(Key::Period, minifb::KeyRepeat::No) { ifs.time_scale = (ifs.time_scale * 2.0).min(8.0); }
        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) { ifs.freeze_c = !ifs.freeze_c; }
        // Tweak IFS Julia constant when frozen
        if ifs.freeze_c {
            if window.is_key_down(Key::O) { ifs.c_x -= 0.0025; }
            if window.is_key_down(Key::P) { ifs.c_x += 0.0025; }
            if window.is_key_down(Key::K) { ifs.c_y -= 0.0025; }
            if window.is_key_down(Key::L) { ifs.c_y += 0.0025; }
            if window.is_key_pressed(Key::U, minifb::KeyRepeat::No) {
                // simple randomize around typical Julia constants
                let mut seed = tick as u64 ^ 0xC0FFEEu64;
                seed ^= (samples_view as u64) << 17;
                let r0 = (seed.wrapping_mul(6364136223846793005).wrapping_add(1) >> 33) as u32;
                let r1 = (seed.wrapping_mul(1442695040888963407).wrapping_add(1) >> 33) as u32;
                let fx = (r0 as f32 / u32::MAX as f32) * 2.0 - 1.0;
                let fy = (r1 as f32 / u32::MAX as f32) * 2.0 - 1.0;
                ifs.c_x = 0.3 * fx;
                ifs.c_y = 0.3 * fy;
            }
        }
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) { ifs.freeze_sets = !ifs.freeze_sets; }
        // 2D set selection
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) { ifs.set2d_variant = Some(0); }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) { ifs.set2d_variant = Some(1); }
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) { ifs.set2d_variant = None; }
        // 3D set family selection cycling
        if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
            let cur = ifs.set3d_index.unwrap_or(0);
            ifs.set3d_index = Some(((cur + 5) % 6) as i32);
        }
        if window.is_key_pressed(Key::N, minifb::KeyRepeat::No) {
            let cur = ifs.set3d_index.unwrap_or(0);
            ifs.set3d_index = Some(((cur + 1) % 6) as i32);
        }
        if window.is_key_pressed(Key::Backspace, minifb::KeyRepeat::No) { ifs.set3d_index = None; }
        if window.is_key_pressed(Key::Minus, minifb::KeyRepeat::No) {
            samples_view = (samples_view.saturating_sub((width * height) / 12)).max((width * height) / 40);
            samples_light = samples_light.saturating_sub((width * height) / 16).max((width * height) / 64);
        }
        if window.is_key_pressed(Key::Equal, minifb::KeyRepeat::No) {
            samples_view = (samples_view + (width * height) / 12).min(width * height * 2);
            samples_light = (samples_light + (width * height) / 16).min(width * height * 2);
        }
        // camera controls
        if window.is_key_down(Key::Left) { cam.yaw -= 0.03; }
        if window.is_key_down(Key::Right) { cam.yaw += 0.03; }
        if window.is_key_down(Key::Up) { cam.pitch = (cam.pitch + 0.03).clamp(-1.2, 1.2); }
        if window.is_key_down(Key::Down) { cam.pitch = (cam.pitch - 0.03).clamp(-1.2, 1.2); }
        if window.is_key_down(Key::Z) { cam.dist = (cam.dist - 0.05).max(1.5); }
        if window.is_key_down(Key::X) { cam.dist = (cam.dist + 0.05).min(8.0); }
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) { cam = camera::Camera::new(); }

        if let renderer::Mode::IFS3D = mode {
            if show_shadows { fb.clear_light(); }
            renderer::render(
                &mut frame,
                width,
                height,
                &pal,
                tick,
                mode,
                Some(&mut fb.zbuf),
                Some(&cam),
                if show_shadows { Some(&light) } else { None },
                if show_shadows { Some(&mut fb.light) } else { None },
                samples_view,
                if show_shadows { samples_light } else { 0 },
                &ifs,
            );
        } else {
            renderer::render(&mut frame, width, height, &pal, tick, mode, None, None, None, None, samples_view, 0, &ifs);
        }
        tick = tick.wrapping_add(1);

        window
            .update_with_buffer(&frame, width, height)
            .expect("Failed to update window");
    }
}


