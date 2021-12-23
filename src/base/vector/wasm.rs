//! Implementation using SIMD intrinsics for WebAssembly.
use core::arch::wasm32::*;
use std::{
	ops::{Add, Div, Mul, Sub},
	slice::from_raw_parts,
};

use crate::{is_shuffle_arg, shuffle_mask, Check, True};

#[repr(transparent)]
#[derive(Copy, Clone)]
/// A four-dimensional row vector.
pub struct Vector {
	data: v128,
}

impl Add for Vector {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self {
		Self {
			data: f32x4_add(self.data, rhs.data),
		}
	}
}

impl Default for Vector {
	#[inline(always)]
	fn default() -> Self { Self { data: f32x4_splat(0.0) } }
}

impl Div for Vector {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self {
		Self {
			data: f32x4_div(self.data, rhs.data),
		}
	}
}

impl Div<f32> for Vector {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self {
		Self {
			data: f32x4_div(self.data, f32x4_splat(rhs)),
		}
	}
}

impl Mul for Vector {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		Self {
			data: f32x4_mul(self.data, rhs.data),
		}
	}
}

impl Mul<f32> for Vector {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self {
		Self {
			data: f32x4_mul(self.data, f32x4_splat(rhs)),
		}
	}
}

impl PartialEq for Vector {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool { v128_any_true(f32x4_eq(self.data, other.data)) }
}

impl Sub for Vector {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self {
		Self {
			data: f32x4_sub(self.data, rhs.data),
		}
	}
}

impl Vector {
	#[inline(always)]
	/// Create a [`Vector`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self {
			data: f32x4(x, y, z, w),
		}
	}

	#[inline(always)]
	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { f32x4_extract_lane::<0>(self.data) }

	#[inline(always)]
	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 { f32x4_extract_lane::<1>(self.data) }

	#[inline(always)]
	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 { f32x4_extract_lane::<2>(self.data) }

	#[inline(always)]
	/// Get the w value
	pub fn w(self) -> f32 { f32x4_extract_lane::<3>(self.data) }

	#[inline(always)]
	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32) { self.data = f32x4_replace_lane::<0>(self.data, val) }

	#[inline(always)]
	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32) { self.data = f32x4_replace_lane::<1>(self.data, val) }

	#[inline(always)]
	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32) { self.data = f32x4_replace_lane::<2>(self.data, val) }

	#[inline(always)]
	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32) { self.data = f32x4_replace_lane::<3>(self.data, val) }

	#[inline(always)]
	/// Get an indexed value from the [`Vector`]. This is slow, don't use it unless you have to.
	/// Panics if idx is not in the range [0, 3].
	pub fn get(self, idx: u8) -> f32 {
		assert!(idx < 4, "Indexed out of Vector bounds");
		unsafe { from_raw_parts((&self as *const Vector) as *const f32, 4)[idx as usize] }
	}

	#[inline(always)]
	/// Shuffles the components of a [`Vector`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32, const W: u32>(self) -> Self
	where
		Check<{ is_shuffle_arg(X, Y, Z, W) }>: True,
		[(); shuffle_mask(W, Z, Y, X) as usize]:,
		[(); X as usize]:,
		[(); Y as usize]:,
		[(); Z as usize]:,
		[(); W as usize]:,
	{
		Self {
			data: u32x4_shuffle::<{ X as usize }, { Y as usize }, { Z as usize }, { W as usize }>(self.data, self.data),
		}
	}

	#[inline(always)]
	/// Shuffles and merges the components of two [`Vector`]s.
	/// Takes `x` and `y` from `vec1`, and `z` and `w` from `vec2`.
	pub fn shuffle_merge<const X: u32, const Y: u32, const Z: u32, const W: u32>(vec1: Vector, vec2: Vector) -> Self
	where
		Check<{ is_shuffle_arg(X, Y, Z, W) }>: True,
		[(); shuffle_mask(W, Z, Y, X) as usize]:,
		[(); X as usize]:,
		[(); Y as usize]:,
		[(); Z as usize + 4]:,
		[(); W as usize + 4]:,
	{
		Self {
			data: u32x4_shuffle::<{ X as usize }, { Y as usize }, { Z as usize + 4 }, { W as usize + 4 }>(
				vec1.data, vec2.data,
			),
		}
	}

	#[inline(always)]
	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self {
		Self {
			data: unsafe { v128_andnot(self.data, SIGNBITS.vec) },
		}
	}

	#[inline(always)]
	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum(self) -> f32 {
		let shuf = u32x4_shuffle::<1, 0, 3, 2>(self.data, self.data);
		let sum = f32x4_add(self.data, shuf);
		let shuf = u32x4_shuffle::<2, 1, 2, 1>(sum, sum);
		let sum = f32x4_add(sum, shuf);
		f32x4_extract_lane::<0>(sum)
	}

	#[inline(always)]
	/// Get the component-wise minimums.
	pub fn min(lhs: Self, rhs: Self) -> Self {
		Self {
			data: f32x4_min(lhs.data, rhs.data),
		}
	}

	#[inline(always)]
	/// Get the component-wise maximums.
	pub fn max(lhs: Self, rhs: Self) -> Self {
		Self {
			data: f32x4_max(lhs.data, rhs.data),
		}
	}

	#[inline(always)]
	/// x: `lhs`.x + `lhs`.y.
	/// y: `lhs`.z + `lhs`.w.
	/// z: `rhs`.x + `rhs`.y.
	/// w: `rhs`.z + `rhs`.w.
	pub fn adj_add(lhs: Self, rhs: Self) -> Self {
		let a = u32x4_shuffle::<0, 2, 0, 2>(lhs.data, rhs.data);
		let b = u32x4_shuffle::<1, 3, 1, 3>(lhs.data, rhs.data);
		Self { data: f32x4_add(a, b) }
	}

	#[inline(always)]
	/// x: `lhs`.x - `lhs`.y.
	/// y: `lhs`.z - `lhs`.w.
	/// z: `rhs`.x - `rhs`.y.
	/// w: `rhs`.z - `rhs`.w.
	pub fn adj_sub(lhs: Self, rhs: Self) -> Self {
		let a = u32x4_shuffle::<0, 2, 0, 2>(lhs.data, rhs.data);
		let b = u32x4_shuffle::<1, 3, 1, 3>(lhs.data, rhs.data);
		Self { data: f32x4_sub(a, b) }
	}

	#[inline(always)]
	/// Subtract and add alternate elements.
	pub fn add_sub(lhs: Self, rhs: Self) -> Self {
		let add = f32x4_add(lhs.data, rhs.data);
		let add = u32x4_shuffle::<1, 3, 1, 3>(add, add);
		let sub = f32x4_sub(lhs.data, rhs.data);
		let sub = u32x4_shuffle::<0, 2, 0, 2>(sub, sub);

		Self {
			data: u32x4_shuffle::<0, 1, 0, 1>(sub, add),
		}
	}
}

union Bits {
	uints: [u32; 4],
	vec: v128,
}

const SIGNBITS: Bits = Bits {
	uints: [0x80000000, 0x80000000, 0x80000000, 0x80000000],
};
