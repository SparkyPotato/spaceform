//! Surface normals.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::{
	convert::Into,
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
	base::{shuffle_args, Vector},
	direction::Direction,
	transform::Transform,
};

#[derive(Copy, Clone, PartialEq)]
/// A surface normal. Mostly the same as [`Direction`]. May NOT be normalized.
pub struct Normal(pub(crate) Vector);

impl Add for Normal
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}

impl AddAssign for Normal
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs }
}

impl Debug for Normal
{
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "Norm [{}, {}, {}]", self.x(), self.y(), self.z())
	}
}

impl Default for Normal
{
	#[inline(always)]
	fn default() -> Self { Self(Vector::new(0f32, 0f32, 0f32, 0f32)) }
}

impl Display for Normal
{
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "Norm [{}, {}, {}]", self.x(), self.y(), self.z())
	}
}

impl Div<f32> for Normal
{
	type Output = Self;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self::Output { Self(self.0 / rhs) }
}

impl DivAssign<f32> for Normal
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: f32) { *self = *self / rhs }
}

impl Mul<f32> for Normal
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self::Output { Self(self.0 * rhs) }
}

impl Mul<Transform> for Normal
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Transform) -> Self::Output { Self(self.0 * rhs.inverse.transpose()) }
}

impl MulAssign<f32> for Normal
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs }
}

impl MulAssign<Transform> for Normal
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Transform) { *self = *self * rhs }
}

impl Neg for Normal
{
	type Output = Self;

	#[inline(always)]
	fn neg(self) -> Self::Output { Self(-self.0) }
}

impl Sub for Normal
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}

impl SubAssign for Normal
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs }
}

impl Normal
{
	#[inline(always)]
	/// Create a new [`Normal`] with the given `x`, `y`, and `z` values.
	pub fn new(x: f32, y: f32, z: f32) -> Self { Self(Vector::new(x, y, z, 0f32)) }

	#[inline(always)]
	/// Get the x value of the [`Direction`].
	pub fn x(self) -> f32 { self.0.x() }

	#[inline(always)]
	/// Get the y value of the [`Direction`].
	pub fn y(self) -> f32 { self.0.y() }

	#[inline(always)]
	/// Get the z value of the [`Direction`].
	pub fn z(self) -> f32 { self.0.z() }

	#[inline(always)]
	/// Set the x value of the [`Direction`].
	pub fn set_x(&mut self, val: f32) { self.0.set_x(val) }

	#[inline(always)]
	/// Set the y value of the [`Direction`].
	pub fn set_y(&mut self, val: f32) { self.0.set_y(val) }

	#[inline(always)]
	/// Set the z value of the [`Direction`].
	pub fn set_z(&mut self, val: f32) { self.0.set_z(val) }

	#[inline(always)]
	/// Get the square of the length of the [`Direction`].
	pub fn length_square(self) -> f32 { self.0.length_square() }

	#[inline(always)]
	/// Get the length of the [`Direction`].
	pub fn length(self) -> f32 { self.0.length() }

	#[inline(always)]
	/// Get the normalized [`Direction`].
	pub fn normalize(self) -> Self { Self(self.0.normalize()) }

	#[inline(always)]
	/// Shuffle the components of a [`Direction`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32>(self) -> Self
	where
		[(); shuffle_args(X, Y, Z, 3)]: Sized,
		[(); _MM_SHUFFLE(3, Z, Y, X) as usize]: Sized,
	{
		Self(self.0.shuffle::<X, Y, Z, 3>())
	}

	#[inline(always)]
	/// Flip the normal if it lies in the opposite hemisphere of `dir`.
	pub fn in_hemisphere(self, dir: Direction) -> Self
	{
		if Direction::dot(self.into(), dir) < 0f32
		{
			-self
		}
		else
		{
			self
		}
	}

	#[inline(always)]
	/// Get the dot product of two [`Direction`]s.
	pub fn dot(lhs: Normal, rhs: Normal) -> f32 { Vector::dot(lhs.0, rhs.0) }
}
