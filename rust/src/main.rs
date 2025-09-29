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
            );
        } else {
            renderer::render(&mut frame, width, height, &pal, tick, mode, None, None, None, None, samples_view, 0);
        }
        tick = tick.wrapping_add(1);

        window
            .update_with_buffer(&frame, width, height)
            .expect("Failed to update window");
    }
}


