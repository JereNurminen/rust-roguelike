pub type TurnNumber = u64;

pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
    Flat(u64),
}

pub struct DieRoll {
    count: u64,
    dice: Dice,
    modifier: i64,
}
