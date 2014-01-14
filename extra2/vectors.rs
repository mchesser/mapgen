#[allow(dead_code)];

use std::fmt;
use std::num::{zero, one, sqrt, sin_cos, atan2};

/// A 2-dimensional vector.
#[deriving(Eq, Clone, Zero)]
pub struct Vec2<T> { 
    x: T,
    y: T 
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T: Primitive + Clone> Vec2<T> {
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
        let (sin_a, cos_a) = sin_cos(angle);
        Vec2::new(mag * cos_a, mag * sin_a)
    }
    
    pub fn length_sqr(&self) -> T {
        self.dot(self)
    }
    
    pub fn length(&self) -> T {
        sqrt(self.length_sqr())
    }
    
    pub fn normalize(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
    } 
    
    pub fn unit(&self) -> Vec2<T> {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }
    
    pub fn rotate(&mut self, angle: T) {
        let (cos_a, sin_a) = sin_cos(angle);
        let (old_x, old_y) = (self.x.clone(), self.y.clone());
        self.x = old_x*cos_a - old_y*sin_a;
        self.y = old_x*sin_a + old_y*cos_a;
    }
    
    pub fn angle(&self) -> T {
        atan2(self.x.clone(), self.y.clone())
    }
}

impl<T: Mul<T, T>> Vec2<T> {
    pub fn scale(&self, scalar: T) -> Vec2<T> {
        Vec2::new(self.x * scalar, self.y * scalar)
    }   
}

impl<T: fmt::Default> ToStr for Vec2<T> {
    fn to_str(&self) -> ~str {
        format!("[{}, {}]", self.x, self.y)
    }
}
