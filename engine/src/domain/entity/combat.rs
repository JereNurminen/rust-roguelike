use serde::Serialize;
use specta::Type;
use ts_rs::TS;

use crate::core::types::DieRoll;

#[derive(PartialEq, Clone, Debug, Serialize, TS, Type)]
#[ts(export)]
pub enum DamageType {
    Slice,
    Pierce,
    Blunt,
    Fire,
}

#[derive(PartialEq, Clone, Debug, Serialize, TS, Type)]
#[ts(export)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: DieRoll,
}
