#[allow(dead_code)];

use std::vec;

pub struct Array2D<T> {
    priv width_: uint,
    priv height_: uint,
    priv data: ~[T]
}

impl<T: Clone> Array2D<T> { 
    #[inline]
    pub fn height(&self) -> uint {
        self.height_
    }
    
    #[inline]
    pub fn width(&self) -> uint {
        self.width_
    }
    
    #[inline]
    pub fn get(&self, x: uint, y: uint) -> T {
        if x > self.width() || y > self.height() { 
            fail!(
                format!("Index out of bounds, x: {}, y: {}, width: {} height: {}",
                    x,
                    y,
                    self.width(),
                    self.height())
            );
        }
        
        self.data[(x + y * self.width()) as int].clone()
    }
    
    #[inline]
    pub fn set(&mut self, x: uint, y: uint, value: T) {
        if x > self.width() || y > self.height() { 
            fail!(
                format!("Index out of bounds, x: {}, y: {}, width: {} height: {}",
                    x,
                    y,
                    self.width(),
                    self.height())
            );
        }
        
        self.data[(x + y * self.width()) as int] = value;
    }
    
    pub fn iter<'r>(&'r self) -> vec::VecIterator<'r, T> {
        self.data.iter()
    }
    
    pub fn mut_iter<'r>(&'r mut self) -> vec::VecMutIterator<'r, T> {
        self.data.mut_iter()
    }
}

impl<T: Clone> Clone for Array2D<T> {
        fn clone(&self) -> Array2D<T> {
            Array2D {
                width_:  self.width(),
                height_: self.height(),
                data:    self.data.clone()
            }
    }
}


pub fn from_fn<T>(width: uint, height: uint, op: |uint, uint| -> T) -> Array2D<T> { 
    Array2D {
        width_: width,
        height_: height,
        data: vec::from_fn(width*height, |i| op(i % width, i / width)),
    }
}

pub fn from_elem<T:Clone>(width: uint, height: uint, elem: T) -> Array2D<T> {
    Array2D {
        width_: width,
        height_: height,
        data: vec::from_elem(width * height, elem.clone())
    }
}

pub fn from_raw<T>(width: uint, height: uint, raw: ~[T]) -> Array2D<T> {
    if width * height != raw.len() {
        fail!("Raw array of invalid length");
    }
    
    Array2D {
        width_: width,
        height_: height,
        data: raw
    }   
}

///
/// Utility Functions
///

pub fn normalise(target: &mut Array2D<f32>) {
    let mut min = target.get(0, 0);
    let mut max = target.get(0, 0);
    
    for &val in target.iter() {
        if min > val {
            min = val;
        } else if max < val{
            max = val;
        }
    }
    
    let factor = 1.0 / (max - min);
    for val in target.mut_iter() {
        *val = (*val - min) * factor;
    }
}

pub fn wrap_get<T: Clone>(target: &Array2D<T>, x: int, y: int) -> T {
    let x = 
        if x < 0 { (target.width() as int + x % target.width() as int) as uint }
        else { x as uint % target.width() };
    
    let y =
        if y < 0 { (target.height() as int + y % target.height() as int) as uint }
        else { y as uint % target.height() };
    
    target.get(x, y)
}

pub fn wrap_set<T: Clone>(target: &mut Array2D<T>, x: int, y: int, value: T) {
    let x = 
        if x < 0 { (target.width() as int + x % target.width() as int) as uint }
        else { x as uint % target.width() };
    
    let y =
        if y < 0 { (target.height() as int + y % target.height() as int) as uint }
        else { y as uint % target.height() };

    target.set(x, y, value)
}
