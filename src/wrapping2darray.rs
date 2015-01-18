use std::slice;
use std::iter;
use std::ops::{Index, IndexMut};

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
            data: iter::repeat(elem).take((width * height) as usize).collect(),
        }
    }

    pub fn from_fn<F>(width: i32, height: i32, mut f: F) -> Wrapping2DArray<T>
        where F: FnMut(i32, i32) -> T
    {
        assert!(width > 0 && height > 0);
        Wrapping2DArray {
            width: width,
            height: height,
            data: (0..(width * height)).map(|i| f(i % width, i / width)).collect(),
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

    /// Returns an iterator over references to the elements of the array in
    /// the order: left-right, up-down
    pub fn iter<'r>(&'r self) -> slice::Iter<'r, T> {
        self.data.iter()
    }

    /// Returns an iterator over mutable references to the elements of the array in
    /// the order: left-right, up-down
    pub fn iter_mut<'r>(&'r mut self) -> slice::IterMut<'r, T> {
        self.data.iter_mut()
    }
}

impl<T> Index<(i32, i32)> for Wrapping2DArray<T> {
    type Output = T;

    fn index(&self, index: &(i32, i32)) -> &T {
        let &(x, y) = index;
        let wrapped_x = {
            if x >= 0 { x % self.width }
            else { self.width + x % self.width }
        };
        let wrapped_y = {
            if y >= 0 { y % self.height }
            else { self.height + y % self.height }
        };

        &self.data[(wrapped_x + wrapped_y * self.width) as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Wrapping2DArray<T> {
    type Output = T;

    fn index_mut(&mut self, index: &(i32, i32)) -> &mut T {
        let &(x, y) = index;
        let wrapped_x = {
            if x >= 0 { x % self.width }
            else { self.width + x % self.width }
        };
        let wrapped_y = {
            if y >= 0 { y % self.height }
            else { self.height + y % self.height }
        };

        &mut self.data[(wrapped_x + wrapped_y * self.width) as usize]
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

