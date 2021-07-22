//! Quaternions.

use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::Vector;
use crate::base::nearly_equal;

#[derive(Copy, Clone, PartialEq)]
/// A quaternion.
pub struct Quaternion(pub(crate) Vector);

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

impl Debug for Quaternion
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.0) }
}

impl Default for Quaternion
{
	#[inline(always)]
	fn default() -> Self { Self(Vector::new(0f32, 0f32, 0f32, 1f32)) }
}

impl Display for Quaternion
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.0) }
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

impl Mul for Quaternion
{
	type Output = Quaternion;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self
	{
		// https://stackoverflow.com/q/18542894/14888326

		let wzyx = self.0.shuffle::<3, 2, 1, 0>();
		let baba = rhs.0.shuffle::<1, 0, 1, 0>();
		let dcdc = rhs.0.shuffle::<3, 2, 3, 2>();

		let z_xwy = Vector::adj_sub(self.0 * baba, wzyx * dcdc);
		let xzy_w = Vector::adj_add(self.0 * dcdc, wzyx * baba);

		let xzwy = Vector::add_sub(
			Vector::shuffle_merge::<0, 1, 2, 3>(xzy_w, z_xwy),
			Vector::shuffle_merge::<1, 0, 3, 2>(z_xwy, xzy_w),
		);

		Self(xzwy.shuffle::<0, 3, 1, 2>())
	}
}

impl MulAssign for Quaternion
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
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
	/// Get the x value of the [`Quaternion`].
	pub fn x(self) -> f32 { self.0.x() }

	#[inline(always)]
	/// Get the y value of the [`Quaternion`].
	pub fn y(self) -> f32 { self.0.y() }

	#[inline(always)]
	/// Get the z value of the [`Quaternion`].
	pub fn z(self) -> f32 { self.0.z() }

	#[inline(always)]
	/// Get the w value of the [`Quaternion`].
	pub fn w(self) -> f32 { self.0.w() }

	#[inline(always)]
	/// Set the x value of the [`Quaternion`].
	pub fn set_x(&mut self, val: f32) { self.0.set_x(val) }

	#[inline(always)]
	/// Set the y value of the [`Quaternion`].
	pub fn set_y(&mut self, val: f32) { self.0.set_y(val) }

	#[inline(always)]
	/// Set the z value of the [`Quaternion`].
	pub fn set_z(&mut self, val: f32) { self.0.set_z(val) }

	#[inline(always)]
	/// Set the w value of the [`Quaternion`].
	pub fn set_w(&mut self, val: f32) { self.0.set_w(val) }

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

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn mul()
	{
		let q = Quaternion::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(q * q, Quaternion::new(8f32, 16f32, 24f32, 2f32));
	}
}
