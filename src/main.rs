extern crate bitmap;

use bitmap::Bitmap;

mod math;
mod color;
mod array2d;
mod mapgen;
mod noise;

fn main() {
    static SIZE: uint = 256*8;

    let test = mapgen::UpperMap::new(SIZE, SIZE);

    println!("Saving elevation map");
    elevation_bitmap(&test, "elevation.bmp");

    println!("Saving flow map");
    flow_bitmap(&test, "flow.bmp");
}

/// Produce an elevation bitmap
fn elevation_bitmap(map: &mapgen::UpperMap, filename: &str) {
    let land_colors = [color::BEAVER, color::BUFF];
    let sea_colors = [color::AZURE, color::COOL_BLACK];

    let mut image = Bitmap::new(map.elevation.width() as i32, map.elevation.height() as i32);

    for (idx, &h) in map.elevation.iter().enumerate() {
        let x = idx as i32 % image.width();
        let y = idx as i32 / image.width();

        // Map colors above (or equal to) 0.0 to land, and below 0.0 to sea
        if h >= 0.0 {
            image.set_pixel(x, y, color::linear_gradient(land_colors, h as f64).to_tuple());
        }
        else {
            image.set_pixel(x, y, color::linear_gradient(sea_colors, -h as f64).to_tuple());
        }
    }

    match image.write_to_file(filename) {
        Ok(_) => {},
        Err(err) => fail!("Failed to write elevation bitmap to file: {}", err)
    }
}
/// Produce a flow bitmap
fn flow_bitmap(map: &mapgen::UpperMap, filename: &str) {
    let mut len_map = array2d::from_fn(map.ocean_flow.width(), map.ocean_flow.height(),
            |x, y| map.ocean_flow.get(x, y).length());

    array2d::normalise(&mut len_map);

    let mut bitmap = Bitmap::new(map.ocean_flow.width() as i32, map.ocean_flow.height() as i32);

    let land_colors = [color::BEAVER, color::BUFF];
    let sea_colors = [color::AZURE, color::BLACK];

    for (idx, (flow, elev)) in len_map.iter().zip(map.elevation.iter()).enumerate() {
        let x = idx as i32 % bitmap.width();
        let y = idx as i32 / bitmap.width();

        if *elev >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(land_colors, *elev as f64).to_tuple());
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(sea_colors, *flow as f64).to_tuple());
        }
    }

    match bitmap.write_to_file(filename) {
        Ok(_) => {},
        Err(err) => fail!("Failed to write flow bitmap to file: {}", err)
    }
}
