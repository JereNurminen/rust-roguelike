use crate::core::types::DieRoll;

#[derive(PartialEq, Clone, Debug)]
pub enum DamageType {
    Slice,
    Pierce,
    Blunt,
    Fire,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: DieRoll,
}
