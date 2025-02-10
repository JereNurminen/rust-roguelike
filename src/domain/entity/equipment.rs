use super::types::EntityId;
use super::combat::Damage;

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

#[derive(PartialEq)]
pub enum ItemKind {
    Weapon { damage: Vec<Damage> },
    Armor { defense: u64 },
    Potion { effect: super::combat::PotionEffect },
}
