use std::fmt::Display;

use specta::Type;
use ts_rs::TS;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, serde::Serialize, TS, Type)]
#[ts(export)]
pub struct WorldPosition {
    pub x: i32,
    pub y: i32,
}

impl WorldPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &WorldPosition) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn manhattan_distance(&self, other: &WorldPosition) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors(&self) -> Vec<WorldPosition> {
        vec![
            WorldPosition::new(self.x + 1, self.y),
            WorldPosition::new(self.x - 1, self.y),
            WorldPosition::new(self.x, self.y + 1),
            WorldPosition::new(self.x, self.y - 1),
        ]
    }
}

impl From<WorldPosition> for (f32, f32) {
    fn from(pos: WorldPosition) -> Self {
        (pos.x as f32, pos.y as f32)
    }
}

impl Display for WorldPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
