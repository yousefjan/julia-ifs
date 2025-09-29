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


