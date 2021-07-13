//! Coordinate systems.

use std::f32::consts::PI;

use crate::direction::Direction;

#[derive(Copy, Clone, PartialEq)]
/// A cartesian coordinate system, in terms of another coordinate system.
pub struct CoordinateSystem
{
	/// The `x` basis vector of the coordinate system.
	pub x: Direction,
	/// The `y` basis vector of the coordinate system.
	pub y: Direction,
	/// The `z` basis vector of the coordinate system.
	pub z: Direction,
}

impl Default for CoordinateSystem
{
	#[inline(always)]
	fn default() -> Self { Self::identity() }
}

impl CoordinateSystem
{
	#[inline(always)]
	/// The identity coordinate system.
	pub fn identity() -> Self
	{
		Self {
			x: Direction::new(1f32, 0f32, 0f32),
			y: Direction::new(0f32, 1f32, 0f32),
			z: Direction::new(0f32, 0f32, 1f32),
		}
	}

	#[inline(always)]
	/// Convert from a spherical coordinate system to a [`Direction`].
	/// # Arguments
	/// `sin`: Sine of the polar angle (theta).  
	/// `cos`: Cosine of the polar angle (theta).  
	/// `phi`: The azimuthal angle.
	/// # Returns
	/// A unit [`Direction`] with `Y` up.
	pub fn from_spherical(&self, sin: f32, cos: f32, phi: f32) -> Direction
	{
		self.x * sin * phi.cos() + self.y * cos + self.z * cos * phi.sin()
	}

	#[inline(always)]
	/// Convert to a spherical coordinate system from a unit [`Direction`].
	/// # Returns
	/// (theta, phi)
	pub fn to_spherical(&self, dir: Direction) -> (f32, f32)
	{
		(dir.y().acos(), {
			let p = dir.z().atan2(dir.x());
			if p < 0f32
			{
				p + 2f32 * PI
			}
			else
			{
				p
			}
		})
	}
}
