//! Quaternions.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::Vector;
use crate::base::nearly_equal;

#[derive(Copy, Clone, PartialEq)]
/// A quaternion.
pub struct Quaternion(Vector);

impl Add for Quaternion
{
	type Output = Quaternion;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) }
}

impl AddAssign for Quaternion
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Default for Quaternion
{
	#[inline(always)]
	fn default() -> Self { Self(Vector::new(0f32, 0f32, 0f32, 1f32)) }
}

impl Div<f32> for Quaternion
{
	type Output = Quaternion;

	#[inline(always)]
	fn div(self, rhs: f32) -> Self { Self(self.0 / rhs) }
}

impl DivAssign<f32> for Quaternion
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; }
}

impl Mul<f32> for Quaternion
{
	type Output = Quaternion;

	#[inline(always)]
	fn mul(self, rhs: f32) -> Self { Self(self.0 * rhs) }
}

impl MulAssign<f32> for Quaternion
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; }
}

impl Sub for Quaternion
{
	type Output = Quaternion;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) }
}

impl SubAssign for Quaternion
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Quaternion
{
	#[inline(always)]
	/// Create a [`Quaternion`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Quaternion(Vector::new(x, y, z, w)) }

	#[inline(always)]
	/// Get the normalized [`Quaternion`].
	pub fn normalize(self) -> Self { Self(self.0.normalize()) }

	#[inline(always)]
	/// Get the dot product of two [`Quaternion`]s.
	pub fn dot(lhs: Quaternion, rhs: Quaternion) -> f32 { Vector::dot(lhs.0, rhs.0) }

	#[inline(always)]
	/// Spherical linear interpolate from `from` to `to` with a factor `t`.
	/// # Panics in debug mode
	/// If either `from` or `to` is not normalized.
	pub fn slerp(from: Quaternion, to: Quaternion, t: f32) -> Quaternion
	{
		assert!(nearly_equal(Self::dot(from, from), 1f32, 0.0001f32));
		assert!(nearly_equal(Self::dot(to, to), 1f32, 0.0001f32));

		let cos = Self::dot(from, to);
		if cos > 0.9995f32
		{
			(from * (1f32 - t) + to * t).normalize()
		}
		else
		{
			let theta = cos.acos();
			let dtheta = theta * t;
			let qperp = (to - from * cos).normalize();
			from * dtheta.cos() + qperp * dtheta.sin()
		}
	}
}
