//! SIMD row vectors.

use std::{
	fmt::{Debug, Display, Formatter, Result},
	ops::{AddAssign, DivAssign, Mul, MulAssign, Neg, SubAssign},
};

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
mod x86;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
pub use x86::*;

#[cfg(all(feature = "simd", target_arch = "wasm32"))]
mod wasm;
#[cfg(all(feature = "simd", target_arch = "wasm32"))]
pub use wasm::*;

#[cfg(not(all(
	feature = "simd",
	any(target_arch = "x86", target_arch = "x86_64", target_arch = "wasm32")
)))]
mod scalar;
#[cfg(not(all(
	feature = "simd",
	any(target_arch = "x86", target_arch = "x86_64", target_arch = "wasm32")
)))]
pub use scalar::*;

use crate::base::Matrix;

impl AddAssign for Vector {
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl Debug for Vector {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl Display for Vector {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "[{}, {}, {}, {}]", self.x(), self.y(), self.z(), self.w())
	}
}

impl DivAssign for Vector {
	#[inline(always)]
	fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

impl DivAssign<f32> for Vector {
	#[inline(always)]
	fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; }
}

impl From<[f32; 4]> for Vector {
	#[inline(always)]
	fn from(val: [f32; 4]) -> Self { Vector::new(val[0], val[1], val[2], val[3]) }
}

impl MulAssign for Vector {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl MulAssign<f32> for Vector {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; }
}

impl Mul<Matrix> for Vector {
	type Output = Self;

	#[inline(always)]
	fn mul(self, rhs: Matrix) -> Self::Output {
		rhs.get_row(0) * self.shuffle::<0, 0, 0, 0>()
			+ rhs.get_row(1) * self.shuffle::<1, 1, 1, 1>()
			+ rhs.get_row(2) * self.shuffle::<2, 2, 2, 2>()
			+ rhs.get_row(3) * self.shuffle::<3, 3, 3, 3>()
	}
}

impl MulAssign<Matrix> for Vector {
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Matrix) { *self = *self * rhs }
}

impl Neg for Vector {
	type Output = Self;

	#[inline(always)]
	fn neg(self) -> Self { Self::default() - self }
}

impl SubAssign for Vector {
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Into<[f32; 4]> for Vector {
	#[inline(always)]
	fn into(self) -> [f32; 4] { [self.x(), self.y(), self.z(), self.w()] }
}

impl Vector {
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
	pub fn cross(lhs: Vector, rhs: Vector) -> Vector {
		let temp = lhs.shuffle::<1, 2, 0, 3>();
		temp * rhs.shuffle::<2, 0, 1, 3>() - (temp * rhs).shuffle::<1, 2, 0, 3>()
	}

	#[inline(always)]
	/// Clamp `val` between `min_val` and `max_val`.
	pub fn clamp(val: Vector, min_val: Vector, max_val: Vector) -> Vector {
		Vector::min(Vector::max(val, min_val), max_val)
	}

	#[inline(always)]
	/// Linear interpolate from `from` to `to` with a factor `t`.
	pub fn lerp(from: Vector, to: Vector, t: f32) -> Vector { from + (from - to) * t }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn getters() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.x(), 1f32);
		assert_eq!(vec.y(), 2f32);
		assert_eq!(vec.z(), 3f32);
		assert_eq!(vec.w(), 4f32);

		assert_eq!(vec.get(0), 1f32);
		assert_eq!(vec.get(1), 2f32);
		assert_eq!(vec.get(2), 3f32);
		assert_eq!(vec.get(3), 4f32);
	}

	#[test]
	fn setters() {
		let mut vec = Vector::default();

		vec.set_x(1f32);
		vec.set_y(2f32);
		vec.set_z(3f32);
		vec.set_w(4f32);

		assert_eq!(vec, Vector::new(1f32, 2f32, 3f32, 4f32));
	}

	#[test]
	fn add() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec + vec, Vector::new(2f32, 4f32, 6f32, 8f32));
	}

	#[test]
	fn subtract() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec - vec, Vector::default());
		assert_eq!(-vec, Vector::new(-1f32, -2f32, -3f32, -4f32))
	}

	#[test]
	fn multiply() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);
		let mat = Matrix::rows([
			[1f32, 2f32, 3f32, 4f32],
			[5f32, 6f32, 7f32, 8f32],
			[9f32, 10f32, 11f32, 12f32],
			[13f32, 14f32, 15f32, 16f32],
		]);

		assert_eq!(vec * 2f32, Vector::new(2f32, 4f32, 6f32, 8f32));
		assert_eq!(vec * vec, Vector::new(1f32, 4f32, 9f32, 16f32));
		assert_eq!(vec * mat, Vector::new(90f32, 100f32, 110f32, 120f32));
	}

	#[test]
	fn divide() {
		let vec = Vector::new(2f32, 4f32, 6f32, 8f32);

		assert_eq!(vec / 2f32, Vector::new(1f32, 2f32, 3f32, 4f32));
		assert_eq!(vec / vec, Vector::new(1f32, 1f32, 1f32, 1f32));
	}

	#[test]
	fn equality() {
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec1, vec2);
	}

	#[test]
	fn index() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.get(0), vec.x());
		assert_eq!(vec.get(1), vec.y());
		assert_eq!(vec.get(2), vec.z());
		assert_eq!(vec.get(3), vec.w());
	}

	#[test]
	fn shuffle() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.shuffle::<0, 0, 0, 0>(), Vector::new(1f32, 1f32, 1f32, 1f32));
		assert_eq!(vec.shuffle::<1, 2, 3, 0>(), Vector::new(2f32, 3f32, 4f32, 1f32));
		assert_eq!(vec.shuffle::<3, 3, 3, 3>(), Vector::new(4f32, 4f32, 4f32, 4f32));
	}

	#[test]
	fn shuffle_merge() {
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(
			Vector::shuffle_merge::<0, 0, 0, 0>(vec1, vec2),
			Vector::new(1f32, 1f32, 4f32, 4f32)
		);
	}

	#[test]
	fn abs() {
		let vec = Vector::new(-1f32, 2f32, -3f32, 4f32);

		assert_eq!(vec.abs(), Vector::new(1f32, 2f32, 3f32, 4f32));
	}

	#[test]
	fn horizontal_sum() {
		let vec = Vector::new(-1f32, 2f32, -3f32, 4f32);
		assert_eq!(vec.hsum(), 2f32);

		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);
		assert_eq!(vec.hsum(), 10f32);
	}

	#[test]
	fn length() {
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);
		assert_eq!(vec.length_square(), 30f32);

		let vec = Vector::new(3f32, 4f32, 0f32, 0f32);
		assert_eq!(vec.length(), 5f32);
	}

	#[test]
	fn cross_product() {
		let vec1 = Vector::new(1f32, 0f32, 0f32, 0f32);
		let vec2 = Vector::new(0f32, 1f32, 0f32, 0f32);

		assert_eq!(Vector::cross(vec1, vec2), Vector::new(0f32, 0f32, 1f32, 0f32));
	}

	#[test]
	fn min_and_max() {
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(Vector::min(vec1, vec2), Vector::new(1f32, 2f32, 2f32, 1f32));
		assert_eq!(Vector::max(vec1, vec2), Vector::new(4f32, 3f32, 3f32, 4f32));
	}

	#[test]
	fn adj_add_and_sub() {
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(Vector::adj_add(vec1, vec2), Vector::new(3f32, 7f32, 7f32, 3f32));
		assert_eq!(Vector::adj_sub(vec1, vec2), Vector::new(-1f32, -1f32, 1f32, 1f32));
	}

	#[test]
	fn add_sub() {
		let vec1 = Vector::new(1f32, 2f32, 3f32, 4f32);
		let vec2 = Vector::new(4f32, 3f32, 2f32, 1f32);

		assert_eq!(Vector::add_sub(vec1, vec2), Vector::new(-3f32, 5f32, 1f32, 5f32));
	}
}
