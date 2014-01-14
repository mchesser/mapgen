#[allow(dead_code)];

/// An RGB color
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8
}


///
/// Color list
///

/// Colors from http://en.wikipedia.org/wiki/Lists_of_colors
pub static AMBER:       Rgb = Rgb { r: 0xFF, g: 0x7F, b: 0x00 };
pub static APPLE_GREEN: Rgb = Rgb { r: 0x8D, g: 0xB6, b: 0x00 };
pub static AZURE:       Rgb = Rgb { r: 0x00, g: 0x7F, b: 0xFF };
pub static BEAVER:      Rgb = Rgb { r: 0x9F, g: 0x81, b: 0x70 };
pub static BLACK:       Rgb = Rgb { r: 0x00, g: 0x00, b: 0x00 };
pub static BLUE:        Rgb = Rgb { r: 0x00, g: 0x00, b: 0xFF };
pub static BRONZE:      Rgb = Rgb { r: 0xCD, g: 0x7F, b: 0x32 };
pub static BROWN:       Rgb = Rgb { r: 0x96, g: 0x4B, b: 0x00 };
pub static BUFF:        Rgb = Rgb { r: 0xF0, g: 0xDC, b: 0x82 };
pub static CAFE_NOIR:   Rgb = Rgb { r: 0x4B, g: 0x36, b: 0x21 };
pub static CHN_VIOLET:  Rgb = Rgb { r: 0x85, g: 0x60, b: 0x88 };
pub static COOL_BLACK:  Rgb = Rgb { r: 0x00, g: 0x2E, b: 0x63 };
pub static DARK_BLUE:   Rgb = Rgb { r: 0x00, g: 0x00, b: 0x8B };
pub static DARK_BROWN:  Rgb = Rgb { r: 0x65, g: 0x43, b: 0x21 };
pub static GOLD:        Rgb = Rgb { r: 0xFF, g: 0xD7, b: 0x00 };
pub static GREEN:       Rgb = Rgb { r: 0x00, g: 0xFF, b: 0x00 };
pub static LIGHT_BROWN: Rgb = Rgb { r: 0xB5, g: 0x65, b: 0x1D };
pub static ORANGE:      Rgb = Rgb { r: 0xFF, g: 0x7F, b: 0x00 };
pub static PURPLE:      Rgb = Rgb { r: 0x80, g: 0x00, b: 0x80 };
pub static RED:         Rgb = Rgb { r: 0xFF, g: 0x00, b: 0x00 };
pub static SUNGLOW:     Rgb = Rgb { r: 0xFF, g: 0xCC, b: 0x33 };
pub static WHITE:       Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
pub static YELLOW:      Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0x00 };

///
/// Color utilities
///

/// Apply linear interpolation over a color map
/// # Arguments
/// `colors` - A vector of colors to choose from
/// `x` - a value between 0.0 and 1.0 to select the color from
/// # Return
/// Returns the color after linear interpolation 
pub fn linear_gradient(colors: &[Rgb], x: f64) -> Rgb {
    assert!(x >= 0.0 && x <= 1.0);
    
    let band_width = (colors.len() - 1) as f64;

    let c1 = (x * band_width) as uint;
    let c2 = if x >= 1.0 { c1 } else { c1 + 1 };
    
    let x_new = 1.0 - (x * band_width - (c1 as f64));
        
    Rgb { 
        r: (x_new * (colors[c1].r as f64) + (1.0 - x_new) * (colors[c2].r as f64)) as u8, 
        g: (x_new * (colors[c1].g as f64) + (1.0 - x_new) * (colors[c2].g as f64)) as u8,
        b: (x_new * (colors[c1].b as f64) + (1.0 - x_new) * (colors[c2].b as f64)) as u8
    }
}
