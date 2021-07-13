// Warnings
#![allow(incomplete_features)] // TODO: Remove once const_evaluatable_checked and const_generics are complete.
// Features
#![feature(array_map)]
#![feature(array_zip)]
#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
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
