//! Coordinate systems.

use crate::Direction;

#[derive(Clone, Copy)]
/// One of the six axis directions.
pub enum Axis
{
	/// +x
	PosX,
	/// +y
	PosY,
	/// +z
	PosZ,
	/// -x
	NegX,
	/// -y
	NegY,
	/// -z
	NegZ,
}

impl Into<Direction> for Axis
{
	fn into(self) -> Direction
	{
		use Axis::*;

		match self
		{
			PosX => Direction::new(1f32, 0f32, 0f32),
			NegX => Direction::new(-1f32, 0f32, 0f32),
			PosY => Direction::new(0f32, 1f32, 0f32),
			NegY => Direction::new(0f32, -1f32, 0f32),
			PosZ => Direction::new(0f32, 0f32, 1f32),
			NegZ => Direction::new(0f32, 0f32, -1f32),
		}
	}
}

#[derive(Clone, Copy)]
/// A mapping from axis directions to human understanding.
pub struct AxisMapping
{
	/// The axis direction that points to the right.
	pub right: Axis,
	/// The axis direction that points forward.
	pub forward: Axis,
	/// The axis direction that points up.
	pub up: Axis,
}
