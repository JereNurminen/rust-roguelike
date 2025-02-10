use super::combat::Damage;
use super::types::EntityId;

pub struct Armor {}

pub enum HandsEquipment {
    TwoHanded(Option<EntityId>),
    OneHanded {
        left: Option<EntityId>,
        right: Option<EntityId>,
    },
}

pub struct Equipment {
    pub armor: Armor,
    pub hands: HandsEquipment,
}

pub struct Inventory {
    pub items: Vec<EntityId>,
}

#[derive(PartialEq, Clone)]
pub enum ItemKind {
    Weapon { damage: Vec<Damage> },
    Armor { defense: u64 },
}
