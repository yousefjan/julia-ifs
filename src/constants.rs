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


