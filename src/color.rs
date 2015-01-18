use math::interpolate::Interpolate;

/// An RGB color
#[derive(Copy, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb {
    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Interpolate for Rgb {
    fn lerp(v: [Rgb; 2], x: f64) -> Rgb {
        Rgb {
            r: Interpolate::lerp([v[0].r, v[1].r], x),
            g: Interpolate::lerp([v[0].g, v[1].g], x),
            b: Interpolate::lerp([v[0].b, v[1].b], x)
        }
    }
}

#[allow(dead_code)]
pub mod consts {
    use super::Rgb;

    /// Colors from http://en.wikipedia.org/wiki/Lists_of_colors
    pub const AMBER:       Rgb = Rgb { r: 0xFF, g: 0x7F, b: 0x00 };
    pub const APPLE_GREEN: Rgb = Rgb { r: 0x8D, g: 0xB6, b: 0x00 };
    pub const AZURE:       Rgb = Rgb { r: 0x00, g: 0x7F, b: 0xFF };
    pub const BEAVER:      Rgb = Rgb { r: 0x9F, g: 0x81, b: 0x70 };
    pub const BLACK:       Rgb = Rgb { r: 0x00, g: 0x00, b: 0x00 };
    pub const BLUE:        Rgb = Rgb { r: 0x00, g: 0x00, b: 0xFF };
    pub const BRONZE:      Rgb = Rgb { r: 0xCD, g: 0x7F, b: 0x32 };
    pub const BROWN:       Rgb = Rgb { r: 0x96, g: 0x4B, b: 0x00 };
    pub const BUFF:        Rgb = Rgb { r: 0xF0, g: 0xDC, b: 0x82 };
    pub const CAFE_NOIR:   Rgb = Rgb { r: 0x4B, g: 0x36, b: 0x21 };
    pub const CHN_VIOLET:  Rgb = Rgb { r: 0x85, g: 0x60, b: 0x88 };
    pub const COOL_BLACK:  Rgb = Rgb { r: 0x00, g: 0x2E, b: 0x63 };
    pub const DARK_BLUE:   Rgb = Rgb { r: 0x00, g: 0x00, b: 0x8B };
    pub const DARK_BROWN:  Rgb = Rgb { r: 0x65, g: 0x43, b: 0x21 };
    pub const GOLD:        Rgb = Rgb { r: 0xFF, g: 0xD7, b: 0x00 };
    pub const GREEN:       Rgb = Rgb { r: 0x00, g: 0xFF, b: 0x00 };
    pub const LIGHT_BROWN: Rgb = Rgb { r: 0xB5, g: 0x65, b: 0x1D };
    pub const ORANGE:      Rgb = Rgb { r: 0xFF, g: 0x7F, b: 0x00 };
    pub const PURPLE:      Rgb = Rgb { r: 0x80, g: 0x00, b: 0x80 };
    pub const RED:         Rgb = Rgb { r: 0xFF, g: 0x00, b: 0x00 };
    pub const SUNGLOW:     Rgb = Rgb { r: 0xFF, g: 0xCC, b: 0x33 };
    pub const WHITE:       Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
    pub const YELLOW:      Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0x00 };
}

//
// Color utilities
//

/// Apply linear interpolation over a color map
///
/// # Arguments
/// `colors` - A vector of colors to choose from
/// `x` - a value between 0.0 and 1.0 to select the color from
pub fn linear_gradient(colors: &[Rgb], x: f64) -> Rgb {
    assert!(x >= 0.0 && x <= 1.0);
    assert!(colors.len() >= 2);

    if x == 0.0 {
        match colors.first() {
            Some(&c) => return c,
            None => unreachable!()
        }
    }
    else if x == 1.0 {
        match colors.last() {
            Some(&c) => return c,
            None => unreachable!()
        }
    }

    let band_width = (colors.len() - 1) as f64;
    let c1 = (x * band_width) as usize;
    let c2 = c1 + 1;
    let x_new = 1.0 - (x * band_width - (c1 as f64));
    Interpolate::lerp([colors[c2], colors[c1]], x_new)
}
