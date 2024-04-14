use super::*;

#[derive(Component, Debug)]
pub struct Tile(pub IVec2);

#[derive(Component)]
pub struct TileHovered;

#[derive(Event, Debug)]
pub struct HoverTileEvent(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickTileEvent(pub IVec2);
