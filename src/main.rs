#![feature(slicing_syntax)]

extern crate bitmap;
extern crate noise;

use std::io::IoResult;
use bitmap::Bitmap;
use wrapping2darray::Wrapping2DArray;

mod math;
mod color;
mod wrapping2darray;
mod mapgen;

fn main() {
    const SIZE: i32 = 256*8;
    let test = mapgen::UpperMap::new(SIZE, SIZE);

    println!("Saving elevation map");
    let _ = elevation_bitmap(&test, "elevation.bmp");

    println!("Saving flow map");
    let _ = flow_bitmap(&test, "flow.bmp");
}

/// Produce an elevation bitmap
fn elevation_bitmap(map: &mapgen::UpperMap, filename: &str) -> IoResult<()> {
    let land_colors = [color::consts::BEAVER, color::consts::BUFF];
    let sea_colors = [color::consts::AZURE, color::consts::COOL_BLACK];

    let mut image = Bitmap::new(map.elevation.width() as i32, map.elevation.height() as i32);

    for (idx, &h) in map.elevation.iter().enumerate() {
        let x = idx as i32 % image.width();
        let y = idx as i32 / image.width();

        // Map colors above (or equal to) 0.0 to land, and below 0.0 to sea
        if h >= 0.0 {
            image.set_pixel(x, y, color::linear_gradient(&land_colors, h as f64).to_tuple());
        }
        else {
            image.set_pixel(x, y, color::linear_gradient(&sea_colors, -h as f64).to_tuple());
        }
    }

    image.write_to_file(filename)
}
/// Produce a flow bitmap
fn flow_bitmap(map: &mapgen::UpperMap, filename: &str) -> IoResult<()> {
    let mut len_map = Wrapping2DArray::from_fn(map.ocean_flow.width(), map.ocean_flow.height(),
            |x, y| map.ocean_flow.get(x, y).length());
    mapgen::normalise(&mut len_map);

    let mut bitmap = Bitmap::new(map.ocean_flow.width() as i32, map.ocean_flow.height() as i32);

    let land_colors = [color::consts::BEAVER, color::consts::BUFF];
    let sea_colors = [color::consts::AZURE, color::consts::BLACK];

    for (idx, (flow, elev)) in len_map.iter().zip(map.elevation.iter()).enumerate() {
        let x = idx as i32 % bitmap.width();
        let y = idx as i32 / bitmap.width();

        if *elev >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(&land_colors, *elev as f64).to_tuple());
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(&sea_colors, *flow as f64).to_tuple());
        }
    }

    bitmap.write_to_file(filename)
}
