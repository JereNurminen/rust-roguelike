pub mod entity;
mod position;

use std::collections::{HashMap, HashSet};

pub use entity::Entity;
use entity::{EntityId, EntityKind};
pub use position::WorldPosition;

use crate::core::types::TurnNumber;

pub struct World {
    entities: HashMap<EntityId, Entity>,
    current_turn: TurnNumber,
    last_entity_id: EntityId,
}

impl World {
    pub fn new(entities: Vec<Entity>) -> Self {
        Self {
            entities: HashMap::new(),
            current_turn: 0,
            last_entity_id: 0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        /*
        for (_, entity) in &mut self.entities {
            entity.update_animation(dt);
        }
        */
    }

    pub fn get_next_entity_id(&mut self) -> EntityId {
        let new_id = self.last_entity_id.overflowing_add(1);
        if self.entities().contains_key(&new_id.0) {
            panic!("Duplicate entity id")
        }
        match new_id {
            (id, false) => {
                self.last_entity_id = id;
                id
            }
            _ => panic!("Entity ID overflow"),
        }
    }

    pub fn get_entity_by_id(&self, entity_id: &EntityId) -> Option<&Entity> {
        self.entities.get(entity_id)
    }

    pub fn get_entities_by_pos(&self, pos: &WorldPosition) -> Vec<&Entity> {
        self.entities
            .values()
            .into_iter()
            .filter(|e| e.position().is_some_and(|entity_pos| entity_pos == *pos))
            .collect()
    }

    pub fn get_entity_by_id_mut(&mut self, entity_id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(entity_id)
    }

    pub fn get_entities_by_pos_mut(&mut self, pos: &WorldPosition) -> Vec<&mut Entity> {
        self.entities
            .values_mut()
            .into_iter()
            .filter(|e| e.position().is_some_and(|entity_pos| entity_pos == *pos))
            .collect()
    }

    pub fn can_move_to(&self, entity: &Entity, pos: &WorldPosition) -> bool {
        let entities_in_target_pos = self.get_entities_by_pos(&pos);
        // TODO: Actual logic for checking if entity can move to target position
        true
    }

    pub fn move_entity(&mut self, entity_id: &EntityId, target: &WorldPosition) {
        if let Some(entity) = self.get_entity_by_id_mut(&entity_id) {
            entity.set_position(Some(*target));
        }
    }

    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }

    fn get_visible_positions(&self, from: &WorldPosition, radius: i32) -> HashSet<WorldPosition> {
        let mut visible = HashSet::new();

        // Check every position within a square of size radius*2+1
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let to = WorldPosition {
                    x: from.x + dx,
                    y: from.y + dy,
                };

                // Skip if beyond radius
                if from.distance_to(&to) > radius as f32 {
                    continue;
                }

                if self.has_line_of_sight(&from, &to) {
                    visible.insert(to);
                }
            }
        }

        visible
    }

    // Check if there's a clear line of sight between two positions
    fn has_line_of_sight(&self, from: &WorldPosition, to: &WorldPosition) -> bool {
        // Always see your own position
        if from == to {
            return true;
        }

        let line = self.get_line(from, to);

        // Check each position along the line except the start and end
        for pos in line.iter().skip(1).take(line.len() - 2) {
            let entities_in_pos = self.get_entities_by_pos(pos);
            if entities_in_pos.iter().any(
                |e| matches!(&e.kind(), EntityKind::Wall { material } if material.blocks_vision),
            ) {
                return false;
            }
        }

        true
    }

    // Bresenham's line algorithm
    fn get_line(&self, from: &WorldPosition, to: &WorldPosition) -> Vec<WorldPosition> {
        let mut line = Vec::new();

        let mut x = from.x;
        let mut y = from.y;

        let dx = (to.x - from.x).abs();
        let dy = (to.y - from.y).abs();

        let sx = if from.x < to.x { 1 } else { -1 };
        let sy = if from.y < to.y { 1 } else { -1 };

        let mut err = if dx > dy { dx } else { -dy } / 2;
        let mut err2;

        loop {
            line.push(WorldPosition { x, y });

            if x == to.x && y == to.y {
                break;
            }

            err2 = err;

            if err2 > -dx {
                err -= dy;
                x += sx;
            }

            if err2 < dy {
                err += dx;
                y += sy;
            }
        }

        line
    }
}
