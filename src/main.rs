use extra2::bitmap::Bitmap;
use extra2::color;
use extra2::array2d;

mod extra2;
mod mapgen;

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
    let mut bitmap = Bitmap::new(map.elevation.width(), map.elevation.height());

    let land_colors = [color::BEAVER, color::BUFF];
    let sea_colors = [color::AZURE, color::COOL_BLACK];

    for (idx, val) in map.elevation.iter().enumerate() {
        let x = idx % bitmap.width;
        let y = idx / bitmap.width;

        if *val >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(land_colors, *val as f64));
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(sea_colors, -*val as f64));
        }
    }

    match bitmap.write_to_file(filename) {
        Ok(_) => {},
        Err(err) => fail!("Failed to write elevation bitmap to file: {:?}", err)
    }
}

/// Produce a flow bitmap
fn flow_bitmap(map: &mapgen::UpperMap, filename: &str) {
    let mut len_map = array2d::from_fn(map.ocean_flow.width(), map.ocean_flow.height(),
            |x, y| map.ocean_flow.get(x, y).length());

    array2d::normalise(&mut len_map);

    let mut bitmap = Bitmap::new(map.ocean_flow.width(), map.ocean_flow.height());

    let land_colors = [color::BEAVER, color::BUFF];
    let sea_colors = [color::AZURE, color::BLACK];

    for (idx, (flow, elev)) in len_map.iter().zip(map.elevation.iter()).enumerate() {
        let x = idx % bitmap.width;
        let y = idx / bitmap.width;

        if *elev >= 0.0 {
            bitmap.set_pixel(x, y, color::linear_gradient(land_colors, *elev as f64));
        }
        else {
            bitmap.set_pixel(x, y, color::linear_gradient(sea_colors, *flow as f64));
        }
    }

    match bitmap.write_to_file(filename) {
        Ok(_) => {},
        Err(err) => fail!("Failed to write flow bitmap to file: {:?}", err)
    }
}
