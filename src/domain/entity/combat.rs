use crate::core::types::DieRoll;

#[derive(PartialEq)]
pub enum DamageType {
    Slice,
    Pierce,
    Blunt,
    Fire,
}

#[derive(PartialEq)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: DieRoll,
}

#[derive(PartialEq)]
pub enum PotionEffect {
    Heal(DieRoll),
    Poison(DieRoll),
}
