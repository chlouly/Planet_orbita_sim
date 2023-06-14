pub const DIMENSION : usize = 2;
pub const DT : f32 = 1.0;
pub const G : f32 = 6.674e-11;


pub fn sub(a : [f32; DIMENSION], b : [f32; DIMENSION]) -> [f32; DIMENSION] {
    //performs a - b = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] - b[i];
    }

    out
}

pub fn add(a : [f32; DIMENSION], b : [f32; DIMENSION]) -> [f32; DIMENSION] {
    //performs a + b = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] + b[i];
    }

    out
}

pub fn mult(a : [f32; DIMENSION], factor : f32) -> [f32; DIMENSION] {
    //performs a * factor = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] * factor;
    }

    out
}

pub fn find_mag(a : [f32; DIMENSION]) -> f32 {
    //finds the magnitude of a
    let mut out : f32 = 0.0;
    for i in 0..DIMENSION {
        out += a[i] * a[i];
    }

    out.sqrt()
}