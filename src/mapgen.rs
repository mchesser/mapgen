use num::Float;
use interpolate::Interpolate;
use basic2d::{Vec2, Circle, Grid};
use noise::{Seed, Brownian2, open_simplex2};

use kd_tree::KdTree;

pub struct UpperMap {
    pub elevation: Grid<f32>,
}

impl UpperMap {
    pub fn new(width: usize, height: usize) -> UpperMap {
        // !!! FIXME: Probably want to have more than one main island
        let islands = vec![
            Circle {
                center: Vec2::new((width/2) as f32, (height/2) as f32),
                radius: width as f32 / 1.8
            }
        ];

        println!("Generating island noise");
        let seed = Seed::new(113);
        let noise = Brownian2::new(open_simplex2, 5).wavelength(width as f64 / 4.5);
        let mut elevation = Grid::from_fn(width, height, |x, y| {
            noise.apply(&seed, &[x as f64, y as f64]) as f32
        });

        normalise(&mut elevation);

        println!("Generating islands");
        create_islands(&mut elevation, islands);

        println!("Randomizing elevation");
        randomize_elevation(&mut elevation);

        println!("Building coastline map");
		let coastline_map = build_coastline_map(&mut elevation);

        println!("Adjusting water depth");
        let coordinates_iter = elevation.coordinates();
        for ((x, y), tile) in coordinates_iter.zip(elevation.iter_mut())
            .filter(|&(_, &mut e)| e < 0.0)
        {
            let (x, y) = (x as f32, y as f32);
            let (land_x, land_y) = coastline_map.find_closest(&(x, y)).unwrap();

			let dist = Vec2::new(x - land_x, y - land_y).length();
            *tile *= (dist / (width as f32 * 0.5)).powf(0.3);
        }

        UpperMap {
            elevation: elevation,
        }
    }
}

/// Creates base islands for the map
fn create_islands(map: &mut Grid<f32>, islands: Vec<Circle>) {
    const SEA_LEVEL: f32 = 0.28;
    for x in (0..map.width()) {
        for y in (0..map.height()) {
            let pos = Vec2::new(x as f32, y as f32);

            // Get elevation of the highest nearby island
            let h = islands.iter().map(|&v| radial_fade(v, pos)).fold(0.0, |a, b| a.max(b));

            // Split into land and sea
            if map[(x, y)] * h < SEA_LEVEL {
                // Water
                let value = -map[(x, y)];
                map[(x, y)] = value;
            }
            else {
                // Land
                map[(x, y)] = h;
            }
        }
    }
}

/// Randomises the elevation in the islands
fn randomize_elevation(map: &mut Grid<f32>) {
    let seed = Seed::new(12347);
    let noise = Brownian2::new(open_simplex2, 8).frequency(0.02);
    let mut rand_map = Grid::from_fn(map.width(), map.height(), |x, y| {
        noise.apply(&seed, &[x as f64, y as f64]) as f32
    });
    normalise(&mut rand_map);

    for (value, &noise) in map.iter_mut().zip(rand_map.iter()) {
        if *value > 0.0 {
            *value = 0.8 * (*value).powf(0.5) * noise + 0.2 * noise;
        }
    }
}

fn build_coastline_map(map: &mut Grid<f32>) -> KdTree<(f32, f32)> {
	// Create a vector of points that represent points on the coastline.
	let coordinates_iter = map.coordinates();
	let mut points: Vec<_> = coordinates_iter
		.filter(|&p| map[p] >= 0.0 && any_surrounding(p, map, |x| x < 0.0))
		.map(|p| (p.0 as f32, p.1 as f32))
		.collect();
    println!("Coastline length: {}", points.len());
	KdTree::new(&mut points).unwrap()
}

/// Check if the any of the surrounding tiles satisfy a function
fn any_surrounding<F>((x, y): (usize, usize), map: &Grid<f32>, mut f: F) -> bool
	where F: FnMut(f32) -> bool
{
	fn get_bounds(val: usize, max: usize) -> (usize, usize) {
		(if val == 0 { val } else { val - 1 }, if val + 1 == max { val } else { val + 1 })
	}

	let (start_x, end_x) = get_bounds(x, map.width());
	let (start_y, end_y) = get_bounds(y, map.height());

	for y in (start_y .. end_y + 1) {
		for x in (start_x .. end_x + 1) {
			if f(map[(x, y)]) {
				return true;
			}
		}
	}

	false
}

//
// Utility functions
//

fn radial_fade(circle: Circle, point: Vec2<f32>) -> f32 {
    let dist = (circle.center - point).length();

    if dist > circle.radius { 0.0 }
    else { 1.0 - dist / circle.radius }
}

pub fn normalise(target: &mut Grid<f32>) {
    let mut min = target[(0, 0)];
    let mut max = target[(0, 0)];

    // FIXME: Can do this faster
    for &val in target.iter() {
        if min > val {
            min = val;
        } else if max < val{
            max = val;
        }
    }

    let factor = 1.0 / (max - min);
    for val in target.iter_mut() {
        *val = (*val - min) * factor;
    }
}

#[allow(dead_code)]
fn upscale<T>(input: &Grid<T>, scale: usize) -> Grid<T>
    where T: Interpolate
{
    Grid::from_fn(input.width() * scale, input.height() * scale, |x, y| {
        let in_x = x / scale;
        let in_y = y / scale;

        let values = [
            [input[(in_x, in_y)].clone(), input[(in_x, in_y + 1)].clone()],
            [input[(in_x + 1, in_y)].clone(), input[(in_x + 1, in_y + 1)].clone()],
        ];

        let dx = ((x % scale) as f64) / (scale as f64);
        let dy = ((y % scale) as f64) / (scale as f64);
        Interpolate::bilerp(values, dx, dy)
    })
}
