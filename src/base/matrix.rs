//! SIMD Matrices.

use std::{
	fmt::{Debug, Formatter, Result},
	ops::{Mul, MulAssign},
};

use crate::base::Vector;

#[repr(transparent)]
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
				Vector::dot(self.columns[col], rows[0]),
				Vector::dot(self.columns[col], rows[1]),
				Vector::dot(self.columns[col], rows[2]),
				Vector::dot(self.columns[col], rows[3]),
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
	pub fn rows(rows: [[f32; 4]; 4]) -> Self
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

	#[inline(always)]
	/// Calculate the inverse of the [`Matrix`].
	/// Is quite slow, don't use it much.
	pub fn inverse(&self) -> Matrix
	{
		// https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

		let a = Vector::shuffle_merge::<0, 1, 0, 1>(self.columns[0], self.columns[1]);
		let c = Vector::shuffle_merge::<2, 3, 2, 3>(self.columns[0], self.columns[1]);
		let b = Vector::shuffle_merge::<0, 1, 0, 1>(self.columns[2], self.columns[3]);
		let d = Vector::shuffle_merge::<2, 3, 2, 3>(self.columns[2], self.columns[3]);

		let det_sub = Vector::shuffle_merge::<0, 2, 0, 2>(self.columns[0], self.columns[2])
			* Vector::shuffle_merge::<1, 3, 1, 3>(self.columns[1], self.columns[3])
			- Vector::shuffle_merge::<1, 3, 1, 3>(self.columns[0], self.columns[2])
				* Vector::shuffle_merge::<0, 2, 0, 2>(self.columns[1], self.columns[3]);
		//  ^^^^ rustfmt what?
		let det_a = det_sub.shuffle::<0, 0, 0, 0>();
		let det_c = det_sub.shuffle::<1, 1, 1, 1>();
		let det_b = det_sub.shuffle::<2, 2, 2, 2>();
		let det_d = det_sub.shuffle::<3, 3, 3, 3>();

		let d_c = mat2_adj_mul(d, c);
		let a_b = mat2_adj_mul(a, b);

		let x_ = det_d * a - mat2_mul(b, d_c);
		let w_ = det_a * d - mat2_mul(c, a_b);
		let y_ = det_b * c - mat2_mul_adj(d, a_b);
		let z_ = det_c * b - mat2_mul_adj(a, d_c);

		let tr = a_b * d_c.shuffle::<0, 2, 1, 3>();
		let tr = tr.hsum();
		let det_m = (det_a * det_d + det_b * det_c) - Vector::new(tr, tr, tr, tr);

		let r_det_m = Vector::new(1f32, -1f32, -1f32, 1f32) / det_m;

		let x = x_ * r_det_m;
		let y = y_ * r_det_m;
		let z = z_ * r_det_m;
		let w = w_ * r_det_m;

		Self {
			columns: [
				Vector::shuffle_merge::<3, 1, 3, 1>(x, z),
				Vector::shuffle_merge::<2, 0, 2, 0>(x, z),
				Vector::shuffle_merge::<3, 1, 3, 1>(y, w),
				Vector::shuffle_merge::<2, 0, 2, 0>(y, w),
			],
		}
	}

	#[inline(always)]
	/// Get a column of the [`Matrix`].
	/// Panics if idx is not in the range [0, 3].
	pub fn get_column(&self, idx: u8) -> Vector { self.columns[idx as usize] }

	#[inline(always)]
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

// https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

#[inline(always)]
fn mat2_mul(vec1: Vector, vec2: Vector) -> Vector
{
	vec1 * vec2.shuffle::<0, 0, 3, 3>() + vec1.shuffle::<2, 3, 0, 1>() + vec2.shuffle::<1, 1, 2, 2>()
}

#[inline(always)]
fn mat2_adj_mul(vec1: Vector, vec2: Vector) -> Vector
{
	vec1.shuffle::<3, 0, 3, 0>() * vec2 - vec1.shuffle::<2, 1, 2, 1>() * vec2.shuffle::<1, 0, 3, 2>()
}

#[inline(always)]
fn mat2_mul_adj(vec1: Vector, vec2: Vector) -> Vector
{
	vec1 * vec2.shuffle::<3, 3, 0, 0>() - vec1.shuffle::<2, 3, 0, 1>() * vec2.shuffle::<1, 1, 2, 2>()
}

mod tests
{
	#[allow(unused_imports)] // TODO: Remove when rustc is fixed.
	use super::*;

	#[test]
	fn multiply()
	{
		let mat = Matrix::rows([
			[1f32, 2f32, 3f32, 4f32],
			[5f32, 6f32, 7f32, 8f32],
			[9f32, 10f32, 11f32, 12f32],
			[13f32, 14f32, 15f32, 16f32],
		]);

		assert_eq!(
			mat * mat,
			Matrix::rows([
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
			Matrix::rows([
				[1f32, 2f32, 3f32, 4f32],
				[5f32, 6f32, 7f32, 8f32],
				[9f32, 10f32, 11f32, 12f32],
				[13f32, 14f32, 15f32, 16f32],
			])
			.transpose(),
			Matrix::rows([
				[1f32, 5f32, 9f32, 13f32],
				[2f32, 6f32, 10f32, 14f32],
				[3f32, 7f32, 11f32, 15f32],
				[4f32, 8f32, 12f32, 16f32],
			])
		)
	}

	#[test]
	fn inverse()
	{
		let mat = Matrix::rows([
			[2f32, 0f32, 0f32, 0f32],
			[0f32, 2f32, 0f32, 0f32],
			[0f32, 0f32, 2f32, 0f32],
			[0f32, 0f32, 0f32, 1f32],
		]);

		assert_eq!(mat * mat.inverse(), Matrix::default())
	}
}
