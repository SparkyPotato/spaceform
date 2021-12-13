//! Implementation using scalar math only.

use core::f32;
use std::{
	ops::{Add, Div, Mul, Sub},
	slice::from_raw_parts,
};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
/// A four-dimensional row vector.
pub struct Vector {
	x: f32,
	y: f32,
	z: f32,
	w: f32,
}

impl Add for Vector {
	type Output = Vector;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl Default for Vector {
	#[inline(always)]
	fn default() -> Self {
		Self {
			x: 0f32,
			y: 0f32,
			z: 0f32,
			w: 0f32,
		}
	}
}

impl Div for Vector {
	type Output = Vector;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
			w: self.w / rhs.w,
		}
	}
}

impl Div<f32> for Vector {
	type Output = Vector;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
			w: self.w / rhs,
		}
	}
}

impl Mul for Vector {
	type Output = Vector;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
			w: self.w * rhs.w,
		}
	}
}

impl Mul<f32> for Vector {
	type Output = Vector;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}

impl Sub for Vector {
	type Output = Vector;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}

impl Vector {
	#[inline(always)]
	/// Create a [`Vector`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }

	#[inline(always)]
	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { self.x }

	#[inline(always)]
	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 { self.y }

	#[inline(always)]
	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 { self.z }

	#[inline(always)]
	/// Get the w value
	pub fn w(self) -> f32 { self.w }

	#[inline(always)]
	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32) { self.x = val }

	#[inline(always)]
	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32) { self.y = val }

	#[inline(always)]
	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32) { self.z = val }

	#[inline(always)]
	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32) { self.w = val }

	#[inline(always)]
	/// Shuffles the components of a [`Vector`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32, const W: u32>(self) -> Self {
		let data = unsafe { from_raw_parts((&self as *const Vector) as *const f32, 4) };

		Self {
			x: data[X as usize],
			y: data[Y as usize],
			z: data[Z as usize],
			w: data[W as usize],
		}
	}

	#[inline(always)]
	/// Shuffles and merges the components of two [`Vector`]s.
	/// Takes `x` and `y` from `vec1`, and `z` and `w` from `vec2`.
	pub fn shuffle_merge<const X: u32, const Y: u32, const Z: u32, const W: u32>(vec1: Vector, vec2: Vector) -> Self {
		let data = unsafe {
			(
				from_raw_parts((&vec1 as *const Vector) as *const f32, 4),
				from_raw_parts((&vec2 as *const Vector) as *const f32, 4),
			)
		};

		Self {
			x: data.0[X as usize],
			y: data.0[Y as usize],
			z: data.1[Z as usize],
			w: data.1[W as usize],
		}
	}

	#[inline(always)]
	/// Get an indexed value from the [`Vector`]. This is slow, don't use it unless you have to.
	/// Panics if idx is not in the range [0, 3].
	pub fn get(self, idx: u8) -> f32 {
		assert!(idx < 4, "Indexed out of Vector bounds");
		unsafe { from_raw_parts((&self as *const Vector) as *const f32, 4)[idx as usize] }
	}

	#[inline(always)]
	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self {
		Self {
			x: self.x.abs(),
			y: self.y.abs(),
			z: self.z.abs(),
			w: self.w.abs(),
		}
	}

	#[inline(always)]
	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum(self) -> f32 { self.x + self.y + self.z + self.w }

	#[inline(always)]
	/// Get the component-wise minimums.
	pub fn min(lhs: Self, rhs: Self) -> Self {
		Self {
			x: f32::min(lhs.x, rhs.x),
			y: f32::min(lhs.y, rhs.y),
			z: f32::min(lhs.z, rhs.z),
			w: f32::min(lhs.w, rhs.w),
		}
	}

	#[inline(always)]
	/// Get the component-wise maximums.
	pub fn max(lhs: Self, rhs: Self) -> Self {
		Self {
			x: f32::max(lhs.x, rhs.x),
			y: f32::max(lhs.y, rhs.y),
			z: f32::max(lhs.z, rhs.z),
			w: f32::max(lhs.w, rhs.w),
		}
	}

	#[inline(always)]
	/// x: `lhs`.x + `lhs`.y.  
	/// y: `lhs`.z + `lhs`.w.  
	/// z: `rhs`.x + `rhs`.y.  
	/// w: `rhs`.z + `rhs`.w.  
	pub fn adj_add(lhs: Self, rhs: Self) -> Self {
		Self {
			x: lhs.x + lhs.y,
			y: lhs.z + lhs.w,
			z: rhs.x + rhs.y,
			w: rhs.z + rhs.w,
		}
	}

	#[inline(always)]
	/// x: `lhs`.x - `lhs`.y.  
	/// y: `lhs`.z - `lhs`.w.  
	/// z: `rhs`.x - `rhs`.y.  
	/// w: `rhs`.z - `rhs`.w.  
	pub fn adj_sub(lhs: Self, rhs: Self) -> Self {
		Self {
			x: lhs.x - lhs.y,
			y: lhs.z - lhs.w,
			z: rhs.x - rhs.y,
			w: rhs.z - rhs.w,
		}
	}

	#[inline(always)]
	/// Subtract and add alternate elements.
	pub fn add_sub(lhs: Self, rhs: Self) -> Self {
		Self {
			x: lhs.x - rhs.x,
			y: lhs.y + rhs.y,
			z: lhs.z - rhs.z,
			w: lhs.w + rhs.w,
		}
	}
}
