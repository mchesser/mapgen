pub trait Interpolate: Clone {
    fn bilerp(v: [[Self; 2]; 2], x: f64, y: f64) -> Self {
        let v0 = Interpolate::lerp([v[0][0].clone(), v[0][1].clone()], y);
        let v1 = Interpolate::lerp([v[1][0].clone(), v[1][1].clone()], y);
        Interpolate::lerp([v0, v1], x)
    }
    fn lerp(v: [Self; 2], x: f64) -> Self;
}

impl Interpolate for f64 {
    fn bilerp(v: [[f64; 2]; 2], x: f64, y: f64) -> f64 {
        v[0][0]*(1.0-x)*(1.0-y) + v[1][0]*x*(1.0-y) + v[0][1]*(1.0-x)*y + v[1][1]*x*y
    }

    fn lerp(v: [f64; 2], x: f64) -> f64 {
        v[0] + x * (v[1] - v[0])
    }
}

impl Interpolate for f32 {
    fn bilerp(v: [[f32; 2]; 2], x: f64, y: f64) -> f32 {
        let x = x as f32;
        let y = y as f32;
        (v[0][0]*(1.0-x)*(1.0-y) + v[1][0]*x*(1.0-y) + v[0][1]*(1.0-x)*y + v[1][1]*x*y)
    }

    fn lerp(v: [f32; 2], x: f64) -> f32 {
        let x = x as f32;
        v[0] + x * (v[1] - v[0])
    }
}

impl Interpolate for u8 {
    fn lerp(v: [u8; 2], x: f64) -> u8 {
        Interpolate::lerp([v[0] as f64, v[1] as f64], x) as u8
    }

    fn bilerp(v: [[u8; 2]; 2], x: f64, y: f64) -> u8 {
        let array = [
            [v[0][0] as f64, v[0][1] as f64],
            [v[1][0] as f64, v[1][1] as f64],
        ];
        Interpolate::bilerp(array, x, y) as u8
    }
}
