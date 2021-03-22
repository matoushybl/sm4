pub fn fabs(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

pub fn fmaxf(a: f32, b: f32) -> f32 {
    if a > b {
        return a;
    }
    b
}

pub fn fminf(a: f32, b: f32) -> f32 {
    if b > a {
        return a;
    }
    b
}
