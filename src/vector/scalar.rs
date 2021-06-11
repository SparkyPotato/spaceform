//! Implementation using scalar math only.

use core::f32;
use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone)]
/// A four-dimensional vector.
pub struct Vector
{
	data: [f32; 4],
}

impl Add for Vector
{
	type Output = Vector;

	fn add(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 + val.1),
		}
	}
}

impl AddAssign for Vector
{
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Debug for Vector
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl Default for Vector
{
	fn default() -> Self
	{
		Self {
			data: [0f32, 0f32, 0f32, 0f32],
		}
	}
}

impl Display for Vector
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl Div for Vector
{
	type Output = Vector;

	fn div(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 / val.1),
		}
	}
}

impl DivAssign for Vector
{
	fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl Div<f32> for Vector
{
	type Output = Vector;

	fn div(self, rhs: f32) -> Self
	{
		Self {
			data: self.data.map(|val| val / rhs),
		}
	}
}

impl DivAssign<f32> for Vector
{
	fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; }
}

impl Mul for Vector
{
	type Output = Vector;

	fn mul(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 * val.1),
		}
	}
}

impl MulAssign for Vector
{
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl Mul<f32> for Vector
{
	type Output = Vector;

	fn mul(self, rhs: f32) -> Self
	{
		Self {
			data: self.data.map(|val| val * rhs),
		}
	}
}

impl MulAssign<f32> for Vector
{
	fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; }
}

impl Neg for Vector
{
	type Output = Self;

	fn neg(self) -> Self { Self::default() - self }
}

impl PartialEq for Vector
{
	fn eq(&self, other: &Vector) -> bool { self.data == other.data }
}

impl Sub for Vector
{
	type Output = Vector;

	fn sub(self, rhs: Self) -> Self
	{
		Self {
			data: self.data.zip(rhs.data).map(|val| val.0 - val.1),
		}
	}
}

impl SubAssign for Vector
{
	fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Vector
{
	/// Create a [`Vector`] from x, y, z, and w values.
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { data: [x, y, z, w] } }

	/// Get the x value of the [`Vector`].
	pub fn x(self) -> f32 { self.data[0] }

	/// Get the y value of the [`Vector`].
	pub fn y(self) -> f32 { self.data[1] }

	/// Get the z value of the [`Vector`].
	pub fn z(self) -> f32 { self.data[2] }

	/// Get the w value
	pub fn w(self) -> f32 { self.data[3] }

	/// Set the x value of the [`Vector`].
	pub fn set_x(&mut self, val: f32) { self.data[0] = val }

	/// Set the y value of the [`Vector`].
	pub fn set_y(&mut self, val: f32) { self.data[1] = val }

	/// Set the z value of the [`Vector`].
	pub fn set_z(&mut self, val: f32) { self.data[2] = val }

	/// Set the w value of the [`Vector`].
	pub fn set_w(&mut self, val: f32) { self.data[3] = val }

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

	/// Get a [`Vector`] containing the absolute values of x, y, z, and w.
	pub fn abs(self) -> Self
	{
		Self {
			data: self.data.map(|val| val.abs()),
		}
	}

	/// Get the three-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum3(self) -> f32 { self.data[0] + self.data[1] + self.data[2] }

	/// Get the four-dimensional horizontal-sum of a [`Vector`].
	pub fn hsum4(self) -> f32 { self.data[0] + self.data[1] + self.data[2] + self.data[3] }

	/// Get the square of the four-dimensional length of the [`Vector`].
	pub fn length3_square(self) -> f32 { dot3(self, self) }

	/// Get the square of the four-dimensional length of the [`Vector`].
	pub fn length4_square(self) -> f32 { dot4(self, self) }

	/// Get the four-dimensional length of the [`Vector`].
	pub fn length3(self) -> f32 { self.length3_square().sqrt() }

	/// Get the four-dimensional length of the [`Vector`].
	pub fn length4(self) -> f32 { self.length4_square().sqrt() }

	/// Get the normalized four-dimensional length of the [`Vector`].
	pub fn normalize3(self) -> Vector { self / self.length3() }

	/// Get the normalized four-dimensional length of the [`Vector`].
	pub fn normalize4(self) -> Vector { self / self.length4() }
}

/// Get the three-dimensional dot product of two [`Vector`]s.
pub fn dot3(lhs: Vector, rhs: Vector) -> f32 { (lhs * rhs).hsum3() }

/// Get the four-dimensional dot product of two [`Vector`]s.
pub fn dot4(lhs: Vector, rhs: Vector) -> f32 { (lhs * rhs).hsum4() }

/// Get the three-dimensional cross product of two [`Vector`]s.
pub fn cross(lhs: Vector, rhs: Vector) -> Vector
{
	let temp = lhs.shuffle::<1, 2, 0, 3>();
	temp * rhs.shuffle::<2, 0, 1, 3>() - (temp * rhs).shuffle::<1, 2, 0, 3>()
}

/// Get the component-wise minimums.
pub fn min(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: lhs.data.zip(rhs.data).map(|val| f32::min(val.0, val.1)),
	}
}

/// Get the component-wise maximums.
pub fn max(lhs: Vector, rhs: Vector) -> Vector
{
	Vector {
		data: lhs.data.zip(rhs.data).map(|val| f32::max(val.0, val.1)),
	}
}
