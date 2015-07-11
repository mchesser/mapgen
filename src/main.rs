extern crate interpolate;
extern crate basic2d;
extern crate bitmap;
extern crate noise;
extern crate rand;
extern crate num;

use std::io;
use std::fs::File;

use bitmap::Bitmap;
use color::reduced_gradient;

mod color;
mod mapgen;
mod kd_tree;

fn main() {
    const SIZE: usize = 256*2;
    let test = mapgen::UpperMap::new(SIZE, SIZE);

    println!("Saving elevation map");
    elevation_bitmap(&test, "elevation.bmp").unwrap();
}

/// Produce an elevation bitmap
fn elevation_bitmap(map: &mapgen::UpperMap, filename: &str) -> io::Result<()> {
    let land_colors = [color::consts::BEAVER, color::consts::BUFF];
    let sea_colors = [color::consts::SKY_BLUE, color::consts::AZURE, color::consts::COOL_BLACK];

    let mut bitmap = Bitmap::new(map.elevation.width() as i32, map.elevation.height() as i32);

    for (idx, &h) in map.elevation.iter().enumerate() {
        let x = idx as i32 % bitmap.width();
        let y = idx as i32 / bitmap.width();

        // Map colors above (or equal to) 0.0 to land, and below 0.0 to sea
        if h >= 0.0 {
            bitmap.set_pixel(x, y, reduced_gradient(&land_colors, h as f64, 12.0).to_tuple());
        }
        else {
            bitmap.set_pixel(x, y, reduced_gradient(&sea_colors, -h as f64, 15.0).to_tuple());
        }
    }
    
    let mut file = try!(File::create(filename));
    bitmap.write(&mut file)
}

