use super::*;

use crate::TILE_SIZE;

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    value as f32 * TILE_SIZE
}

pub fn grid_tile_center_to_world(value: i32) -> f32 {
    grid_tile_edge_to_world(value) + TILE_SIZE / 2.0
}

pub fn grid_tile_to_navmesh_index(value: i32) -> usize {
    (value + GRID_SIZE_HALF) as usize
}

pub fn navmesh_index_to_grid_tile(value: usize) -> i32 {
    value as i32 - GRID_SIZE_HALF
}

pub fn world_pos_to_grid(value: f32) -> i32 {
    (value / TILE_SIZE).floor() as i32
}

pub trait WorldTranslationHelper {
    fn world_pos_to_grid(&self) -> IVec2;
}
pub trait GridTranslationHelper {
    fn grid_tile_edge_to_world(&self) -> Vec2;
    fn grid_tile_center_to_world(&self) -> Vec2;
}

impl WorldTranslationHelper for Vec2 {
    fn world_pos_to_grid(&self) -> IVec2 {
        IVec2::new(world_pos_to_grid(self.x), world_pos_to_grid(self.y))
    }
}

impl GridTranslationHelper for IVec2 {
    fn grid_tile_edge_to_world(&self) -> Vec2 {
        Vec2::new(
            grid_tile_edge_to_world(self.x),
            grid_tile_edge_to_world(self.y),
        )
    }

    fn grid_tile_center_to_world(&self) -> Vec2 {
        Vec2::new(
            grid_tile_center_to_world(self.x),
            grid_tile_center_to_world(self.y),
        )
    }
}
