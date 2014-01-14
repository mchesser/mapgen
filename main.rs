//use extra2::array2d;
use extra2::bitmap::Bitmap;
use extra2::color;

mod extra2;
mod noise;
mod mapgen;

fn main() {
	static SIZE: uint = 256*8;
	/*
	let mut random = array2d::from_elem(SIZE, SIZE, 0f32);
	noise::random_noise(&mut random, SIZE/4, 64);
	
	let mut bitmap = Bitmap::new(SIZE, SIZE);
	
	
	let colors = [color::WHITE, color::BLACK];
	for (idx, val) in random.raw().iter().enumerate() {
		let x = idx % bitmap.width;
		let y = idx / bitmap.width;
		
		bitmap.set_pixel(x, y, color::linear_gradient(colors, *val as f64));
	}
	
	bitmap.write_to_file("test.bmp");
	*/
	
	let test = mapgen::UpperMap::new(SIZE, SIZE);
	elevation_bitmap(&test, "test.bmp");
}

fn elevation_bitmap(map: &mapgen::UpperMap, filename: &str) {
	let mut bitmap = Bitmap::new(map.width(), map.height());
	
	let land_colors = [color::BLACK, color::WHITE];
	let sea_colors = [color::AZURE, color::DARK_BLUE];
	
	for (idx, val) in map.elevation.raw().iter().enumerate() {
		let x = idx % bitmap.width;
		let y = idx / bitmap.width;
		
		if *val >= 0.0 {
			bitmap.set_pixel(x, y, color::linear_gradient(land_colors, *val as f64));
		}
		else {
			bitmap.set_pixel(x, y, color::linear_gradient(sea_colors, -*val as f64));
		}
	}
	
	bitmap.write_to_file(filename);
}
