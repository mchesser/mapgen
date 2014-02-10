use extra2::array2d;
use extra2::array2d::Array2D;
use extra2::vectors::Vec2;
use extra2::shapes::{Circle, Rect};
use extra2::interpolate::Interpolate;
use std::util;
use std::rand;
use std::rand::Rng;

mod noise;

// !!! FIXME: Use proper constants from Rust after numbers library is more stable
static TAU: f32 = 2.0*3.14159265358979323;

pub struct UpperMap {
    elevation: Array2D<f32>,
    ocean_flow: Array2D<Vec2<f32>>,
}

impl UpperMap {
    pub fn new(width: uint, height: uint) -> UpperMap {

        // !!! FIXME: Probably want to have more than one main island
        let islands = box [
            Circle {
                center: Vec2::new((width/2) as f32, (height/2) as f32),
                radius: width as f32 / 1.3
            }
        ];

        println!("Generating island noise");
        let mut elevation = noise::random_noise(width, height, width / 4, 16);

        println!("Generating islands");
        create_islands(&mut elevation, islands);

        println!("Randomizing elevation");
        randomize_elevation(&mut elevation);

        println!("Creating lower resolution land map");
        let land_map = array2d::from_fn(width/4, height/4, |x, y| {
            let mut acc = 0.0;
            for x_ in range(x*4, x*4 + 4) {
                for y_ in range(y*4, y*4 + 4) {
                    acc += elevation.get(x_, y_);
                }
            }
            acc / (4.0 * 4.0)
        });

        let tmp_map = noise::random_noise(width/4, height/4, width/8, 0);

        println!("Creating ocean flow base");
        // !!! FIXME: This vector shouldn't be allocated
        // !!! SOLUTION: Perform transformations in flood fill?
        let ocean_flow_tmp = array2d::from_fn(width/4, height/4, |x, y| {
                Vec2::from_polar(tmp_map.get(x, y) * TAU / 4.0, 1.0)
        });
        let mut ocean_flow = array2d::from_elem(width/4, height/4, Vec2::zero());
        flood_fill_if_less(&mut ocean_flow, &land_map, 0.0, &ocean_flow_tmp, 0, 0);

        println!("Simulating ocean flow");
        simulate_ocean_flow(&land_map, &mut ocean_flow);

        // !!! FIXME: Maps are lower resolution than input amount
        // !!! SOLUTION: Up-scale using some sort of interpolation.
        UpperMap {
            elevation: elevation,
            ocean_flow: upscale(&ocean_flow, 4)
        }
    }
}

/// Creates base islands for the map
fn create_islands(map: &mut Array2D<f32>, islands: ~[Circle]) {
    static SEA_LEVEL: f32 = 0.32;

    for x in range(0, map.width()) {
        for y in range(0, map.height()) {
            let pos = Vec2::new(x as f32, y as f32);

            // Get the elevation factor of the island that affects the point the most
            let h = islands.iter().map(|v| radial_fade(*v, pos)).max_by(|&x| x).unwrap();

            // Split into land and sea
            if map.get(x, y) * h < SEA_LEVEL {
                // Water
                let value = -map.get(x, y);
                map.set(x, y, value);
            }
            else {
                // Land
                map.set(x, y, h);
            }
        }
    }
}

/// Randomises the elevation in the islands
fn randomize_elevation(map: &mut Array2D<f32>) {
    let rand_map = noise::random_noise(map.width(), map.height(), map.width() / 8, 0);

    for (value, noise) in map.mut_iter().zip(rand_map.iter()) {
        if *value > 0.0 {
            *value = *noise;
        }
    }
}

/// Simulates ocean flow, based on initial flow data and land data
fn simulate_ocean_flow(land_data: &Array2D<f32>, flow_data: &mut Array2D<Vec2<f32>>) {
    // A list of the possible adjacent tiles
    static ADJ: [(int, int),..9] =
            [(-1, -1), ( 0, -1), ( 1, -1),
             (-1,  0), ( 0,  0), ( 1,  0),
             (-1,  1), ( 0,  1), ( 1,  1)];

    let source_flow = flow_data.clone();
    let mut rng = rand::weak_rng();

    // Simulate for 25 steps
    for _ in range(0, 25) {
        // !!! FIXME: This is not very realistic, and limits how much the flow data can change as a
        // result of other factors.
        let mut old = array2d::from_fn(flow_data.width(), flow_data.height(),
                |x, y| source_flow.get(x, y).scale(0.1));

        util::swap(flow_data, &mut old);

        for x in range(0, old.width()) {
            for y in range(0, old.height()) {
                // No water on this square
                if old.get(x, y).length_sqr() == 0.0 {
                    continue;
                }
                // Moving water in ocean
                else if land_data.get(x, y) < 0.0 {
                    let direction = old.get(x, y).unit();

                    let offset = Vec2::new(x as f32, y as f32) + direction;
                    let water_rect = Rect { x: offset.x, y: offset.y, width: 1.0, height: 1.0 };

                    // !!! FIXME: Could be made much more efficient
                    for &(dx, dy) in ADJ.iter() {
                        let nx = (x as int + dx);
                        let ny = (y as int + dy);

                        let grid_rect = Rect { x: nx as f32, y: ny as f32, width: 1.0, height: 1.0 };
                        let factor = water_rect.intersect_area(&grid_rect);

                        let prev = array2d::wrap_get(flow_data, nx, ny);
                        array2d::wrap_set(flow_data, nx, ny, prev + old.get(x, y).scale(factor));
                    }
                }
                // Moving water on land
                else {
                    // Water is on land, determine which way the sea is to push it back out that way

                    // !!! FIXME: Doesn't seem very efficient...
                    let ocean_tiles: ~[&(int, int)] = ADJ.iter().filter(|& &(dx, dy)| {
                        let nx = (x as int + dx);
                        let ny = (y as int + dy);

                        array2d::wrap_get(land_data, nx, ny) < 0.0
                    }).collect();

                    // !!! FIXME: This should be handled correctly
                    if ocean_tiles.len() == 0 {
                        println!("Trapped Water");
                        continue;
                    }

                    // !!! FIXME: This doesn't look so great, and doesn't affect some things enough
                    let factor = 0.5 / ocean_tiles.len() as f32;
                    for & &(dx, dy) in ocean_tiles.iter() {
                        let nx = (x as int + dx);
                        let ny = (y as int + dy);
                        let mut flow = Vec2::new(dx as f32, dy as f32).unit().scale(factor);
                        flow.rotate(TAU / 10.0 * (rng.gen::<f32>() * 2.0 - 1.0));

                        let prev = array2d::wrap_get(flow_data, nx, ny);
                        array2d::wrap_set(flow_data, nx, ny, prev + flow);
                    }

                    // Remove the water from this tile now
                    flow_data.set(x, y, Vec2::zero());
                }
            }
        }

        // !!! FIXME: This doesn't seem very realistic
        for val in flow_data.mut_iter() {
            *val = val.scale(0.9);
        }
    }
}


///
/// Utility functions
///
fn radial_fade(circle: Circle, point: Vec2<f32>) -> f32 {
    let dist = (circle.center - point).length();

    if dist > circle.radius {
        0.0
    }
    else {
        1.0 - dist / circle.radius
    }
}

fn flood_fill_if_less<A: Clone + Eq, B: Clone + Ord>(target: &mut Array2D<A>, check: &Array2D<B>,
        thres: B, source: &Array2D<A>, x: uint, y: uint) {

    if check.get(x, y) > thres {
        return;
    }

    let mut active = box [(x, y)];
    loop {
        let (x, y) = match active.pop() {
            Some(v) => v,
            None => break
        };

        if check.get(x, y) <= thres && target.get(x, y) != source.get(x, y) {
            target.set(x, y, source.get(x, y));

            if y > 0 { active.push((x, y-1)); }
            if y+1 < check.height() { active.push((x, y+1)); }
            if x > 0 { active.push((x-1, y)); }
            if x+1 < check.width() { active.push((x+1, y)); }
        }
    }
}

fn upscale<T: Interpolate>(input: &Array2D<T>, scale: uint) -> Array2D<T> {
    array2d::from_fn(input.width()*scale, input.height()*scale, |x, y| {
        let in_x = (x/scale) as int;
        let in_y = (y/scale) as int;

        let values = [
            [array2d::wrap_get(input, in_x, in_y), array2d::wrap_get(input, in_x, in_y+1)],
            [array2d::wrap_get(input, in_x+1, in_y), array2d::wrap_get(input, in_x+1, in_y+1)],
        ];

        let dx = ((x % scale) as f64) / (scale as f64);
        let dy = ((y % scale) as f64) / (scale as f64);
        Interpolate::bilerp(values, dx, dy)
    })
}
