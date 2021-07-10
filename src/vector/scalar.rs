//! Implementation using scalar math only.

use core::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone)]
/// A four-dimensional row vector.
pub struct Vector
{
	data: [f32; 4],
}

impl Add for Vector
{
	type Output = Vector;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 + val.1),
		}
	}
}

impl AddAssign for Vector
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Default for Vector
{
	#[inline(always)]
	fn default() -> Self
	{
		Self {
			data: [0f32, 0f32, 0f32, 0f32],
		}
	}
}

impl Div for Vector
{
	type Output = Vector;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 / val.1),
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
	type Output = Vector;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self
	{
		Self {
			data: self.data.map(|val| val / rhs),
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
	type Output = Vector;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 * val.1),
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
	type Output = Vector;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self
	{
		Self {
			data: self.data.map(|val| val * rhs),
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
	fn eq(&self, other: &Vector) -> bool { self.data == other.data }
}

impl Sub for Vector
{
	type Output = Vector;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 - val.1),
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
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { data: [x, y, z, w] } }

	#[inline(always)]
	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { self.data[0] }

	#[inline(always)]
	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 { self.data[1] }

	#[inline(always)]
	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 { self.data[2] }

	#[inline(always)]
	/// Get the w value
	pub fn w(self) -> f32 { self.data[3] }

	#[inline(always)]
	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32) { self.data[0] = val }

	#[inline(always)]
	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32) { self.data[1] = val }

	#[inline(always)]
	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32) { self.data[2] = val }

	#[inline(always)]
	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32) { self.data[3] = val }

	#[inline(always)]
	/// Shuffles the components of a [`Vector`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32, const W: u32>(self) -> Self
	{
		Self {
			data: [
				self.data[X as usize],
				self.data[Y as usize],
				self.data[Z as usize],
				self.data[W as usize],
			],
		}
	}

	#[inline(always)]
	/// Get an indexed value from the [`Vector`]. This is slow, don't use it unless you have to.
	/// Panics if idx is not in the range [0, 3].
	pub fn get(&self, idx: u8) -> f32 { self.data[idx as usize] }

	#[inline(always)]
	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self
	{
		Self {
			data: self.data.map(|val| val.abs()),
		}
	}

	#[inline(always)]
	/// Get the three-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum3(self) -> f32 { self.data[0] + self.data[1] + self.data[2] }

	#[inline(always)]
	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum4(self) -> f32 { self.data[0] + self.data[1] + self.data[2] + self.data[3] }
}

#[inline(always)]
/// Get the component-wise minimums.
pub fn min(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: lhs.data.zip(rhs.data).map(|val| f32::min(val.0, val.1)),
	}
}

#[inline(always)]
/// Get the component-wise maximums.
pub fn max(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: lhs.data.zip(rhs.data).map(|val| f32::max(val.0, val.1)),
	}
}
