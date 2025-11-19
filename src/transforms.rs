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
            // z = 0.0; // overwritten later
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
pub fn set3_d(mut x: f32, mut y: f32, mut z: f32, randu: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.sqrt();
        if length == x.abs() {
            if x < 0.0 {
                let angle = randu * std::f32::consts::PI;
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
pub fn set3_e(mut x: f32, mut y: f32, mut z: f32, randu: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.powf(1.0 / 3.0);
        if length == x.abs() {
            if x < 0.0 {
                let angle = randu * std::f32::consts::PI;
                y = angle.cos() * amp;
                z = angle.sin() * amp;
                x = 0.0;
            } else {
                x = amp;
            }
        } else {
            length = (y * y + z * z).sqrt();
            let (yy, zz) = if length > 0.0 { (y / length, z / length) } else { (1.0, 0.0) };
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
pub fn set3_d3(mut x: f32, mut y: f32, mut z: f32, randu: f32) -> (f32, f32, f32) {
    let mut length = (x * x + y * y + z * z).sqrt();
    if length > 0.0 {
        let amp = length.powf(1.0 / 3.0);
        if length == x.abs() {
            if x < 0.0 {
                let angle = randu * std::f32::consts::PI;
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
fn tick_hash(_x: f32, _y: f32, _z: f32) -> f32 { 0.5 }

// Symmetry Modes

// Returns (pmodi, coli, palupflag)
// pmodi: 0 for COLPAL, 1 for COLMOD
pub fn mod2x(x: &mut f32, y: &mut f32, z: &mut f32, duoi: bool) -> (u32, usize, bool) {
    let mut palupflag = false;
    if duoi {
        *x = -*x;
        *y = -*y;
        *z = -*z;
        palupflag = true;
    }
    (0, 0, palupflag)
}

pub fn mod3x(x: &mut f32, y: &mut f32, z: &mut f32, rng_val: i32) -> (u32, usize, bool) {
    let mut length = (*y * *y + *z * *z).sqrt();
    let (mut yy, mut zz);
    if length > 0.0 {
        yy = *y / length;
        zz = *z / length;
    } else {
        yy = 1.0;
        zz = 0.0;
    }
    *y = length;
    let di = rng_val; // int(RND * 3)
    let coli = (di << 1) + 4;
    let mut palupflag = false;
    if di == 0 {
        palupflag = true;
    }

    let r3xv = 120.0f32.to_radians();
    let r3xx = r3xv.cos();
    let r3xy = r3xv.sin();

    if di == 1 {
        let t = r3xx * *x - r3xy * *y;
        *y = r3xx * *y + r3xy * *x;
        *x = t;
    }
    if di == 2 {
        let t = r3xx * *x - (-r3xy) * *y;
        *y = r3xx * *y + (-r3xy) * *x;
        *x = t;
    }
    *z = *y * zz;
    *y = *y * yy;

    (0, coli as usize, palupflag)
}

pub fn mod4x(x: &mut f32, y: &mut f32, _z: &mut f32, duoi: bool) -> (u32, usize, bool) {
    let mut palupflag = false;
    if duoi {
        let t = -*y;
        *y = *x;
        *x = t;
        palupflag = true;
    }
    (0, 0, palupflag)
}

pub fn mod6x(x: &mut f32, y: &mut f32, z: &mut f32, duoi: bool) -> (u32, usize, bool) {
    let mut palupflag = false;
    if duoi {
        let t = -*y;
        *y = *z;
        *z = *x;
        *x = t;
        palupflag = true;
    }
    (0, 0, palupflag)
}

pub fn mod6xx(x: &mut f32, y: &mut f32, z: &mut f32, mut multi: usize) -> (u32, usize, bool) {
    if multi > 6 { multi = 0; }
    let coli = multi + 4;
    if multi > 0 {
        for _ in 0..multi {
            let t = -*y;
            *y = *z;
            *z = *x;
            *x = t;
        }
    }
    (1, coli, false)
}

pub fn mod8x(x: &mut f32, y: &mut f32, z: &mut f32, mut multi: usize) -> (u32, usize, bool) {
    multi &= 0x07;
    let coli = multi + 4;
    if (multi & 0x04) != 0 { *x = -*x; }
    if (multi & 0x02) != 0 { *y = -*y; }
    if (multi & 0x01) != 0 { *z = -*z; }
    (1, coli, false)
}

pub fn mod2x6x(x: &mut f32, y: &mut f32, z: &mut f32, duoi: bool, rnd_bool: bool) -> (u32, usize, bool) {
    let mut palupflag = false;
    if rnd_bool {
        let t = -*y;
        *y = *z;
        *z = *x;
        *x = t;
        palupflag = true;
    }
    if duoi {
        *x = -*x;
        *y = -*y;
        *z = -*z;
        palupflag = true;
    }
    (0, 0, palupflag)
}

// Bottom Plane Helpers
pub fn get_bottom_vectors(set_idx: i32) -> [(f32, f32, f32); 4] {
    let x = 0.41;
    let y = 0.45;
    if set_idx > 5 {
        [
            (-x, x, y),
            (x, x, y),
            (x, -x, y),
            (-x, -x, y),
        ]
    } else {
        [
            (-x, y, x),
            (x, y, x),
            (x, y, -x),
            (-x, y, -x),
        ]
    }
}
