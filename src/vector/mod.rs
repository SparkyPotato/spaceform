//! SIMD Vectors.

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
mod x86;
#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
pub use x86::*;

#[cfg(not(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64"))))]
mod scalar;
#[cfg(not(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64"))))]
pub use scalar::*;

/// Clamp `val` between `min_val` and `max_val`.
pub fn clamp(val: Vector, min_val: Vector, max_val: Vector) -> Vector { min(max(val, min_val), max_val) }

/// Linear interpolate from `from` to `to` with a factor `t`.
pub fn lerp(from: Vector, to: Vector, t: f32) -> Vector { from + (from - to) * t }

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
	fn shuffle()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);

		assert_eq!(vec.shuffle::<0, 0, 0, 0>(), Vector::new(1f32, 1f32, 1f32, 1f32));
		assert_eq!(vec.shuffle::<1, 2, 3, 0>(), Vector::new(2f32, 3f32, 4f32, 1f32));
		assert_eq!(vec.shuffle::<3, 3, 3, 3>(), Vector::new(4f32, 4f32, 4f32, 4f32));
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
		assert_eq!(vec.hsum3(), -2f32);
		assert_eq!(vec.hsum4(), 2f32);

		let vec = Vector::new(1f32, 2f32, 3f32, 0f32);
		assert_eq!(vec.hsum3(), 6f32);
		assert_eq!(vec.hsum4(), 6f32);
	}

	#[test]
	fn length()
	{
		let vec = Vector::new(1f32, 2f32, 3f32, 4f32);
		assert_eq!(vec.length3_square(), 14f32);
		assert_eq!(vec.length4_square(), 30f32);

		let vec = Vector::new(3f32, 4f32, 0f32, 0f32);
		assert_eq!(vec.length3(), 5f32);
		assert_eq!(vec.length4(), 5f32);
	}

	#[test]
	fn cross_product()
	{
		let vec1 = Vector::new(1f32, 0f32, 0f32, 0f32);
		let vec2 = Vector::new(0f32, 1f32, 0f32, 0f32);

		assert_eq!(cross(vec1, vec2), Vector::new(0f32, 0f32, 1f32, 0f32));
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
