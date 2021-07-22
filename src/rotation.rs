//! Rotations.

use crate::{base::Quaternion, coordinate_system::AxisMapping, Direction};

/// The order to apply euler rotations in.
pub enum RotationOrder
{
	/// Pitch, yaw, and then roll.
	PYR,
	/// Pitch, roll, yaw.
	PRY,
	/// Roll, pitch, yaw.
	RPY,
	/// Roll, yaw, pitch.
	RYP,
	/// Yaw, pitch, roll.
	YPR,
	/// Yaw, roll, pitch.
	YRP,
}

/// A rotation described by euler angles in radians. Positive angles convey an anti-clockwise rotation.
pub struct EulerAngles
{
	/// Rotation along the `right` vector while looking along the vector, from the origin.
	pub pitch: f32,
	/// Rotation along the `up` vector.
	pub yaw: f32,
	/// Rotation along the `forward` vector.
	pub roll: f32,
	/// Order of rotation.
	pub order: RotationOrder,
}

#[derive(Copy, Clone, PartialEq)]
/// A rotation in 3D space.
pub struct Rotation(pub(crate) Quaternion);

impl Rotation
{
	/// Create a [`Rotation`] from [`EulerAngles`].
	pub fn euler(angles: EulerAngles, mapping: AxisMapping) -> Self
	{
		let sin_pitch = (angles.pitch / 2f32).sin();
		let cos_pitch = (angles.pitch / 2f32).cos();
		let sin_yaw = (angles.yaw / 2f32).sin();
		let cos_yaw = (angles.yaw / 2f32).cos();
		let sin_roll = (angles.roll / 2f32).sin();
		let cos_roll = (angles.roll / 2f32).cos();

		let right: Direction = mapping.right.into();
		let forward: Direction = mapping.forward.into();
		let up: Direction = mapping.up.into();

		let mut pitch = Quaternion(right.0 * sin_pitch);
		pitch.set_w(cos_pitch);
		let mut yaw = Quaternion(up.0 * sin_yaw);
		yaw.set_w(cos_yaw);
		let mut roll = Quaternion(forward.0 * sin_roll);
		roll.set_w(cos_roll);

		use RotationOrder::*;

		Self(match angles.order
		{
			PYR => pitch * yaw * roll,
			PRY => pitch * roll * yaw,
			RPY => roll * pitch * yaw,
			RYP => roll * yaw * pitch,
			YPR => yaw * pitch * roll,
			YRP => yaw * roll * pitch,
		})
	}
}
