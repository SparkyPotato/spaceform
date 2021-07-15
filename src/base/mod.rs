//! These are the base mathematical types, which are then abstracted by more usable types.

pub mod matrix;
pub mod quaternion;
pub mod vector;

use core::f32;

pub use matrix::*;
pub use quaternion::*;
pub use vector::*;

#[inline(always)]
/// Calculate if two floats are equal to each other with a given `epsilon`.
pub fn nearly_equal(lhs: f32, rhs: f32, epsilon: f32) -> bool { (rhs - lhs).abs() < epsilon }
