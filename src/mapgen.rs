use std::num::FloatMath;
use std::mem;
use std::rand;
use std::rand::Rng;

use wrapping2darray;
use wrapping2darray::Wrapping2DArray;

use math::vectors::Vec2;
use math::shapes::{Circle, Rect};
use math::interpolate::Interpolate;

use noise::source::{Source, RidgedMulti, Perlin};

static TAU: f32 = 2.0*3.14159265358979323;

pub struct UpperMap {
    pub elevation: Wrapping2DArray<f32>,
    pub ocean_flow: Wrapping2DArray<Vec2<f32>>,
}

impl UpperMap {
    pub fn new(width: i32, height: i32) -> UpperMap {
        let scale = 20.0 / (width as f32);

        // !!! FIXME: Probably want to have more than one main island
        let islands = vec![
            Circle {
                center: Vec2::new((width/2) as f32, (height/2) as f32),
                radius: width as f32 / 1.3
            }
        ];

        let noise_source = RidgedMulti::new().frequency(1.0 / (width as f64)).octaves(8);

        println!("Generating island noise");
        let mut elevation = Wrapping2DArray::from_fn(width, height, |x, y| {
            noise_source.get(x as f32, y as f32, 0.0)
        });
        normalise(&mut elevation);

        println!("Generating islands");
        create_islands(&mut elevation, islands);

        println!("Randomizing elevation");
        randomize_elevation(&mut elevation);

        println!("Creating lower resolution land map");
        let land_map = Wrapping2DArray::from_fn(width / 4, height / 4, |x, y| {
            let mut acc = 0.0;
            for scaled_x in range(x*4, x*4 + 4) {
                for scaled_y in range(y*4, y*4 + 4) {
                    acc += *elevation.get(scaled_x, scaled_y);
                }
            }
            acc / (4.0 * 4.0)
        });

        let noise_source = RidgedMulti::new().frequency(0.02).octaves(8);
        let ocean_flow_tmp = Wrapping2DArray::from_fn(width / 4, height / 4, |x, y| {
            Vec2::from_polar(noise_source.get(x as f32, y as f32, 0.0) * TAU / 4.0, 0.5)
        });
        let mut ocean_flow = Wrapping2DArray::from_elem(width / 4, height / 4, Vec2::zero());
        flood_fill_if_less(&mut ocean_flow, &land_map, 0.0, &ocean_flow_tmp, 0, 0);

        println!("Simulating ocean flow");
        simulate_ocean_flow(&land_map, &mut ocean_flow);

        UpperMap {
            elevation: elevation,
            ocean_flow: upscale(&ocean_flow, 4)
        }
    }
}

/// Creates base islands for the map
fn create_islands(map: &mut Wrapping2DArray<f32>, islands: Vec<Circle>) {
    const SEA_LEVEL: f32 = 0.32;
    for x in range(0, map.width()) {
        for y in range(0, map.height()) {
            let pos = Vec2::new(x as f32, y as f32);

            // Get the elevation factor of the island that affects the point the most
            let h = islands.iter().map(|v| radial_fade(*v, pos)).fold(0.0f32, |a, b| a.max(b));

            // Split into land and sea
            if *map.get(x, y) * h < SEA_LEVEL {
                // Water
                let value = -*map.get(x, y);
                *map.get_mut(x, y) = value;
            }
            else {
                // Land
                *map.get_mut(x, y) = h;
            }
        }
    }
}


/// Randomises the elevation in the islands
fn randomize_elevation(map: &mut Wrapping2DArray<f32>) {
    let noise_source = Perlin::new().frequency(0.02).octaves(8);
    let mut rand_map = Wrapping2DArray::from_fn(map.width(), map.height(), |x, y| {
        noise_source.get(x as f32, y as f32, 0.0)
    });
    normalise(&mut rand_map);

    for (value, noise) in map.iter_mut().zip(rand_map.iter()) {
        if *value > 0.0 {
            *value = *noise;
        }
    }
}


/// Simulates ocean flow, based on initial flow data and land data
fn simulate_ocean_flow(land_data: &Wrapping2DArray<f32>, flow_data: &mut Wrapping2DArray<Vec2<f32>>) {
    // A list of the possible adjacent tiles
    static ADJ: [(i32, i32); 9] =
            [(-1, -1), ( 0, -1), ( 1, -1),
             (-1,  0), ( 0,  0), ( 1,  0),
             (-1,  1), ( 0,  1), ( 1,  1)];

    let source_flow = flow_data.clone();
    let mut rng = rand::weak_rng();

    // Simulate for 25 steps
    for _ in range(0u, 25) {
        // !!! FIXME: This is not very realistic, and limits how much the flow data can change as a
        // result of other factors.
        let mut old = Wrapping2DArray::from_fn(flow_data.width(), flow_data.height(),
                |x, y| source_flow.get(x, y).scale(0.1));

        mem::swap(flow_data, &mut old);

        for x in range(0, old.width()) {
            for y in range(0, old.height()) {
                // No water on this square
                if old.get(x, y).length_sqr() == 0.0 {
                    continue;
                }
                // Moving water in ocean
                else if *land_data.get(x, y) < 0.0 {
                    let direction = old.get(x, y).unit();
                    //print!("{}", direction);

                    let offset = Vec2::new(x as f32, y as f32) + direction;
                    let water_rect = Rect { x: offset.x, y: offset.y, width: 1.0, height: 1.0 };

                    // !!! FIXME: Could be made much more efficient
                    for &(dx, dy) in ADJ.iter() {
                        let nx = x + dx;
                        let ny = y + dy;

                        let grid_rect = Rect { x: nx as f32, y: ny as f32, width: 1.0, height: 1.0 };
                        let factor = water_rect.intersect_area(&grid_rect);

                        let prev = *flow_data.get(nx, ny);
                        *flow_data.get_mut(nx, ny) = prev + old.get(x, y).scale(factor);
                    }
                }
                // Moving water on land
                else {
                    // Water is on land, determine which way the sea is to push it back out that way

                    // !!! FIXME: Doesn't seem very efficient...
                    let ocean_tiles: Vec<&(i32, i32)> = ADJ.iter().filter(|& &(dx, dy)| {
                        let nx = x + dx;
                        let ny = y + dy;

                        *land_data.get(nx, ny) < 0.0
                    }).collect();

                    // !!! FIXME: This should be handled correctly
                    if ocean_tiles.len() == 0 {
                        println!("Trapped Water");
                        continue;
                    }

                    // !!! FIXME: This doesn't look so great, and doesn't affect some things enough
                    let factor = 0.5 / ocean_tiles.len() as f32;
                    for & &(dx, dy) in ocean_tiles.iter() {
                        let nx = x + dx;
                        let ny = y + dy;
                        let mut flow = Vec2::new(dx as f32, dy as f32).unit().scale(factor);
                        flow.rotate(TAU / 10.0 * (rng.gen::<f32>() * 2.0 - 1.0));

                        let prev = *flow_data.get(nx, ny);
                        *flow_data.get_mut(nx, ny) = prev + flow;
                    }

                    // Remove the water from this tile now
                    *flow_data.get_mut(x, y) = Vec2::zero();
                }
            }
        }

        // !!! FIXME: This doesn't seem very realistic
        for val in flow_data.iter_mut() {
            *val = val.scale(0.9);
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

fn flood_fill_if_less<A, B>(target: &mut Wrapping2DArray<A>, check: &Wrapping2DArray<B>, thres: B,
    source: &Wrapping2DArray<A>, x: i32, y: i32) where A: Clone + PartialEq, B: Clone + PartialOrd
{
    if *check.get(x, y) > thres {
        return;
    }

    let mut active = vec![(x, y)];
    loop {
        let (x, y) = match active.pop() {
            Some(v) => v,
            None => break
        };

        if *check.get(x, y) <= thres && *target.get(x, y) != *source.get(x, y) {
            *target.get_mut(x, y) = source.get(x, y).clone();

            if y > 0 { active.push((x, y-1)); }
            if y+1 < check.height() { active.push((x, y+1)); }
            if x > 0 { active.push((x-1, y)); }
            if x+1 < check.width() { active.push((x+1, y)); }
        }
    }
}

pub fn normalise(target: &mut Wrapping2DArray<f32>) {
    let mut min = *target.get(0, 0);
    let mut max = *target.get(0, 0);

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

fn upscale<T: Interpolate + Clone>(input: &Wrapping2DArray<T>, scale: i32) -> Wrapping2DArray<T> {
    Wrapping2DArray::from_fn(input.width() * scale, input.height() * scale, |x, y| {
        let in_x = (x / scale) as i32;
        let in_y = (y / scale) as i32;

        let values = [
            [input.get(in_x, in_y).clone(), input.get(in_x, in_y + 1).clone()],
            [input.get(in_x + 1, in_y).clone(), input.get(in_x + 1, in_y + 1).clone()],
        ];

        let dx = ((x % scale) as f64) / (scale as f64);
        let dy = ((y % scale) as f64) / (scale as f64);
        Interpolate::bilerp(values, dx, dy)
    })
}
