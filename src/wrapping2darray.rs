use std::slice;
use std::iter;

/// A structure for storing a 2D collection of elements where the edges of the array wrap.
pub struct Wrapping2DArray<T> {
    width: i32,
    height: i32,
    data: Vec<T>,
}

impl<T: Clone> Wrapping2DArray<T> {
    pub fn from_elem(width: i32, height: i32, elem: T) -> Wrapping2DArray<T> {
        assert!(width > 0 && height > 0);

        Wrapping2DArray {
            width: width,
            height: height,
            data: iter::repeat(elem).take((width * height) as uint).collect(),
        }

    }

    pub fn from_fn<F>(width: i32, height: i32, mut f: F) -> Wrapping2DArray<T>
        where F: FnMut(i32, i32) -> T
    {
        assert!(width > 0 && height > 0);
        Wrapping2DArray {
            width: width,
            height: height,
            data: range(0, width * height).map(|i| f(i % width, i / width)).collect(),
        }
    }

    /// Returns the height of the array
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Returns the width of the array
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Returns a reference to the value at index `x`, `y`
    pub fn get(&self, x: i32, y: i32) -> &T {
        let wrapped_x = {
            if x >= 0 { x % self.width }
            else { self.width + x % self.width }
        };
        let wrapped_y = {
            if y >= 0 { y % self.height }
            else { self.height + y % self.height }
        };

        &self.data[(wrapped_x + wrapped_y * self.width) as uint]
    }

    /// Returns a mutable reference to the value at index `x`, `y`
    pub fn get_mut<'a>(&'a mut self, x: i32, y: i32) -> &'a mut T {
        let wrapped_x = {
            if x >= 0 { x % self.width }
            else { self.width + x % self.width }
        };
        let wrapped_y = {
            if y >= 0 { y % self.height }
            else { self.height + y % self.height }
        };

        &mut self.data[(wrapped_x + wrapped_y * self.width) as uint]
    }

    /// Returns an iterator over references to the elements of the array in
    /// the order: left-right, up-down
    pub fn iter<'r>(&'r self) -> slice::Items<'r, T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the elements of the array in
    /// the order: left-right, up-down
    pub fn iter_mut<'r>(&'r mut self) -> slice::MutItems<'r, T> {
        self.data.iter_mut()
    }
}

impl<T: Clone> Clone for Wrapping2DArray<T> {
    fn clone(&self) -> Wrapping2DArray<T> {
        Wrapping2DArray {
            width: self.width,
            height: self.height,
            data: self.data.clone()
        }
    }
}

