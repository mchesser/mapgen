use num::Float;
use interpolate::Interpolate;
use basic2d::{Vec2, Circle, Grid};
use noise::{Seed, Brownian2, perlin2};

pub struct UpperMap {
    pub elevation: Grid<f32>,
}

impl UpperMap {
    pub fn new(width: usize, height: usize) -> UpperMap {
        // !!! FIXME: Probably want to have more than one main island
        let islands = vec![
            Circle {
                center: Vec2::new((width/2) as f32, (height/2) as f32),
                radius: width as f32 / 1.3
            }
        ];

        println!("Generating island noise");
        let seed = Seed::new(212345);
        let noise = Brownian2::new(perlin2, 8).wavelength(width as f64 / 4.0);
        let mut elevation = Grid::from_fn(width, height, |x, y| {
            noise.apply(&seed, &[x as f64, y as f64]) as f32
        });

        normalise(&mut elevation);

        println!("Generating islands");
        create_islands(&mut elevation, islands);

        println!("Randomizing elevation");
        randomize_elevation(&mut elevation);

        UpperMap {
            elevation: elevation,
        }
    }
}

/// Creates base islands for the map
fn create_islands(map: &mut Grid<f32>, islands: Vec<Circle>) {
    const SEA_LEVEL: f32 = 0.32;
    for x in (0..map.width()) {
        for y in (0..map.height()) {
            let pos = Vec2::new(x as f32, y as f32);

            // Get the elevation factor of the island that affects the point the most
            let h = islands.iter().map(|&v| radial_fade(v, pos)).fold(0.0f32, |a, b| a.max(b));

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
    let noise = Brownian2::new(perlin2, 8).frequency(0.02);
    let mut rand_map = Grid::from_fn(map.width(), map.height(), |x, y| {
        noise.apply(&seed, &[x as f64, y as f64]) as f32
    });
    normalise(&mut rand_map);

    for (value, noise) in map.iter_mut().zip(rand_map.iter()) {
        if *value > 0.0 {
            *value = *noise;
        }
    }
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
