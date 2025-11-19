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

    let mut frame = vec![0u32; width * height];
    let mut fb = Framebuffers::new();
    let mut cam = camera::Camera::new();
    let light = light::LightCam::new();
    let mut show_shadows = true;
    
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
    println!("  B: Toggle Background");
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
            println!("Palette Randomized");
        }

        // Sets
        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            let cur = ifs.set3d_index.unwrap_or(0);
            ifs.set3d_index = Some((cur + 1) % 8);
            println!("Set: {:?}", ifs.set3d_index.unwrap());
        }

        // X-Modes
        if window.is_key_pressed(Key::X, minifb::KeyRepeat::No) {
            ifs.x_mode = (ifs.x_mode + 1) % 7;
            println!("X-Mode: {}", ifs.x_mode);
        }

        // Secret Ingredient
        if window.is_key_pressed(Key::Z, minifb::KeyRepeat::No) {
            ifs.secret_ingredient = !ifs.secret_ingredient;
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
            println!("Preset: {}", ifs.preset_idx);
            if ifs.preset_idx == 9 {
                // Randomize preset 9
                // We don't actually store the random params in constants, 
                // but renderer handles random generation if needed or we can just let it be dynamic
                // For now, renderer uses a fixed random seed or similar.
                println!("Preset 9 is random/dynamic");
            }
        }

        // Background
        if window.is_key_pressed(Key::B, minifb::KeyRepeat::No) {
            ifs.show_background = !ifs.show_background; // Toggle
            println!("Background Hidden: {}", ifs.show_background);
        }

        // Randomize
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            let mut rng = rand::thread_rng();
            cam = camera::Camera::new();
            cam.yaw = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);
            cam.pitch = rng.gen_range(-1.0..1.0);
            // Also randomize C if in random preset
            if ifs.preset_idx == 9 {
                ifs.c_x = rng.gen_range(-1.0..1.0);
                ifs.c_y = rng.gen_range(-1.0..1.0);
                ifs.c_z = rng.gen_range(-1.0..1.0);
            }
            println!("Randomized View");
        }

        // Animation
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            ifs.freeze_c = !ifs.freeze_c;
            println!("Animation Frozen: {}", ifs.freeze_c);
        }

        // Quality
        if window.is_key_pressed(Key::Minus, minifb::KeyRepeat::No) {
            samples_view = (samples_view as f32 * 0.8) as usize;
            samples_view = samples_view.max(10000);
            println!("Samples: {}", samples_view);
        }
        if window.is_key_pressed(Key::Equal, minifb::KeyRepeat::No) {
            samples_view = (samples_view as f32 * 1.25) as usize;
            println!("Samples: {}", samples_view);
        }

        // Camera
        if window.is_key_down(Key::Left) { cam.yaw -= 0.05; }
        if window.is_key_down(Key::Right) { cam.yaw += 0.05; }
        if window.is_key_down(Key::Up) { cam.pitch = (cam.pitch + 0.05).clamp(-1.5, 1.5); }
        if window.is_key_down(Key::Down) { cam.pitch = (cam.pitch - 0.05).clamp(-1.5, 1.5); }
        
        if window.is_key_down(Key::PageUp) { cam.dist = (cam.dist - 0.1).max(0.5); }
        if window.is_key_down(Key::PageDown) { cam.dist = (cam.dist + 0.1).min(20.0); }
        if window.is_key_pressed(Key::Home, minifb::KeyRepeat::No) { cam.dist = 3.5; }

        // Render
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

        tick = tick.wrapping_add(1);

        window
            .update_with_buffer(&frame, width, height)
            .expect("Failed to update window");
    }
}
