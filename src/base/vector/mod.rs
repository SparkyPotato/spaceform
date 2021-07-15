//! SIMD row vectors.

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
mod x86;
use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Mul, MulAssign},
};

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
pub use x86::*;

#[cfg(not(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64"))))]
mod scalar;
#[cfg(not(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64"))))]
pub use scalar::*;

use crate::base::Matrix;

/// Function that restricts the input arguments to `shuffle` at compile time.
pub const fn shuffle_args(x: u32, y: u32, z: u32, w: u32) -> usize
{
	if x < 4 && y < 4 && z < 4 && w < 4
	{
		1
	}
	else
	{
		panic!("Shuffle arguments must be in the range [0, 3]")
	}
}

impl Debug for Vector
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
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

impl Mul<Matrix> for Vector
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Matrix) -> Self::Output
	{
		Vector::new(
			Self::dot(rhs.get_column(0), self),
			Self::dot(rhs.get_column(1), self),
			Self::dot(rhs.get_column(2), self),
			Self::dot(rhs.get_column(3), self),
		)
	}
}

impl MulAssign<Matrix> for Vector
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Matrix) { *self = *self * rhs }
}

impl Vector
{
	#[inline(always)]
	/// Get the square of the four-dimensional length of the [`Vector`].
	pub fn length_square(self) -> f32 { Self::dot(self, self) }

	#[inline(always)]
	/// Get the four-dimensional length of the [`Vector`].
	pub fn length(self) -> f32 { self.length_square().sqrt() }

	#[inline(always)]
	/// Get the normalized four-dimensional [`Vector`].
	pub fn normalize(self) -> Self { self / self.length() }

	#[inline(always)]
	/// Get the four-dimensional dot product of two [`Vector`]s.
	pub fn dot(lhs: Vector, rhs: Vector) -> f32 { (lhs * rhs).hsum() }

	#[inline(always)]
	/// Get the three-dimensional cross product of two [`Vector`]s.
	pub fn cross(lhs: Vector, rhs: Vector) -> Vector
	{
		let temp = lhs.shuffle::<1, 2, 0, 3>();
		temp * rhs.shuffle::<2, 0, 1, 3>() - (temp * rhs).shuffle::<1, 2, 0, 3>()
	}

	#[inline(always)]
	/// Clamp `val` between `min_val` and `max_val`.
	pub fn clamp(val: Vector, min_val: Vector, max_val: Vector) -> Vector { min(max(val, min_val), max_val) }

	#[inline(always)]
	/// Linear interpolate from `from` to `to` with a factor `t`.
	pub fn lerp(from: Vector, to: Vector, t: f32) -> Vector { from + (from - to) * t }
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn getters()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.x(), 1f32);
		assert_eq!(vec.y(), 2f32);
		assert_eq!(vec.z(), 3f32);
		assert_eq!(vec.w(), 4f32);
	}

	#[test]
	fn setters()
	{
		let mut vec = Vector::default();

		vec.set_x(1f32);
		vec.set_y(2f32);
		vec.set_z(3f32);
		vec.set_w(4f32);

		assert_eq!(vec, Vector::new(1f32, 2f32, 3f32, 4f32));
	}

	#[test]
	fn add()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec + vec, Vector::new(2f32, 4f32, 6f32, 8f32));
	}

	#[test]
	fn subtract()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec - vec, Vector::default());
		assert_eq!(-vec, Vector::new(-1f32, -2f32, -3f32, -4f32))
	}

	#[test]
	fn multiply()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec * 2f32, Vector::new(2f32, 4f32, 6f32, 8f32));
		assert_eq!(vec * vec, Vector::new(1f32, 4f32, 9f32, 16f32));
	}

	#[test]
	fn divide()
	{
		let vec = Vector::new(2f32, 4f32, 6f32, 8f32);

		assert_eq!(vec / 2f32, Vector::new(1f32, 2f32, 3f32, 4f32));
		assert_eq!(vec / vec, Vector::new(1f32, 1f32, 1f32, 1f32));
	}

	#[test]
	fn equality()
	{
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec1, vec2);
	}

	#[test]
	fn index()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.get(0), vec.x());
		assert_eq!(vec.get(1), vec.y());
		assert_eq!(vec.get(2), vec.z());
		assert_eq!(vec.get(3), vec.w());
	}

	#[test]
	fn shuffle()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.shuffle::<0, 0, 0, 0>(), Vector::new(1f32, 1f32, 1f32, 1f32));
		assert_eq!(vec.shuffle::<1, 2, 3, 0>(), Vector::new(2f32, 3f32, 4f32, 1f32));
		assert_eq!(vec.shuffle::<3, 3, 3, 3>(), Vector::new(4f32, 4f32, 4f32, 4f32));
	}

	#[test]
	fn shuffle_merge()
	{
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(
			Vector::shuffle_merge::<0, 0, 0, 0>(vec1, vec2),
			Vector::new(1f32, 1f32, 4f32, 4f32)
		);
	}

	#[test]
	fn abs()
	{
		let vec = Vector::new(-1f32, 2f32, -3f32, 4f32);

		assert_eq!(vec.abs(), Vector::new(1f32, 2f32, 3f32, 4f32));
	}

	#[test]
	fn horizontal_sum()
	{
		let vec = Vector::new(-1f32, 2f32, -3f32, 4f32);
		assert_eq!(vec.hsum(), 2f32);

		let vec = Vector::new(1f32, 2f32, 3f32, 0f32);
		assert_eq!(vec.hsum(), 6f32);
	}

	#[test]
	fn length()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);
		assert_eq!(vec.length_square(), 30f32);

		let vec = Vector::new(3f32, 4f32, 0f32, 0f32);
		assert_eq!(vec.length(), 5f32);
	}

	#[test]
	fn cross_product()
	{
		let vec1 = Vector::new(1f32, 0f32, 0f32, 0f32);
		let vec2 = Vector::new(0f32, 1f32, 0f32, 0f32);

		assert_eq!(Vector::cross(vec1, vec2), Vector::new(0f32, 0f32, 1f32, 0f32));
	}

	#[test]
	fn min_and_max()
	{
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(min(vec1, vec2), Vector::new(1f32, 2f32, 2f32, 1f32));
		assert_eq!(max(vec1, vec2), Vector::new(4f32, 3f32, 3f32, 4f32));
	}
}
