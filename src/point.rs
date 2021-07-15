//! Points in 3D space.

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{
	base::{shuffle_args, Vector},
	coordinate_system::CoordinateSystem,
	direction::Direction,
};

#[derive(Copy, Clone, PartialEq)]
/// A point in 3D space, with a W coordinate of 1.
pub struct Point(pub(crate) Vector);

impl Add<Direction> for Point
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Direction) -> Self::Output { Self(self.0 + rhs.0) }
}

impl AddAssign<Direction> for Point
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Direction) { *self = *self + rhs }
}

impl Debug for Point
{
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "Point [{}, {}, {}]", self.x(), self.y(), self.z())
	}
}

impl Default for Point
{
	#[inline(always)]
	fn default() -> Self { Self(Vector::new(0f32, 0f32, 0f32, 1f32)) }
}

impl Display for Point
{
	#[inline(always)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "Point [{}, {}, {}]", self.x(), self.y(), self.z())
	}
}

impl Sub for Point
{
	type Output = Direction;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output { Direction(self.0 - rhs.0) }
}

impl Sub<Direction> for Point
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Direction) -> Self::Output { Self(self.0 - rhs.0) }
}

impl SubAssign<Direction> for Point
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Direction) { *self = *self - rhs }
}

impl Point
{
	#[inline(always)]
	/// Create a new [`Point`] with the given `x`, `y`, and `z` values.
	pub fn new(x: f32, y: f32, z: f32) -> Self { Self(Vector::new(x, y, z, 1f32)) }

	#[inline(always)]
	/// Get the x value of the [`Point`].
	pub fn x(self) -> f32 { self.0.x() }

	#[inline(always)]
	/// Get the y value of the [`Point`].
	pub fn y(self) -> f32 { self.0.y() }

	#[inline(always)]
	/// Get the z value of the [`Point`].
	pub fn z(self) -> f32 { self.0.z() }

	#[inline(always)]
	/// Set the x value of the [`Point`].
	pub fn set_x(&mut self, val: f32) { self.0.set_x(val) }

	#[inline(always)]
	/// Set the y value of the [`Point`].
	pub fn set_y(&mut self, val: f32) { self.0.set_y(val) }

	#[inline(always)]
	/// Set the z value of the [`Point`].
	pub fn set_z(&mut self, val: f32) { self.0.set_z(val) }

	#[inline(always)]
	/// Shuffle the components of a [`Point`].
	pub fn shuffle<const X: u32, const Y: u32, const Z: u32>(self) -> Self
	where
		[(); shuffle_args(X, Y, Z, 3)]: Sized,
		[(); _MM_SHUFFLE(3, Z, Y, X) as usize]: Sized,
	{
		Self(self.0.shuffle::<X, Y, Z, 3>())
	}

	#[inline(always)]
	/// Transform to the coordinate system.
	pub fn transform_to(self, system: &CoordinateSystem) -> Point
	{
		Self::new(
			Vector::dot(self.0, system.x.0),
			Vector::dot(self.0, system.y.0),
			Vector::dot(self.0, system.z.0),
		)
	}

	#[inline(always)]
	/// Transform from the coordinate system.
	pub fn transform_from(self, system: &CoordinateSystem) -> Point
	{
		Self(system.x.0 * self.x() + system.y.0 * self.y() + system.z.0 * self.z())
	}

	#[inline(always)]
	/// Linear interpolate from `from` to `to` with a factor `t`.
	pub fn lerp(from: Point, to: Point, t: f32) -> Point { Point(Vector::lerp(from.0, to.0, t)) }
}
