//! Coordinate systems.

use crate::Direction;

#[derive(Clone, Copy)]
/// A mapping from axis directions to human understanding.
pub struct CoordinateSystem
{
	/// The direction that points to the right.
	pub right: Direction,
	/// The direction that points forward.
	pub forward: Direction,
	/// The direction that points up.
	pub up: Direction,
}
