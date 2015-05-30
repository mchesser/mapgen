use std::fmt;
use std::ops::{Add, Sub, Mul};

use num::{One, Zero};
use num::traits::Float;
use math::interpolate::Interpolate;

/// A 2-dimensional vector.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x: x, y: y }
    }
}

impl<T> Vec2<T> {
    /// Create a new vector of length 0
    pub fn zero() -> Vec2<T> where T: Zero {
        Vec2 { x: Zero::zero(), y: Zero::zero() }
    }

    /// Create the unit vector in the x direction
    pub fn unit_x() -> Vec2<T> where T: Zero + One {
        Vec2 { x: One::one(), y: Zero::zero() }
    }

    /// Create the unit vector in the y direction
    pub fn unit_y() -> Vec2<T> where T: Zero + One {
        Vec2 { x: Zero::zero(), y: One::one() }
    }

    /// Create a new vector from polar coordinates
    pub fn from_polar(angle: T, mag: T) -> Vec2<T> where T: Float {
        let (sin_a, cos_a) = angle.sin_cos();
        Vec2::new(mag * cos_a, mag * sin_a)
    }

    /// Calculate the dot product between this and another vector
    pub fn dot(&self, other: &Vec2<T>) -> T where T: Float {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Calculates the length squared of the vector. Avoids taking a square root.
    pub fn length_sqr(&self) -> T where T: Float {
        self.dot(self)
    }

    /// Calculates the length of the vector
    pub fn length(&self) -> T where T: Float {
        self.length_sqr().sqrt()
    }

    /// Normalises the vector
    pub fn normalize(&mut self) where T: Float {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
    }

    /// Creates a unit vector in the direction of the vector
    pub fn unit(&self) -> Vec2<T> where T: Float {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }

    /// Rotates a vector by a specified angle
    pub fn rotate(&mut self, angle: T) where T: Float {
        let (cos_a, sin_a) = angle.sin_cos();
        let (old_x, old_y) = (self.x.clone(), self.y.clone());
        self.x = old_x*cos_a - old_y*sin_a;
        self.y = old_x*sin_a + old_y*cos_a;
    }

    /// Gets the angle of the vector
    pub fn angle(&self) -> T where T: Float {
        self.x.atan2(self.y)
    }
}

impl<T> Add for Vec2<T> where T: Add<Output=T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Vec2<T> where T: Sub<Output=T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Vec2<T> where T: Copy + Mul<Output=T> {
    /// Creates a new vector equal to the vector scaled by a scalar value
    pub fn scale(&self, scalar: T) -> Vec2<T> {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    /// Provides a string representation of the vector
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T: Interpolate> Interpolate for Vec2<T> {
    fn lerp(v: [Vec2<T>; 2], x: f64) -> Vec2<T> {
        Vec2 {
            x: Interpolate::lerp([v[0].x.clone(), v[1].x.clone()], x),
            y: Interpolate::lerp([v[0].y.clone(), v[1].y.clone()], x)
        }
    }
}
