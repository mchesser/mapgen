#[allow(dead_code)];

use std::rand;
use std::rand::Rng;
use std::iter::range_step;
use extra2::array2d::Array2D;

mod extra2;

// Generate a random field based on the midpoint displacement algorithm
pub fn random_noise(target: &mut Array2D<f32>, feature_size: uint, min_res: uint) {
	let mut rng = rand::weak_rng();
	
	// Set the initial sample points
	for x in range_step(0, target.width(), feature_size) {
		for y in range_step(0, target.height(), feature_size) {
			target.set(x, y, rng.gen::<f32>());
		}
	}	
	
	let mut step_size = feature_size / 2u;
	let mut factor = 1.0;
	
	// Apply the diamond square algorithm
	while step_size > 1 {
		let half_size = step_size / 2u;
		
		// Diamond
		for x in range_step(half_size, target.width() + step_size, step_size) {
			for y in range_step(half_size, target.height() + step_size, step_size) {
				let noise = (rng.gen::<f32>() * 2.0 - 1.0) * factor;
				let value = average_corner(target, x, y, half_size) + noise;
				
				wrap_set(target, x, y, value);
			}
		}
		
		// Square
		for x in range_step(0, target.width(), step_size) {
			for y in range_step(0, target.height(), step_size) {
				let noise = (rng.gen::<f32>() * 2.0 - 1.0) * factor;
				let value = average_side(target, x + half_size, y, half_size) + noise;
			
				wrap_set(target, x + half_size, y, value);
				
				let noise = (rng.gen::<f32>() * 2.0 - 1.0) * factor;
				let value = average_side(target, x, y + half_size, half_size) + noise;
				
				wrap_set(target, x, y + half_size, value);
			}
		}
		
		step_size /= 2;
		
		factor = 
			if step_size >= min_res { 
				factor / 2.0
			}
			else {
				0.0
			};
	}
	
	normalise(target);
}

fn wrap_get(target: &Array2D<f32>, x: uint, y: uint) -> f32 {
	target.get(x % target.width(), y % target.height())
}

fn wrap_set(target: &mut Array2D<f32>, x: uint, y: uint, value: f32) {
	let w = target.width();
	let h = target.height();
	target.set(x % w, y % h, value);
}

fn normalise(target: &mut Array2D<f32>) {
	let mut min = target.get(0, 0);
	let mut max = target.get(0, 0);
	
	for &val in target.raw().iter() {
		if min > val {
			min = val;
		} else if max < val{
			max = val;
		}
	}
	
	let factor = 1.0 / (max - min);
	for val in target.raw_mut().mut_iter() {
		*val = (*val - min) * factor;
	}
}

fn average_side(target: &mut Array2D<f32>, x: uint, y: uint, offset: uint) -> f32 {
	let left   = wrap_get(target, x - offset, y);
	let right  = wrap_get(target, x + offset, y);
	let top    = wrap_get(target, x, y - offset);
	let bottom = wrap_get(target, x, y + offset);

	(left + right + top + bottom) / 4.0
}


fn average_corner(target: &mut Array2D<f32>, x: uint, y: uint, offset: uint) -> f32 {
	let top_left     = wrap_get(target, x - offset, y - offset);
	let top_right    = wrap_get(target, x + offset, y - offset);
	let bottom_left  = wrap_get(target, x - offset, y + offset);
	let bottom_right = wrap_get(target, x + offset, y + offset);

	(top_left + top_right + bottom_left + bottom_right) / 4.0
}
