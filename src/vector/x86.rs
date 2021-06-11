//! Implementation using SIMD intrinsics for x86 processors.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::f32;
use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone)]
/// A four-dimensional vector.
pub struct Vector
{
	data: __m128,
}

impl Add for Vector
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self
	{
		Self {
			data: unsafe { _mm_add_ps(self.data, rhs.data) },
		}
	}
}

impl AddAssign for Vector
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Debug for Vector
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl Default for Vector
{
	#[inline(always)]
	fn default() -> Self
	{
		Self {
			data: unsafe { _mm_setzero_ps() },
		}
	}
}

impl Display for Vector
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl Div for Vector
{
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self
	{
		Self {
			data: unsafe { _mm_div_ps(self.data, rhs.data) },
		}
	}
}

impl DivAssign for Vector
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl Div<f32> for Vector
{
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self
	{
		Self {
			data: unsafe { _mm_div_ps(self.data, _mm_set1_ps(rhs)) },
		}
	}
}

impl DivAssign<f32> for Vector
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; }
}

impl Mul for Vector
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self
	{
		Self {
			data: unsafe { _mm_mul_ps(self.data, rhs.data) },
		}
	}
}

impl MulAssign for Vector
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl Mul<f32> for Vector
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self
	{
		Self {
			data: unsafe { _mm_mul_ps(self.data, _mm_set1_ps(rhs)) },
		}
	}
}

impl MulAssign<f32> for Vector
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; }
}

impl Neg for Vector
{
	type Output = Self;

	#[inline(always)]
	fn neg(self) -> Self { Self::default() - self }
}

impl PartialEq for Vector
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		unsafe {
			let vcmp = _mm_castps_si128(_mm_cmpeq_ps(self.data, other.data));
			_mm_movemask_epi8(vcmp) == 0xffff
		}
	}
}

impl Sub for Vector
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self
	{
		Self {
			data: unsafe { _mm_sub_ps(self.data, rhs.data) },
		}
	}
}

impl SubAssign for Vector
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Vector
{
	#[inline(always)]
	/// Create a [`Vector`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self
	{
		Self {
			data: unsafe { _mm_set_ps(w, z, y, x) },
		}
	}

	#[inline(always)]
	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { unsafe { _mm_cvtss_f32(self.data) } }

	#[inline(always)]
	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 { unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, _MM_SHUFFLE(1, 1, 1, 1))) } }

	#[inline(always)]
	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 { unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, _MM_SHUFFLE(2, 2, 2, 2))) } }

	#[inline(always)]
	/// Get the w value
	pub fn w(self) -> f32 { unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, _MM_SHUFFLE(3, 3, 3, 3))) } }

	#[inline(always)]
	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32)
	{
		unsafe {
			self.data = _mm_move_ss(self.data, _mm_set_ss(val));
		}
	}

	#[inline(always)]
	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32)
	{
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // y_zw
			let t = _mm_shuffle_ps(t, t, _MM_SHUFFLE(3, 2, 0, 0)); // yyzw
			self.data = _mm_move_ss(t, self.data);
		}
	}

	#[inline(always)]
	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32)
	{
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // zy_w
			let t = _mm_shuffle_ps(t, t, _MM_SHUFFLE(3, 0, 1, 0)); // zyzw
			self.data = _mm_move_ss(t, self.data);
		}
	}

	#[inline(always)]
	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32)
	{
		unsafe {
			let t = _mm_move_ss(self.data, _mm_set_ss(val)); // wyz_
			let t = _mm_shuffle_ps(t, t, _MM_SHUFFLE(0, 2, 1, 0)); // wyzw
			self.data = _mm_move_ss(t, self.data);
		}
	}

	#[inline(always)]
	/// Shuffles the components of a [`Vector`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32, const W: u32>(self) -> Self
	{
		Self {
			data: unsafe { _mm_shuffle_ps(self.data, self.data, _MM_SHUFFLE(W, Z, Y, X)) },
		}
	}

	#[inline(always)]
	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self
	{
		unsafe {
			Self {
				data: _mm_andnot_ps(SIGNBITS.vec, self.data),
			}
		}
	}

	#[inline(always)]
	/// Get the three-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum3(self) -> f32
	{
		let shuf = self.shuffle::<1, 2, 3, 0>();
		let res = self + shuf;
		let shuf = shuf.shuffle::<1, 2, 3, 0>();
		let res = res + shuf;
		unsafe { _mm_cvtss_f32(res.data) }
	}

	#[inline(always)]
	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum4(self) -> f32
	{
		unsafe {
			let shuf = _mm_movehdup_ps(self.data);
			let sum = _mm_add_ps(self.data, shuf);
			let shuf = _mm_movehl_ps(shuf, sum);
			let sum = _mm_add_ps(sum, shuf);
			_mm_cvtss_f32(sum)
		}
	}

	#[inline(always)]
	/// Get the square of the four-dimensional length of the [`Vector`].
	pub fn length3_square(self) -> f32 { dot3(self, self) }

	#[inline(always)]
	/// Get the square of the four-dimensional length of the [`Vector`].
	pub fn length4_square(self) -> f32 { dot4(self, self) }

	#[inline(always)]
	/// Get the four-dimensional length of the [`Vector`].
	pub fn length3(self) -> f32 { self.length3_square().sqrt() }

	#[inline(always)]
	/// Get the four-dimensional length of the [`Vector`].
	pub fn length4(self) -> f32 { self.length4_square().sqrt() }

	#[inline(always)]
	/// Get the normalized four-dimensional length of the [`Vector`].
	pub fn normalize3(self) -> Self { self / self.length3() }

	#[inline(always)]
	/// Get the normalized four-dimensional length of the [`Vector`].
	pub fn normalize4(self) -> Self { self / self.length4() }
}

#[inline(always)]
/// Get the three-dimensional dot product of two [`Vector`]s.
pub fn dot3(lhs: Vector, rhs: Vector) -> f32 { (lhs * rhs).hsum3() }

#[inline(always)]
/// Get the four-dimensional dot product of two [`Vector`]s.
pub fn dot4(lhs: Vector, rhs: Vector) -> f32 { (lhs * rhs).hsum4() }

#[inline(always)]
/// Get the three-dimensional cross product of two [`Vector`]s.
pub fn cross(lhs: Vector, rhs: Vector) -> Vector
{
	let temp = lhs.shuffle::<1, 2, 0, 3>();
	temp * rhs.shuffle::<2, 0, 1, 3>() - (temp * rhs).shuffle::<1, 2, 0, 3>()
}

#[inline(always)]
/// Get the component-wise minimums.
pub fn min(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: unsafe { _mm_min_ps(lhs.data, rhs.data) },
	}
}

#[inline(always)]
/// Get the component-wise maximums.
pub fn max(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: unsafe { _mm_max_ps(lhs.data, rhs.data) },
	}
}

union Bits
{
	uints: [u32; 4],
	vec: __m128,
}

const SIGNBITS: Bits = Bits {
	uints: [0x80000000, 0x80000000, 0x80000000, 0x80000000],
};
