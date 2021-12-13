//! SIMD Matrices.

use core::f32;
use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{Mul, MulAssign},
};

use crate::base::Vector;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
/// A 4x4 matrix.
pub struct Matrix {
	rows: [Vector; 4],
}

impl Debug for Matrix {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl Default for Matrix {
	#[inline(always)]
	fn default() -> Self { Matrix::identity() }
}

impl Display for Matrix {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl Mul for Matrix {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self {
		let mut rows = [Vector::default(); 4];
		for i in 0..4 {
			rows[i] = rhs.rows[0] * self.rows[i].shuffle::<0, 0, 0, 0>()
				+ rhs.rows[1] * self.rows[i].shuffle::<1, 1, 1, 1>()
				+ rhs.rows[2] * self.rows[i].shuffle::<2, 2, 2, 2>()
				+ rhs.rows[3] * self.rows[i].shuffle::<3, 3, 3, 3>()
		}

		Self { rows }
	}
}

impl MulAssign for Matrix {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl Matrix {
	#[inline(always)]
	/// Create a [`Matrix`] from 16 elements.
	pub fn rows(rows: [[f32; 4]; 4]) -> Self {
		Self {
			rows: [
				Vector::new(rows[0][0], rows[0][1], rows[0][2], rows[0][3]),
				Vector::new(rows[1][0], rows[1][1], rows[1][2], rows[1][3]),
				Vector::new(rows[2][0], rows[2][1], rows[2][2], rows[2][3]),
				Vector::new(rows[3][0], rows[3][1], rows[3][2], rows[3][3]),
			],
		}
	}

	#[inline(always)]
	/// Create a [`Matrix`] from 16 elements.
	pub fn row_vectors(rows: [Vector; 4]) -> Self { Self { rows } }

	#[inline(always)]
	/// Create an identity [`Matrix`].
	pub fn identity() -> Self {
		Self {
			rows: [
				Vector::new(1f32, 0f32, 0f32, 0f32),
				Vector::new(0f32, 1f32, 0f32, 0f32),
				Vector::new(0f32, 0f32, 1f32, 0f32),
				Vector::new(0f32, 0f32, 0f32, 1f32),
			],
		}
	}

	#[inline(always)]
	/// Calculate the transpose of the [`Matrix`].
	pub fn transpose(&self) -> Matrix {
		let temp = [
			Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[0], self.rows[1]),
			Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[0], self.rows[1]),
			Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[2], self.rows[3]),
			Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[2], self.rows[3]),
		];

		Self {
			rows: [
				Vector::shuffle_merge::<0, 2, 0, 2>(temp[0], temp[2]),
				Vector::shuffle_merge::<1, 3, 1, 3>(temp[0], temp[2]),
				Vector::shuffle_merge::<0, 2, 0, 2>(temp[1], temp[3]),
				Vector::shuffle_merge::<1, 3, 1, 3>(temp[1], temp[3]),
			],
		}
	}

	#[inline(always)]
	/// Calculate the determinant of the [`Matrix`].
	/// Is quite slow, don't use it much.
	pub fn det(&self) -> f32 {
		// https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

		let a = Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[0], self.rows[1]);
		let c = Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[0], self.rows[1]);
		let b = Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[2], self.rows[3]);
		let d = Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[2], self.rows[3]);

		let det_sub = Vector::shuffle_merge::<0, 2, 0, 2>(self.rows[0], self.rows[2])
			* Vector::shuffle_merge::<1, 3, 1, 3>(self.rows[1], self.rows[3])
			- Vector::shuffle_merge::<1, 3, 1, 3>(self.rows[0], self.rows[2])
				* Vector::shuffle_merge::<0, 2, 0, 2>(self.rows[1], self.rows[3]);
		//  ^^^^ rustfmt what?
		let det_a = det_sub.shuffle::<0, 0, 0, 0>();
		let det_c = det_sub.shuffle::<1, 1, 1, 1>();
		let det_b = det_sub.shuffle::<2, 2, 2, 2>();
		let det_d = det_sub.shuffle::<3, 3, 3, 3>();

		let d_c = mat2_adj_mul(d, c);
		let a_b = mat2_adj_mul(a, b);

		let tr = a_b * d_c.shuffle::<0, 2, 1, 3>();
		let tr = tr.hsum();
		((det_a * det_d + det_b * det_c) - Vector::new(tr, tr, tr, tr)).x()
	}

	#[inline(always)]
	/// Calculate the inverse of the [`Matrix`].
	/// Is quite slow, don't use it much.
	pub fn inverse(&self) -> Matrix {
		// https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

		let a = Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[0], self.rows[1]);
		let c = Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[0], self.rows[1]);
		let b = Vector::shuffle_merge::<0, 1, 0, 1>(self.rows[2], self.rows[3]);
		let d = Vector::shuffle_merge::<2, 3, 2, 3>(self.rows[2], self.rows[3]);

		let det_sub = Vector::shuffle_merge::<0, 2, 0, 2>(self.rows[0], self.rows[2])
			* Vector::shuffle_merge::<1, 3, 1, 3>(self.rows[1], self.rows[3])
			- Vector::shuffle_merge::<1, 3, 1, 3>(self.rows[0], self.rows[2])
				* Vector::shuffle_merge::<0, 2, 0, 2>(self.rows[1], self.rows[3]);
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
			rows: [
				Vector::shuffle_merge::<3, 1, 3, 1>(x, z),
				Vector::shuffle_merge::<2, 0, 2, 0>(x, z),
				Vector::shuffle_merge::<3, 1, 3, 1>(y, w),
				Vector::shuffle_merge::<2, 0, 2, 0>(y, w),
			],
		}
	}

	#[inline(always)]
	/// Get a row of the [`Matrix`].
	/// Panics if idx is not in the range [0, 3].
	pub fn get_row(&self, idx: u8) -> Vector { self.rows[idx as usize] }

	#[inline(always)]
	/// Get a column of the [`Matrix`].
	/// Panics if idx is not in the range [0, 3].
	pub fn get_column(&self, idx: u8) -> Vector {
		Vector::new(
			self.rows[0].get(idx),
			self.rows[1].get(idx),
			self.rows[2].get(idx),
			self.rows[3].get(idx),
		)
	}
}

// https://lxjk.github.io/2017/09/03/Fast-4x4-Matrix-Inverse-with-SSE-SIMD-Explained.html

#[inline(always)]
fn mat2_mul(vec1: Vector, vec2: Vector) -> Vector {
	vec1 * vec2.shuffle::<0, 0, 3, 3>() + vec1.shuffle::<2, 3, 0, 1>() + vec2.shuffle::<1, 1, 2, 2>()
}

#[inline(always)]
fn mat2_adj_mul(vec1: Vector, vec2: Vector) -> Vector {
	vec1.shuffle::<3, 0, 3, 0>() * vec2 - vec1.shuffle::<2, 1, 2, 1>() * vec2.shuffle::<1, 0, 3, 2>()
}

#[inline(always)]
fn mat2_mul_adj(vec1: Vector, vec2: Vector) -> Vector {
	vec1 * vec2.shuffle::<3, 3, 0, 0>() - vec1.shuffle::<2, 3, 0, 1>() * vec2.shuffle::<1, 1, 2, 2>()
}

mod tests {
	#[allow(unused_imports)] // TODO: Remove when rustc is fixed.
	use super::*;

	#[test]
	fn multiply() {
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
	fn transpose() {
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
	fn inverse() {
		let mat = Matrix::rows([
			[2f32, 0f32, 0f32, 0f32],
			[0f32, 2f32, 0f32, 0f32],
			[0f32, 0f32, 2f32, 0f32],
			[0f32, 0f32, 0f32, 1f32],
		]);

		assert_eq!(mat * mat.inverse(), Matrix::default())
	}
}
