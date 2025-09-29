pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub dist: f32,
}

impl Camera {
    pub fn new() -> Self { Self { yaw: 0.0, pitch: 0.0, dist: 3.5 } }

    pub fn view_project(&self, x: f32, y: f32, z: f32, width: usize, height: usize, scale: f32) -> Option<(i32, i32, f32)> {
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();

        // rotate around Y (yaw), then X (pitch)
        let xr =  cy * x + 0.0 * y + sy * z;
        let yr =  sp * (sy * x + 0.0 * y - cy * z) + cp * y;
        let zr =  cp * (sy * x + 0.0 * y - cy * z) - sp * y;

        let zc = zr + self.dist;
        if zc <= 0.0 { return None; }
        let inv = 1.0 / zc;
        let sx = (xr * inv * scale + (width as f32 * 0.5)) as i32;
        let sy = (yr * inv * scale + (height as f32 * 0.5)) as i32;
        Some((sx, sy, zc))
    }
}


