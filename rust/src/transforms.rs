#[inline]
pub fn set2d(mut x: f32, mut y: f32) -> (f32, f32) {
    let length = (x * x + y * y).sqrt();
    if length > 0.0 {
        let amp = length.sqrt();
        if length == x.abs() {
            if x < 0.0 {
                y = amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            x = ((x - length) / 2.0) + length;
            y = y / 2.0;
            let s = amp / (x * x + y * y).sqrt();
            x *= s;
            y *= s;
        }
    }
    (x, y)
}

#[inline]
pub fn set2d3(mut x: f32, mut y: f32) -> (f32, f32) {
    let mut length = (x * x + y * y).sqrt();
    if length > 0.0 {
        let amp = length.powf(1.0 / 3.0);
        if length == x.abs() {
            if x < 0.0 {
                y = amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            x = ((x - length) / 3.0) + length;
            y = y / 3.0;
            length = amp / (x * x + y * y).sqrt();
            x *= length;
            y *= length;
        }
    }
    (x, y)
}

#[inline]
pub fn set3_a(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let mut llength = (y * y + z * z).sqrt();
        if llength > 0.0 {
            y /= llength;
            z /= llength;
            let mut t = 1.0f32;
            if z < 0.0 { t = -t; }
            let cosx = y;
            y = ((1.0 + cosx) / 2.0).sqrt();
            z = t * ((1.0 - cosx) / 2.0).sqrt();
            y *= llength;
            z *= llength;
        }
        llength = (x * x + y * y).sqrt();
        if llength > 0.0 {
            x /= llength;
            y /= llength;
            let mut t = 1.0f32;
            if y < 0.0 { t = -t; }
            let cosx = x;
            x = ((1.0 + cosx) / 2.0).sqrt();
            y = t * ((1.0 - cosx) / 2.0).sqrt();
            x *= llength;
            y *= llength;
        }
        llength = (x * x + y * y + z * z).sqrt();
        length = length.sqrt();
        if llength > 0.0 {
            x = (x / llength) * length;
            y = (y / llength) * length;
            z = (z / llength) * length;
        }
    }
    (x, y, z)
}

#[inline]
pub fn set3_b(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let mut llength = (y * y + z * z).sqrt();
        if llength > 0.0 {
            let mut t = 1.0f32;
            if z < 0.0 { t = -t; }
            let cosx = y / llength;
            y = ((1.0 + cosx) / 2.0).sqrt();
            z = t * ((1.0 - cosx) / 2.0).sqrt();
        }
        llength = (x * x + y * y).sqrt();
        if llength > 0.0 {
            let mut t = 1.0f32;
            if y < 0.0 { t = -t; }
            let cosx = x / llength;
            x = ((1.0 + cosx) / 2.0).sqrt();
            y = t * ((1.0 - cosx) / 2.0).sqrt();
        }
        llength = (x * x + y * y + z * z).sqrt();
        length = length.sqrt();
        if llength > 0.0 {
            x = (x / llength) * length;
            y = (y / llength) * length;
            z = (z / llength) * length;
        }
    }
    (x, y, z)
}

#[inline]
pub fn set3_c(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let mut llength = (y * y + z * z).sqrt();
        if llength > 0.0 {
            let mut yt = y / llength;
            let mut zt = -z / llength;
            y = length;
            z = 0.0;
            llength = (x * x + y * y).sqrt();
            if llength > 0.0 {
                let mut t = 1.0f32;
                if y < 0.0 { t = -t; }
                let cosx = x / llength;
                llength = llength.sqrt();
                x = ((1.0 + cosx) / 2.0).sqrt() / llength;
                y = t * ((1.0 - cosx) / 2.0).sqrt() / llength;
            }
            llength = (yt * yt + zt * zt).sqrt();
            if llength > 0.0 {
                let mut t = 1.0f32;
                if zt < 0.0 { t = -t; }
                let cosx = yt / llength;
                yt = ((1.0 + cosx) / 2.0).sqrt();
                zt = t * ((1.0 - cosx) / 2.0).sqrt();
            }
            llength = (yt * yt + zt * zt).sqrt();
            if llength > 0.0 {
                yt = yt / llength;
                zt = -zt / llength;
            }
            z = zt * y;
            y = yt * y;
        }
        llength = (x * x + y * y + z * z).sqrt();
        length = length.sqrt();
        if llength > 0.0 {
            x = (x / llength) * length;
            y = (y / llength) * length;
            z = (z / llength) * length;
        }
    }
    (x, y, z)
}

#[inline]
pub fn set3_d(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.sqrt();
        if length == x.abs() {
            if x < 0.0 {
                let angle = (tick_hash(x, y, z) * std::f32::consts::PI);
                y = angle.cos() * amp;
                z = angle.sin() * amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            x = ((x - length) / 2.0) + length;
            y = y / 2.0;
            z = z / 2.0;
            length = amp / (x * x + y * y + z * z).sqrt();
            x *= length;
            y *= length;
            z *= length;
        }
    }
    (x, y, z)
}

#[inline]
pub fn set3_e(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.powf(1.0 / 3.0);
        if length == x.abs() {
            if x < 0.0 {
                let angle = (tick_hash(x, y, z) * std::f32::consts::PI);
                y = angle.cos() * amp;
                z = angle.sin() * amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            length = (y * y + z * z).sqrt();
            let (mut yy, mut zz) = if length > 0.0 { (y / length, z / length) } else { (1.0, 0.0) };
            y = length;
            length = (x * x + y * y).sqrt();
            if length > 0.0 {
                let mut t = 1.0f32;
                if y < 0.0 { t = -t; }
                let cosx = x / length;
                x = ((1.0 + cosx) / 2.0).sqrt();
                y = t * ((1.0 - cosx) / 2.0).sqrt();
            }
            z = y * zz;
            y = y * yy;
            let s = amp / (x * x + y * y + z * z).sqrt();
            x *= s; y *= s; z *= s;
        }
    }
    (x, y, z)
}

#[inline]
pub fn set3_d3(mut x: f32, mut y: f32, mut z: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.powf(1.0 / 3.0);
        if length == x.abs() {
            if x < 0.0 {
                let angle = (tick_hash(x, y, z) * std::f32::consts::PI);
                y = angle.cos() * amp;
                z = angle.sin() * amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            x = ((x - length) * (2.0 * 3.0)) + length;
            y = y * (2.0 * 3.0);
            z = z * (2.0 * 3.0);
            let s = amp / (x * x + y * y + z * z).sqrt();
            x *= s; y *= s; z *= s;
        }
    }
    (x, y, z)
}

#[inline]
fn tick_hash(x: f32, y: f32, z: f32) -> f32 { ((x + 3.1 * y + 5.3 * z).sin() * 0.5 + 0.5).max(0.0) }


