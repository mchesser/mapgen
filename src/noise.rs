#[allow(dead_code)];

use std::rand;
use std::rand::Rng;
use std::iter::range_step;
use extra2::array2d::Array2D;
use extra2::array2d;

mod extra2;

/// Generate a random field based on the midpoint displacement algorithm
/// # Arguments
/// `width` - The width of the final array
/// `height` - The height of the final array
/// `feature_size` - The feature size of the array
/// `min_res` - The minimum resolution to go to
/// # Return
/// A new array of random numbers
pub fn random_noise(width: uint, height: uint, feature_size: uint, min_res: uint) -> Array2D<f32> {
    //
    // Utility functions
    //
    fn average_side(target: &Array2D<f32>, x: uint, y: uint, offset: uint) -> f32 {
        let left   = array2d::wrap_get(target, x as int - offset as int, y as int);
        let right  = array2d::wrap_get(target, x as int + offset as int, y as int);
        let top    = array2d::wrap_get(target, x as int, y as int - offset as int);
        let bottom = array2d::wrap_get(target, x as int, y as int + offset as int);

        (left + right + top + bottom) / 4.0
    }

    fn average_corner(target: &Array2D<f32>, x: uint, y: uint, offset: uint) -> f32 {
        let top_left     = array2d::wrap_get(target, x as int - offset as int, y as int - offset as int);
        let top_right    = array2d::wrap_get(target, x as int + offset as int, y as int - offset as int);
        let bottom_left  = array2d::wrap_get(target, x as int - offset as int, y as int + offset as int);
        let bottom_right = array2d::wrap_get(target, x as int + offset as int, y as int + offset as int);

        (top_left + top_right + bottom_left + bottom_right) / 4.0
    }
    
    
    //
    // Main algorithm
    //
    
    let mut target = array2d::from_elem(width, height, 0.0_f32);
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
                let value = average_corner(&target, x, y, half_size) + noise;
                
                array2d::wrap_set(&mut target, x as int, y as int, value);
            }
        }
        
        // Square
        for x in range_step(0, target.width(), step_size) {
            for y in range_step(0, target.height(), step_size) {
                let noise = (rng.gen::<f32>() * 2.0 - 1.0) * factor;
                let value = average_side(&target, x + half_size, y, half_size) + noise;
            
                array2d::wrap_set(&mut target, x as int + half_size as int, y as int, value);
                
                let noise = (rng.gen::<f32>() * 2.0 - 1.0) * factor;
                let value = average_side(&target, x, y + half_size, half_size) + noise;
                
                array2d::wrap_set(&mut target, x as int, y as int + half_size as int, value);
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
    
    array2d::normalise(&mut target);
    target
}