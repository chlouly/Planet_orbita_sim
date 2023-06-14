use crate::constants::*;


pub fn sub(a : [f64; DIMENSION], b : [f64; DIMENSION]) -> [f64; DIMENSION] {
    //performs a - b = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] - b[i];
    }

    out
}

pub fn add(a : [f64; DIMENSION], b : [f64; DIMENSION]) -> [f64; DIMENSION] {
    //performs a + b = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] + b[i];
    }

    out
}

pub fn mult(a : [f64; DIMENSION], factor : f64) -> [f64; DIMENSION] {
    //performs a * factor = out
    let mut out = [0.0; DIMENSION];
    for i in 0..DIMENSION {
        out[i] = a[i] * factor;
    }

    out
}

pub fn find_mag(a : [f64; DIMENSION]) -> f64 {
    //finds the magnitude of a
    let mut out : f64 = 0.0;
    for i in 0..DIMENSION {
        out += a[i] * a[i];
    }

    out.sqrt()
}