#[derive(Clone)]
pub struct CoreAttributes {
    pub strength: u64,
    pub speed: u64,
    pub durability: u64,
    pub fortitude: u64,
    pub magic: u64,
}

impl CoreAttributes {
    pub fn new(strength: u64, speed: u64, durability: u64, fortitude: u64, magic: u64) -> Self {
        Self {
            strength,
            speed,
            durability,
            fortitude,
            magic,
        }
    }
}

impl Default for CoreAttributes {
    fn default() -> Self {
        Self {
            strength: 10,
            speed: 10,
            durability: 10,
            fortitude: 10,
            magic: 10,
        }
    }
}

pub struct Stats {
    pub sight_radius: u64,
    pub hearing_threshold: u64,
}

#[derive(Clone)]
pub enum Exhaustion {
    WellRested,
    Rested,
    Normal,
    Tired,
    Exhausted,
}

#[derive(Clone)]
pub struct Status {
    pub health: u64,
    pub stamina: u64,
    pub mana: u64,
    pub exhaustion: Exhaustion,
}

pub struct StatVariance {
    pub low: u64,
    pub high: u64,
}

pub struct TemplateStat(pub u64, pub StatVariance);

pub struct CreatureTemplate {
    pub kind: CreatureRaceKind,
    pub max_health: TemplateStat,
    pub max_stamina: TemplateStat,
    pub max_mana: TemplateStat,
    pub stats: CoreAttributes,
}

pub enum CreatureRaceKind {
    Human,
    Goblin,
}
