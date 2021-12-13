//! Implementation using SIMD intrinsics for x86 processors.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::{f32, panic};
use std::{
	ops::{Add, Div, Mul, Sub},
	slice::from_raw_parts,
};

use crate::{is_shuffle_arg, shuffle_mask, Check, True};

#[repr(transparent)]
#[derive(Copy, Clone)]
/// A four-dimensional row vector.
pub struct Vector {
	data: __m128,
}

impl Add for Vector {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_add_ps(self.data, rhs.data) },
		}
	}
}

impl Default for Vector {
	#[inline(always)]
	fn default() -> Self {
		Self {
			data: unsafe { _mm_setzero_ps() },
		}
	}
}

impl Div for Vector {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_div_ps(self.data, rhs.data) },
		}
	}
}

impl Div<f32> for Vector {
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self {
		Self {
			data: unsafe { _mm_div_ps(self.data, _mm_set1_ps(rhs)) },
		}
	}
}

impl Mul for Vector {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_mul_ps(self.data, rhs.data) },
		}
	}
}

impl Mul<f32> for Vector {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self {
		Self {
			data: unsafe { _mm_mul_ps(self.data, _mm_set1_ps(rhs)) },
		}
	}
}

impl PartialEq for Vector {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		unsafe {
			let vcmp = _mm_castps_si128(_mm_cmpeq_ps(self.data, other.data));
			_mm_movemask_epi8(vcmp) == 0xffff
		}
	}
}

impl Sub for Vector {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_sub_ps(self.data, rhs.data) },
		}
	}
}

impl Vector {
	#[inline(always)]
	/// Create a [`Vector`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self {
			data: unsafe { _mm_set_ps(w, z, y, x) },
		}
	}

	#[inline(always)]
	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { unsafe { _mm_cvtss_f32(self.data) } }

	#[inline(always)]
	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 {
		unsafe { _mm_cvtss_f32(_mm_shuffle_ps::<{ _MM_SHUFFLE(1, 1, 1, 1) }>(self.data, self.data)) }
	}

	#[inline(always)]
	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 {
		unsafe { _mm_cvtss_f32(_mm_shuffle_ps::<{ _MM_SHUFFLE(2, 2, 2, 2) }>(self.data, self.data)) }
	}

	#[inline(always)]
	/// Get the w value
	pub fn w(self) -> f32 {
		unsafe { _mm_cvtss_f32(_mm_shuffle_ps::<{ _MM_SHUFFLE(3, 3, 3, 3) }>(self.data, self.data)) }
	}

	#[inline(always)]
	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32) {
		unsafe {
			self.data = _mm_move_ss(self.data, _mm_set_ss(val));
		}
	}

	#[inline(always)]
	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32) {
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // y_zw
			let t = _mm_shuffle_ps::<{ _MM_SHUFFLE(3, 2, 0, 0) }>(t, t); // yyzw
			self.data = _mm_move_ss(t, self.data);
		}
	}

	#[inline(always)]
	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32) {
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // zy_w
			let t = _mm_shuffle_ps::<{ _MM_SHUFFLE(3, 0, 1, 0) }>(t, t); // zyzw
			self.data = _mm_move_ss(t, self.data);
		}
	}

	#[inline(always)]
	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32) {
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // wyz_
			let t = _mm_shuffle_ps::<{ _MM_SHUFFLE(0, 2, 1, 0) }>(t, t); // wyzw
			self.data = _mm_move_ss(t, self.data);
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
	/// Shuffles the components of a [`Vector`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32, const W: u32>(self) -> Self
	where
		Check<{ is_shuffle_arg(X, Y, Z, W) }>: True,
		[(); shuffle_mask(W, Z, Y, X) as usize]:,
	{
		Self {
			data: unsafe { _mm_shuffle_ps::<{ shuffle_mask(W, Z, Y, X) }>(self.data, self.data) },
		}
	}

	#[inline(always)]
	/// Shuffles and merges the components of two [`Vector`]s.
	/// Takes `x` and `y` from `vec1`, and `z` and `w` from `vec2`.
	pub fn shuffle_merge<const X: u32, const Y: u32, const Z: u32, const W: u32>(vec1: Vector, vec2: Vector) -> Self
	where
		Check<{ is_shuffle_arg(X, Y, Z, W) }>: True,
		[(); shuffle_mask(W, Z, Y, X) as usize]:,
	{
		Self {
			data: unsafe { _mm_shuffle_ps::<{ shuffle_mask(W, Z, Y, X) }>(vec1.data, vec2.data) },
		}
	}

	#[inline(always)]
	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self {
		unsafe {
			Self {
				data: _mm_andnot_ps(SIGNBITS.vec, self.data),
			}
		}
	}

	#[inline(always)]
	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum(self) -> f32 {
		unsafe {
			let shuf = _mm_shuffle_ps::<{ _MM_SHUFFLE(2, 3, 0, 1) }>(self.data, self.data);
			let sum = _mm_add_ps(self.data, shuf);
			let shuf = _mm_movehl_ps(shuf, sum);
			let sum = _mm_add_ps(sum, shuf);
			_mm_cvtss_f32(sum)
		}
	}

	#[inline(always)]
	/// Get the component-wise minimums.
	pub fn min(lhs: Self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_min_ps(lhs.data, rhs.data) },
		}
	}

	#[inline(always)]
	/// Get the component-wise maximums.
	pub fn max(lhs: Self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_max_ps(lhs.data, rhs.data) },
		}
	}

	#[inline(always)]
	/// x: `lhs`.x + `lhs`.y.  
	/// y: `lhs`.z + `lhs`.w.  
	/// z: `rhs`.x + `rhs`.y.  
	/// w: `rhs`.z + `rhs`.w.  
	pub fn adj_add(lhs: Self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_hadd_ps(lhs.data, rhs.data) },
		}
	}

	#[inline(always)]
	/// x: `lhs`.x - `lhs`.y.  
	/// y: `lhs`.z - `lhs`.w.  
	/// z: `rhs`.x - `rhs`.y.  
	/// w: `rhs`.z - `rhs`.w.  
	pub fn adj_sub(lhs: Self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_hsub_ps(lhs.data, rhs.data) },
		}
	}

	#[inline(always)]
	/// Subtract and add alternate elements.
	pub fn add_sub(lhs: Self, rhs: Self) -> Self {
		Self {
			data: unsafe { _mm_addsub_ps(lhs.data, rhs.data) },
		}
	}
}

union Bits {
	uints: [u32; 4],
	vec: __m128,
}

const SIGNBITS: Bits = Bits {
	uints: [0x80000000, 0x80000000, 0x80000000, 0x80000000],
};
