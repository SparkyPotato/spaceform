// Warnings
#![allow(incomplete_features)] // TODO: Remove once const_evaluatable_checked and const_generics are complete.
// Features
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![feature(const_panic)]
#![feature(stdarch)]
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
