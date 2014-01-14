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
	
	pub fn raw<'r>(&'r self) -> &'r ~[T] {
		&'r self.data
	}
	
	pub fn raw_mut<'r>(&'r mut self) -> &'r mut ~[T] {
		&'r mut self.data
	}
}

impl<T: Clone> Clone for Array2D<T> {
		fn clone(&self) -> Array2D<T> {
			Array2D {
				width_:  self.width(),
				height_: self.height(),
				data:   self.data.clone()
			}
	}
}


pub fn from_fn<T>(width: uint, height: uint, op: |uint, uint| -> T) -> Array2D<T> {
	let size = width * height;
	let mut v = vec::with_capacity(size);
	let mut i = 0u;
	
	while i < size {
		let h = i / width;
		let w = i - h;
		
		v.push(op(w, h));				
		i += 1;
	}
	
	Array2D {
		width_: width,
		height_: height,
		data: v,
	}
}

pub fn from_elem<T:Clone>(width: uint, height: uint, elem: T) -> Array2D<T> {
	Array2D {
		width_: width,
		height_: height,
		data: vec::from_elem(width * height, elem.clone())
	}
}
