mod palette;
mod constants;
mod buffers;
mod renderer;
mod transforms;

use minifb::{Key, Window, WindowOptions};

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

    let mut pal = palette::Palette::new();
    pal.randomize();

    let mut tick: u32 = 0;
    let mut mode = renderer::Mode::Julia2D;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            pal.randomize();
        }
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
            mode = match mode { renderer::Mode::Julia2D => renderer::Mode::IFS2D, _ => renderer::Mode::Julia2D };
        }

        renderer::render(&mut frame, width, height, &pal, tick, mode);
        tick = tick.wrapping_add(1);

        window
            .update_with_buffer(&frame, width, height)
            .expect("Failed to update window");
    }
}


