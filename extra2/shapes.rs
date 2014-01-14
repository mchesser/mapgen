#[allow(dead_code)];

use extra2::vectors::Vec2;
use std::num::{min, max};

pub struct Circle {
    center: Vec2<f32>,
    radius: f32,
}

pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32
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
    
    pub fn move_vec(&mut self, vec: Vec2<f32>) {
        self.x += vec.x;
        self.y += vec.y;
    }
    
    pub fn intersect_area(&self, other: &Rect) -> f32 {
        let x_intersect = min(self.right(), other.right()) - max(self.left(), other.left());
        let y_intersect = min(self.bottom(), other.bottom()) - max(self.top(), other.top());        
        
        if x_intersect < 0.0 || y_intersect < 0.0 {
            0.0
        }
        else {
            x_intersect * y_intersect
        }       
    }       
}
