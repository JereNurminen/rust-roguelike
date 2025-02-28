pub type TurnNumber = u64;

#[derive(PartialEq, Clone, Debug)]
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

impl Dice {
    pub fn sides(&self) -> u64 {
        match self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::D100 => 100,
            Dice::Flat(sides) => *sides,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct DieRoll {
    count: u64,
    dice: Dice,
    modifier: i64,
}

impl DieRoll {
    fn roll(&self) -> i64 {
        let roll_result = match self.dice {
            Dice::Flat(sides) => sides * self.count,
            _ => {
                // Repeat this process count times
                (0..self.count)
                    .map(|_| rand::random_range(1..self.dice.sides()))
                    .sum()
            }
        };
        roll_result as i64 + self.modifier
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    //NorthEast,
    East,
    //SouthEast,
    South,
    //SouthWest,
    West,
    //NorthWest,
}
