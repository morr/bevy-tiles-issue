use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PrevHoveredTilePos(pub Option<IVec2>);
