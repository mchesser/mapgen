#![allow(dead_code)]

use std::slice;

/// A structure for storing a 2D collection of elements
pub struct Array2D<T> {
    width: uint,
    height: uint,
    data: Vec<T>,
}

impl<T: Clone> Array2D<T> {
    #[inline]
    /// Returns the height of the array
    /// # Return
    /// The height of the array
    pub fn height(&self) -> uint {
        self.height
    }

    #[inline]
    /// Returns the width of the array
    pub fn width(&self) -> uint {
        self.width
    }

    #[inline]
    /// Returns a reference to the value at index `x`, `y`
    /// # Arguments
    /// `x` - the x coordinate
    /// `y` - the y coordinate
    pub fn get<'a>(&'a self, x: uint, y: uint) -> &'a T {
        if x > self.width() || y > self.height() {
            fail!(
                format!("Index out of bounds, x: {}, y: {}, width: {} height: {}",
                    x,
                    y,
                    self.width(),
                    self.height())
            );
        }

        self.data.get((x + y * self.width()) as uint)
    }

    #[inline]
    /// Returns a mutable reference to the value at index `x`, `y`
    /// # Arguments
    /// `x` - the x coordinate
    /// `y` - the y coordinate
    pub fn get_mut<'a>(&'a mut self, x: uint, y: uint) -> &'a mut T {
        if x > self.width() || y > self.height() {
            fail!(
                format!("Index out of bounds, x: {}, y: {}, width: {} height: {}",
                    x,
                    y,
                    self.width(),
                    self.height())
            );
        }
        
        let width = self.width;
        self.data.get_mut((x + y * width) as uint)
    }

    /// Returns an iterator over references to the elements of the array in
    /// the order: left-right, up-down
    pub fn iter<'r>(&'r self) -> slice::Items<'r, T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the elements of the array in
    /// the order: left-right, up-down
    pub fn mut_iter<'r>(&'r mut self) -> slice::MutItems<'r, T> {
        self.data.mut_iter()
    }
}

impl<T: Clone> Clone for Array2D<T> {
    fn clone(&self) -> Array2D<T> {
        Array2D {
            width:  self.width(),
            height: self.height(),
            data:   self.data.clone()
        }
    }
}

/// Creates a new array from a function
/// # Arguments
/// `width` - The width of the array
/// `height` - The height of the array
/// `op` - The function to use
pub fn from_fn<T>(width: uint, height: uint, op: |uint, uint| -> T) -> Array2D<T> {
    Array2D {
        width: width,
        height: height,
        data: Vec::from_fn(width * height, |i| op(i % width, i / width)),
    }
}

/// Creates a new array from a base element
/// # Arguments
/// `width` - The width of the array
/// `height` - The height of the array
/// `elem` - The element to use
pub fn from_elem<T:Clone>(width: uint, height: uint, elem: T) -> Array2D<T> {
    Array2D {
        width: width,
        height: height,
        data: Vec::from_elem(width * height, elem.clone())
    }
}

/// Creates a new array from a raw vector.
///  - Fails if the raw vector has the wrong length.
///  - Raw vector is moved
/// # Arguments
/// `width` - The width of the array
/// `height` - The height of the array
/// `raw` - The raw vector
pub fn from_raw<T>(width: uint, height: uint, raw: Vec<T>) -> Array2D<T> {
    if width * height != raw.len() {
        fail!("Raw array of invalid length");
    }

    Array2D {
        width: width,
        height: height,
        data: raw
    }
}

///
/// Utility Functions
///

/// Normalises a 2D array of floats to values between 0.0 and 1.0
/// # Argumens
/// `target` - the array to normalise
pub fn normalise(target: &mut Array2D<f32>) {
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
    for val in target.mut_iter() {
        *val = (*val - min) * factor;
    }
}

/// Gets a value from an array, wrapping the x and y values to fit in the domain
/// # Arguments
/// `target` - the array to get the values from
/// `x` - the x coordinate
/// `y` - the y coordinate
/// # Return
/// The element found after wrapping x and y
pub fn wrap_get<T: Clone>(target: &Array2D<T>, x: int, y: int) -> T {
    let x =
        if x < 0 { (target.width() as int + x % target.width() as int) as uint }
        else { x as uint % target.width() };

    let y =
        if y < 0 { (target.height() as int + y % target.height() as int) as uint }
        else { y as uint % target.height() };

    target.get(x, y).clone()
}

/// Sets a value in an array, wrapping the x and y values to fit in the domain
/// # Arguments
/// `target` - the array
/// `x` - the x coordinate
/// `y` - the y coordinate
/// `value` - The value to set
pub fn wrap_set<T: Clone>(target: &mut Array2D<T>, x: int, y: int, value: T) {
    let x =
        if x < 0 { (target.width() as int + x % target.width() as int) as uint }
        else { x as uint % target.width() };

    let y =
        if y < 0 { (target.height() as int + y % target.height() as int) as uint }
        else { y as uint % target.height() };

    *target.get_mut(x, y) = value;
}
