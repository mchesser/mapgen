use std::cmp::{partial_min, partial_max};
use math::vectors::Vec2;

/// Circle structure, with center and radius
#[derive(Copy, Clone)]
pub struct Circle {
    pub center: Vec2<f32>,
    pub radius: f32,
}

/// Rectangle structure
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Rect {
    pub fn top_left(&self) -> Vec2<f32> {
        Vec2::new(self.top(), self.left())
    }

    pub fn top_right(&self) -> Vec2<f32> {
        Vec2::  new(self.x + self.width, self.y)
    }

    pub fn bottom_left(&self) -> Vec2<f32> {
        Vec2::new(self.x, self.y + self.height)
    }

    pub fn bottom_right(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width, self.y + self.height)
    }

    pub fn center(&self) -> Vec2<f32> {
        Vec2::new(self.x + self.width/2.0, self.y + self.height/2.0)
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Moves the rectangle by a specified vector
    pub fn move_vec(&mut self, vec: Vec2<f32>) {
        self.x += vec.x;
        self.y += vec.y;
    }

    /// Calculate the intersection area of two rectangles
    pub fn intersect_area(&self, other: &Rect) -> f32 {
        let x_intersect = partial_min(self.right(), other.right()).unwrap() -
            partial_max(self.left(), other.left()).unwrap();

        let y_intersect = partial_min(self.bottom(), other.bottom()).unwrap() -
            partial_max(self.top(), other.top()).unwrap();

        if x_intersect < 0.0 || y_intersect < 0.0 {
            0.0
        }
        else {
            x_intersect * y_intersect
        }
    }
}
