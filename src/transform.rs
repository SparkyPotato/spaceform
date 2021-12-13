//! Affine transformations.

use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Mul, MulAssign},
};

use crate::{
	base::{Matrix, Vector},
	Direction,
	Rotation,
};

#[repr(C)]
#[derive(Clone, Copy)]
/// An affine transformation that can be applied to [`crate::Point`]s, [`Direction`]s, and [`crate::Normal`]s.
pub struct Transform {
	pub(crate) matrix: Matrix,
	pub(crate) inverse: Matrix,
}

impl Debug for Transform {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.matrix) }
}

impl Default for Transform {
	#[inline(always)]
	fn default() -> Self { Self::identity() }
}

impl Display for Transform {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}", self.matrix) }
}

impl Mul for Transform {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		Self {
			matrix: self.matrix * rhs.matrix,
			inverse: rhs.inverse * self.inverse,
		}
	}
}

impl MulAssign for Transform {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs }
}

impl Transform {
	#[inline(always)]
	/// Get the identity [`Transform`] that leaves everything unchanged.
	pub fn identity() -> Self {
		Self {
			matrix: Matrix::identity(),
			inverse: Matrix::identity(),
		}
	}

	#[inline(always)]
	/// Get a translation [`Transform`].
	pub fn translate(dir: Direction) -> Transform {
		Self {
			matrix: Matrix::row_vectors([
				Vector::new(1f32, 0f32, 0f32, 0f32),
				Vector::new(0f32, 1f32, 0f32, 0f32),
				Vector::new(0f32, 0f32, 1f32, 0f32),
				dir.0 + Vector::new(0f32, 0f32, 0f32, 1f32),
			]),
			inverse: Matrix::row_vectors([
				Vector::new(1f32, 0f32, 0f32, 0f32),
				Vector::new(0f32, 1f32, 0f32, 0f32),
				Vector::new(0f32, 0f32, 1f32, 0f32),
				-dir.0 + Vector::new(0f32, 0f32, 0f32, 1f32),
			]),
		}
	}

	#[inline(always)]
	/// Get a scaling [`Transform`].
	pub fn scale(scale: Direction) -> Transform {
		let inv = Vector::new(1f32, 1f32, 1f32, 1f32) / scale.0;

		Self {
			matrix: Matrix::rows([
				[scale.x(), 0f32, 0f32, 0f32],
				[0f32, scale.y(), 0f32, 0f32],
				[0f32, 0f32, scale.z(), 0f32],
				[0f32, 0f32, 0f32, 1f32],
			]),
			inverse: Matrix::rows([
				[inv.x(), 0f32, 0f32, 0f32],
				[0f32, inv.y(), 0f32, 0f32],
				[0f32, 0f32, inv.z(), 0f32],
				[0f32, 0f32, 0f32, 1f32],
			]),
		}
	}

	#[inline(always)]
	/// Get a rotation [`Transform`].
	pub fn rotate(rotation: Rotation) -> Transform {
		let x = rotation.0.x();
		let y = rotation.0.y();
		let z = rotation.0.z();
		let w = rotation.0.w();

		let matrix = Matrix::rows([
			[
				1f32 - 2f32 * (y * y + z * z),
				2f32 * (x * y + z * w),
				2f32 * (x * z - y * w),
				0f32,
			],
			[
				2f32 * (x * y - z * w),
				1f32 - 2f32 * (x * x + z * z),
				2f32 * (y * z + x * w),
				0f32,
			],
			[
				2f32 * (x * z + y * w),
				2f32 * (y * z - x * w),
				1f32 - 2f32 * (x * x + y * y),
				0f32,
			],
			[0f32, 0f32, 0f32, 1f32],
		]);

		Self {
			matrix,
			inverse: matrix.transpose(),
		}
	}

	#[inline(always)]
	/// Get the inverse of the [`Transform`].
	/// Is quite fast (faster than [`Matrix::inverse`]).
	pub fn inverse(&self) -> Self {
		Self {
			matrix: self.inverse,
			inverse: self.matrix,
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::{Direction, Point};

	#[test]
	fn translation() {
		let point = Point::new(0f32, 0f32, 0f32);

		assert_eq!(
			point * Transform::translate(Direction::new(5f32, 5f32, 5f32)),
			Point::new(5f32, 5f32, 5f32)
		);
		assert_eq!(
			point * Transform::translate(Direction::new(5f32, 5f32, 5f32)).inverse(),
			Point::new(-5f32, -5f32, -5f32)
		);
	}

	#[test]
	fn scale() {
		let point = Point::new(1f32, 1f32, 1f32);

		assert_eq!(
			point * Transform::scale(Direction::new(5f32, 5f32, 5f32)),
			Point::new(5f32, 5f32, 5f32)
		);
		assert_eq!(
			point * Transform::scale(Direction::new(5f32, 5f32, 5f32)).inverse(),
			Point::new(1f32 / 5f32, 1f32 / 5f32, 1f32 / 5f32)
		);
	}
}
