pub const NAME: &str = "3D IFS";
pub const ZDEPTH: i32 = 16384;
pub const PALSIZE: usize = 8192; // must be power of two

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;
pub const MIDX: i32 = WIDTH >> 1;
pub const MIDY: i32 = HEIGHT >> 1;
pub const BWIDTH: i32 = WIDTH << 1;
pub const BHEIGHT: i32 = HEIGHT << 1;
pub const BMIDX: i32 = MIDX << 1;
pub const BMIDY: i32 = MIDY << 1;
pub const LWIDTH: i32 = 4096;
pub const LHEIGHT: i32 = 4096;
pub const LMIDX: i32 = LWIDTH >> 1;
pub const LMIDY: i32 = LHEIGHT >> 1;

pub const BGCOLOR: u32 = 0x00103050;

pub const PRESETS: [(f32, f32, f32); 9] = [
    (0.0, 0.0, 0.0),
    (0.5, -0.3, 0.0),
    (-1.414289, 0.0, 0.0),
    (0.285, 0.013, 0.0),
    (0.35355339, 0.0, 0.0), // sqrt(2)/4
    (0.387860, 0.154406, 1.0),
    (-0.6875, -0.0625, -0.24849984),
    (-0.717612232, 0.217535936, 0.3), // y is 0.3 from line 1828
    (-0.25, 0.5, 0.75),
];

// Bottom plane colors (indices 0-3)
pub const BOTTOM_COLORS: [(u8, u8, u8); 4] = [
    (0x90, 0x90, 0x90),
    (0x70, 0x70, 0x70),
    (0x90, 0x90, 0x90),
    (0x70, 0x70, 0x70),
];

// Fractal colors (indices 4-11 -> 0-7 in this array)
pub const FRACTAL_COLORS: [(u8, u8, u8); 8] = [
    (0xFF, 0x00, 0x50),
    (0xFF, 0x80, 0x00),
    (0xFF, 0xFF, 0x00),
    (0x80, 0xC0, 0x00),
    (0x00, 0xC0, 0x40),
    (0x00, 0x80, 0xC0),
    (0x00, 0x00, 0xFF),
    (0x80, 0x00, 0xFF),
];
