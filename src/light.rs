pub struct LightCam {
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
}

impl LightCam {
    pub fn new() -> Self { Self { yaw: 0.9, pitch: 0.8, dist: 3.5 } }

    pub fn rotate_point(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();

        let xr = cy * x + sy * z;
        let yr = sp * (sy * x - cy * z) + cp * y;
        let zr = cp * (sy * x - cy * z) - sp * y;
        (xr, yr, zr)
    }

    pub fn unrotate_point(&self, xr: f32, yr: f32, zr: f32) -> (f32, f32, f32) {
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();

        let y = cp * yr - sp * zr;
        let axis = sp * yr + cp * zr;
        let x = cy * xr + sy * axis;
        let z = sy * xr - cy * axis;
        (x, y, z)
    }

    pub fn project(&self, x: f32, y: f32, z: f32, width: usize, height: usize, scale: f32) -> Option<(i32, i32, f32)> {
        let (xr, yr, zr) = self.rotate_point(x, y, z);

        let zc = zr + self.dist;
        if zc <= 0.0 { return None; }
        let inv = 1.0 / zc;
        let sx = (xr * inv * scale + (width as f32 * 0.5)) as i32;
        let sy = (yr * inv * scale + (height as f32 * 0.5)) as i32;
        Some((sx, sy, zc))
    }
}


