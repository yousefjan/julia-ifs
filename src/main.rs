mod palette;
mod constants;
mod buffers;
mod renderer;
mod transforms;
mod camera;
mod light;

use minifb::{Key, Window, WindowOptions};
use buffers::Framebuffers;
use rand::Rng;

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

    let mut fb = Framebuffers::new();
    let mut cam = camera::Camera::new();
    let light = light::LightCam::new();
    let mut show_shadows = true;
    let mut view_dirty = true;
    let mut shadow_dirty = true;
    
    // Sample counts
    let mut samples_view: usize = (width * height) / 4; 
    let mut samples_light: usize = (width * height) / 6;

    let mut ifs = renderer::IfsControl::default();
    ifs.set3d_index = Some(3); // Start with Set D
    ifs.preset_idx = 0;
    ifs.freeze_c = true;

    let mut pal = palette::Palette::new();
    pal.randomize_with_seed(0x5EED);

    let mut tick: u32 = 0;
    // We primarily use IFS3D mode as it covers all C++ functionality
    let mode = renderer::Mode::IFS3D;

    println!("Controls:");
    println!("  Esc: Exit");
    println!("  Space: Toggle Animation (Freeze C)");
    println!("  S: Cycle Set (A, B, C, D, E, D3, 2D, 2D3)");
    println!("  X: Cycle X-Mode (Symmetry)");
    println!("  Z: Toggle Secret Ingredient");
    println!("  N: Next Preset");
    println!("  B: Cycle Background");
    println!("  C: Clear View + Shadow Buffers");
    println!("  V: Clear View Buffer");
    println!("  L: Toggle Lightness");
    println!("  W: Cycle Whiter Shade of Pale (Color Correction)");
    println!("  P: Randomize Palette");
    println!("  R: Randomize View/Params");
    println!("  Arrows: Rotate Camera");
    println!("  PgUp/PgDn: Zoom");
    println!("  Home: Reset Zoom");
    println!("  +/-: Adjust Quality (Sample Count)");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Palette
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            pal.randomize();
            view_dirty = true;
            println!("Palette Randomized");
        }

        // Sets
        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            let cur = ifs.set3d_index.unwrap_or(0);
            ifs.set3d_index = Some((cur + 1) % 8);
            view_dirty = true;
            shadow_dirty = true;
            println!("Set: {:?}", ifs.set3d_index.unwrap());
        }

        // X-Modes
        if window.is_key_pressed(Key::X, minifb::KeyRepeat::No) {
            ifs.x_mode = (ifs.x_mode + 1) % 7;
            view_dirty = true;
            shadow_dirty = true;
            println!("X-Mode: {}", ifs.x_mode);
        }

        // Secret Ingredient
        if window.is_key_pressed(Key::Z, minifb::KeyRepeat::No) {
            ifs.secret_ingredient = !ifs.secret_ingredient;
            view_dirty = true;
            shadow_dirty = true;
            if ifs.secret_ingredient {
                let mut rng = rand::thread_rng();
                ifs.secret_square = rng.gen_bool(0.5);
                ifs.secret_extra_coord = rng.gen_bool(0.5);
                if !ifs.secret_square { ifs.secret_extra_coord = true; }
                ifs.secret_size = 1.0 + rng.gen::<f32>() * 5.0;
                println!("Secret Ingredient: ON (Sq:{}, Ex:{}, Sz:{:.2})", ifs.secret_square, ifs.secret_extra_coord, ifs.secret_size);
            } else {
                println!("Secret Ingredient: OFF");
            }
        }

        // Presets
        if window.is_key_pressed(Key::N, minifb::KeyRepeat::No) {
            ifs.preset_idx = (ifs.preset_idx + 1) % 10;
            view_dirty = true;
            shadow_dirty = true;
            println!("Preset: {}", ifs.preset_idx);
            if ifs.preset_idx == 9 {
                let mut rng = rand::thread_rng();
                ifs.c_x = rng.gen_range(-1.0..1.0);
                ifs.c_y = rng.gen_range(-1.0..1.0);
                ifs.c_z = rng.gen_range(-1.0..1.0);
                println!(
                    "Preset 9 randomized to ({:.3}, {:.3}, {:.3})",
                    ifs.c_x, ifs.c_y, ifs.c_z
                );
            }
        }

        // Background
        if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
            ifs.background_mode = (ifs.background_mode + 1) % 5;
            view_dirty = true;
            shadow_dirty = true;
            let mode_name = match ifs.background_mode {
                0 => "On",
                1 => "Off blue",
                2 => "Off black",
                3 => "Off grey",
                _ => "Off white",
            };
            println!("Background: {}", mode_name);
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            view_dirty = true;
            shadow_dirty = true;
            println!("Clearing view and shadow buffers");
        }

        if window.is_key_pressed(Key::V, minifb::KeyRepeat::No) {
            view_dirty = true;
            println!("Clearing view buffer");
        }

        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No) {
            ifs.lightness = (ifs.lightness + 1) % 2;
            view_dirty = true;
            let mode_name = if ifs.lightness == 0 { "Dark" } else { "Light" };
            println!("Lightness: {}", mode_name);
        }

        // Whiter Shade
        if window.is_key_pressed(Key::W, minifb::KeyRepeat::No) {
            ifs.whitershade = (ifs.whitershade + 1) % 3;
            view_dirty = true;
            let mode_name = match ifs.whitershade {
                1 => "Fluorescent",
                2 => "Filament",
                _ => "Normal"
            };
            println!("Whiter Shade: {}", mode_name);
        }

        // Randomize
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            let mut rng = rand::thread_rng();
            cam = camera::Camera::new();
            cam.yaw = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);
            cam.pitch = rng.gen_range(-1.0..1.0);
            view_dirty = true;
            // Also randomize C if in random preset
            if ifs.preset_idx == 9 {
                ifs.c_x = rng.gen_range(-1.0..1.0);
                ifs.c_y = rng.gen_range(-1.0..1.0);
                ifs.c_z = rng.gen_range(-1.0..1.0);
                view_dirty = true;
                shadow_dirty = true;
            }
            println!("Randomized View");
        }

        // Animation
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            ifs.freeze_c = !ifs.freeze_c;
            view_dirty = true;
            shadow_dirty = true;
            println!("Animation Frozen: {}", ifs.freeze_c);
        }

        // Quality
        if window.is_key_pressed(Key::Minus, minifb::KeyRepeat::No) {
            samples_view = (samples_view as f32 * 0.8) as usize;
            samples_view = samples_view.max(10000);
            view_dirty = true;
            println!("Samples: {}", samples_view);
        }
        if window.is_key_pressed(Key::Equal, minifb::KeyRepeat::No) {
            samples_view = (samples_view as f32 * 1.25) as usize;
            view_dirty = true;
            println!("Samples: {}", samples_view);
        }

        // Camera
        if window.is_key_down(Key::Left) { cam.yaw -= 0.05; view_dirty = true; }
        if window.is_key_down(Key::Right) { cam.yaw += 0.05; view_dirty = true; }
        if window.is_key_down(Key::Up) { cam.pitch = (cam.pitch + 0.05).clamp(-1.5, 1.5); view_dirty = true; }
        if window.is_key_down(Key::Down) { cam.pitch = (cam.pitch - 0.05).clamp(-1.5, 1.5); view_dirty = true; }
        
        if window.is_key_down(Key::PageUp) { cam.dist = (cam.dist - 0.1).max(0.5); view_dirty = true; }
        if window.is_key_down(Key::PageDown) { cam.dist = (cam.dist + 0.1).min(20.0); view_dirty = true; }
        if window.is_key_pressed(Key::Home, minifb::KeyRepeat::No) { cam.dist = 3.5; view_dirty = true; }

        // Render
        let clear_view_buffer = view_dirty || !ifs.freeze_c;
        let clear_shadow_map = show_shadows && (shadow_dirty || !ifs.freeze_c);

        renderer::render(
            &pal,
            tick,
            mode,
            &mut fb,
            Some(&cam),
            if show_shadows { Some(&light) } else { None },
            samples_view,
            if show_shadows { samples_light } else { 0 },
            &mut ifs,
            clear_view_buffer,
            clear_shadow_map,
        );

        if view_dirty && ifs.freeze_c {
            view_dirty = false;
        }
        if shadow_dirty && ifs.freeze_c {
            shadow_dirty = false;
        }

        tick = tick.wrapping_add(1);

        window
            .update_with_buffer(&fb.screen, width, height)
            .expect("Failed to update window");
    }
}
