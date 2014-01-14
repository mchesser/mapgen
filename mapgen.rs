use extra2::array2d;
use extra2::array2d::Array2D;
use extra2::vectors::Vec2;

mod extra2;
mod noise;


pub struct UpperMap {
	priv width_: uint,
	priv height_: uint,
	elevation: Array2D<f32>,
}

impl UpperMap {
	pub fn new(width: uint, height: uint) -> UpperMap {
		let mut upper_map = UpperMap {
			width_: width,
			height_: height,
			elevation: array2d::from_elem(width, height, 0f32)
		};
		
		let islands = ~[ 
			Marker { 
				center: Vec2::new((width/2) as f32, (height/2) as f32),
				radius: width as f32 / 1.3
			}
		];
		
		create_islands(&mut upper_map.elevation, islands);
		randomize_elevation(&mut upper_map.elevation);
		
		upper_map
	}
	
	pub fn width(&self) -> uint {
		self.width_
	}
	
	pub fn height(&self) -> uint {
		self.height_
	}
}

struct Marker {
	center: Vec2<f32>,
	radius: f32,
}

impl Marker {
	fn factor(&self, point: Vec2<f32>) -> f32 {
		let dist = (self.center - point).length();
		
		if dist > self.radius {
			0.0
		}
		else {
			1.0 - dist / self.radius
		}
	}
}


fn create_islands(map: &mut Array2D<f32>, islands: ~[Marker]) {
	static SEA_LEVEL: f32 = 0.32;

	let width_tmp = map.width();
	noise::random_noise(map, width_tmp / 4, 16);
	
	for x in range(0, map.width()) {
		for y in range(0, map.height()) {
			let pos = Vec2::new(x as f32, y as f32);
			
			let h = islands.iter().map(|v| v.factor(pos)).max_by(|&x| x).unwrap();			
			
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

fn randomize_elevation(map: &mut Array2D<f32>) {
	let mut rand_map = array2d::from_elem(map.width(), map.height(), 0f32);
	noise::random_noise(&mut rand_map, map.width() / 8, 0);
	
	for (value, noise) in map.raw_mut().mut_iter().zip(rand_map.raw().iter()) {
		if *value > 0.0 {
			*value = *noise;
		}
	}
}

fn simulate_ocean_flow(land_data: &mut Array2D<f32>, flow_data: &mut Array2D<Vec2<f32>>) {
	
}
