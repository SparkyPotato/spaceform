// Warnings
#![allow(incomplete_features)] // TODO: Remove once generic_const_exprs is complete.
// Features
#![feature(generic_const_exprs)]
// Rustdoc
#![warn(missing_docs)]
// Clippy
#![warn(clippy::all, clippy::nursery)]

//! spaceform is a SIMD-accelerated library for 3D graphics.

pub mod base;
pub mod coordinate_system;
pub mod direction;
pub mod normal;
pub mod point;
pub mod rotation;
pub mod transform;

pub use direction::Direction;
pub use normal::Normal;
pub use point::Point;
pub use rotation::{EulerAngles, Rotation, RotationOrder};
pub use transform::Transform;

/// Check if an argument is valid to pass into `shuffle`.
pub const fn is_shuffle_arg(x: u32, y: u32, z: u32, w: u32) -> bool { x < 4 && y < 4 && z < 4 && w < 4 }

/// Get the x86 shuffle mask, because of const generic limitations.
pub const fn shuffle_mask(z: u32, y: u32, x: u32, w: u32) -> i32 { ((z << 6) | (y << 4) | (x << 2) | w) as i32 }

// Rust SFINAE :)
/// Check if a const expression is true or false.
///
/// Implements [`True`] if it is, [`False`] otherwise.
pub struct Check<const V: bool>;
/// A true expression.
pub trait True {}
/// A false expression.
pub trait False {}

impl True for Check<true> {}
impl False for Check<false> {}
