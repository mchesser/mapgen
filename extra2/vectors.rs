#[allow(dead_code)];

use std::fmt;
use std::num::{zero, one, sqrt};

/// A 2-dimensional vector.
#[deriving(Eq, Clone, Zero)]
pub struct Vec2<T> { 
	x: T,
	y: T 
}

impl<T: Primitive + Clone> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2 { x: x, y: y }
	}

	pub fn zero() -> Vec2<T> {
		Vec2 { x: zero(), y: zero() }
	}
	
	pub fn unit_x() -> Vec2<T> {
		Vec2 { x: one(), y: zero() }
	}
	
	pub fn unit_y() -> Vec2<T> {
		Vec2 { x: zero(), y: one() }
	}
	
	pub fn dot(&self, other: &Vec2<T>) -> T {
		(self.x * other.x) + (self.y * other.y)
	}
}

impl<T: Primitive + Clone> Add<Vec2<T>, Vec2<T>> for Vec2<T> {
	fn add(&self, _rhs: &Vec2<T>) -> Vec2<T> {
		Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
	}
}

impl<T: Primitive + Clone> Sub<Vec2<T>, Vec2<T>> for Vec2<T> {
	fn sub(&self, _rhs: &Vec2<T>) -> Vec2<T> {
		Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
	}
}


impl<T: Float> Vec2<T> {
	pub fn from_polar(angle: T, mag: T) -> Vec2<T> {
		Vec2::new(mag * cos(angle), mag * sin(angle))
	}
	
	pub fn length_sqr(&self) -> T {
		self.dot(self)
	}
	
	pub fn length(&self) -> T {
		sqrt(self.length_sqr())
	}
}

impl<T: fmt::Default> ToStr for Vec2<T> {
    fn to_str(&self) -> ~str {
        format!("[{}, {}]", self.x, self.y)
    }
}
