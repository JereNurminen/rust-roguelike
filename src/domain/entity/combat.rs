use crate::core::types::DieRoll;

#[derive(PartialEq, Clone)]
pub enum DamageType {
    Slice,
    Pierce,
    Blunt,
    Fire,
}

#[derive(PartialEq, Clone)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: DieRoll,
}
