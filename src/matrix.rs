//! SIMD Matrices.

use std::{
	fmt::{Debug, Formatter, Result},
	ops::{Mul, MulAssign},
};

use crate::vector::{dot4, Vector};

#[derive(Copy, Clone, PartialEq)]
/// A 4x4 matrix.
pub struct Matrix
{
	// Stored column major to make vector * matrix faster.
	columns: [Vector; 4],
}

impl Debug for Matrix
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result
	{
		write!(
			f,
			"{}, {}, {}, {}",
			self.get_row(0),
			self.get_row(1),
			self.get_row(2),
			self.get_row(3)
		)
	}
}

impl Default for Matrix
{
	#[inline(always)]
	fn default() -> Self { Matrix::identity() }
}

impl Mul for Matrix
{
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self
	{
		let rows = [
			Vector::new(
				rhs.columns[0].x(),
				rhs.columns[1].x(),
				rhs.columns[2].x(),
				rhs.columns[3].x(),
			),
			Vector::new(
				rhs.columns[0].y(),
				rhs.columns[1].y(),
				rhs.columns[2].y(),
				rhs.columns[3].y(),
			),
			Vector::new(
				rhs.columns[0].z(),
				rhs.columns[1].z(),
				rhs.columns[2].z(),
				rhs.columns[3].z(),
			),
			Vector::new(
				rhs.columns[0].w(),
				rhs.columns[1].w(),
				rhs.columns[2].w(),
				rhs.columns[3].w(),
			),
		];

		let mut columns = [Vector::default(); 4];

		for col in 0..4
		{
			columns[col] = Vector::new(
				dot4(self.columns[col], rows[0]),
				dot4(self.columns[col], rows[1]),
				dot4(self.columns[col], rows[2]),
				dot4(self.columns[col], rows[3]),
			);
		}

		Self { columns }
	}
}

impl MulAssign for Matrix
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl Matrix
{
	#[inline(always)]
	/// Create a [`Matrix`] from 16 elements.
	pub fn new(rows: [[f32; 4]; 4]) -> Self
	{
		Self {
			columns: [
				Vector::new(rows[0][0], rows[1][0], rows[2][0], rows[3][0]),
				Vector::new(rows[0][1], rows[1][1], rows[2][1], rows[3][1]),
				Vector::new(rows[0][2], rows[1][2], rows[2][2], rows[3][2]),
				Vector::new(rows[0][3], rows[1][3], rows[2][3], rows[3][3]),
			],
		}
	}

	#[inline(always)]
	/// Create an identity [`Matrix`].
	pub fn identity() -> Self
	{
		Self {
			columns: [
				Vector::new(1f32, 0f32, 0f32, 0f32),
				Vector::new(0f32, 1f32, 0f32, 0f32),
				Vector::new(0f32, 0f32, 1f32, 0f32),
				Vector::new(0f32, 0f32, 0f32, 1f32),
			],
		}
	}

	#[inline(always)]
	/// Calculate the transpose of the [`Matrix`].
	pub fn transpose(&self) -> Matrix
	{
		Matrix {
			columns: [
				Vector::new(
					self.columns[0].x(),
					self.columns[1].x(),
					self.columns[2].x(),
					self.columns[3].x(),
				),
				Vector::new(
					self.columns[0].y(),
					self.columns[1].y(),
					self.columns[2].y(),
					self.columns[3].y(),
				),
				Vector::new(
					self.columns[0].z(),
					self.columns[1].z(),
					self.columns[2].z(),
					self.columns[3].z(),
				),
				Vector::new(
					self.columns[0].w(),
					self.columns[1].w(),
					self.columns[2].w(),
					self.columns[3].w(),
				),
			],
		}
	}

	/// Get a column of the [`Matrix`].
	/// Panics if idx is not in the range [0, 3].
	pub fn get_column(&self, idx: u8) -> Vector { self.columns[idx as usize] }

	/// Get a row of the [`Matrix`].
	/// Panics if idx is not in the range [0, 3].
	pub fn get_row(&self, idx: u8) -> Vector
	{
		Vector::new(
			self.columns[0].get(idx),
			self.columns[1].get(idx),
			self.columns[2].get(idx),
			self.columns[3].get(idx),
		)
	}
}

mod tests
{
	use super::*;

	#[test]
	fn multiply()
	{
		let mat = Matrix::new([
			[1f32, 2f32, 3f32, 4f32],
			[5f32, 6f32, 7f32, 8f32],
			[9f32, 10f32, 11f32, 12f32],
			[13f32, 14f32, 15f32, 16f32],
		]);

		assert_eq!(
			mat * mat,
			Matrix::new([
				[90f32, 100f32, 110f32, 120f32],
				[202f32, 228f32, 254f32, 280f32],
				[314f32, 356f32, 398f32, 440f32],
				[426f32, 484f32, 542f32, 600f32],
			])
		);
	}

	#[test]
	fn transpose()
	{
		assert_eq!(
			Matrix::new([
				[1f32, 2f32, 3f32, 4f32],
				[5f32, 6f32, 7f32, 8f32],
				[9f32, 10f32, 11f32, 12f32],
				[13f32, 14f32, 15f32, 16f32],
			])
			.transpose(),
			Matrix::new([
				[1f32, 5f32, 9f32, 13f32],
				[2f32, 6f32, 10f32, 14f32],
				[3f32, 7f32, 11f32, 15f32],
				[4f32, 8f32, 12f32, 16f32],
			])
		)
	}
}
