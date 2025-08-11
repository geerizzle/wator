use super::config::{self, Config};

#[derive(Clone)]
pub enum Entity {
    Fish { age: u32 },
    Shark { age: u32, energy: u32 },
    Empty,
}

impl Entity {
    pub fn is_fish(&self) -> bool {
        matches!(self, Entity::Fish { .. })
    }

    pub fn is_shark(&self) -> bool {
        matches!(self, Entity::Shark { .. })
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Entity::Empty)
    }

    pub fn gain_energy(&mut self) {
        if let Entity::Shark { energy, .. } = self {
            *energy += 2;
        }
    }

    pub fn spawn_new(&self, config: Config) -> Self {
        match self {
            Entity::Fish { .. } => Entity::Fish { age: 0 },
            Entity::Shark { .. } => Entity::Shark {
                age: 0,
                energy: config.shark_energy,
            },
            Entity::Empty => Entity::Empty,
        }
    }

    pub fn can_reproduce(&self) -> bool {
        match self {
            Entity::Fish { age } => *age >= 3,
            Entity::Shark { age, .. } => *age >= 5,
            Entity::Empty => false,
        }
    }

    pub fn is_dead(&self, config: Config) -> bool {
        match self {
            Entity::Fish { age } => *age >= config.fish_age,
            Entity::Shark { age, energy } => *age >= config.shark_age || *energy == 0,
            Entity::Empty => false,
        }
    }

    pub fn deprive(&mut self) {
        match self {
            Entity::Fish { age } => {
                *age += 1;
            }
            Entity::Shark { age, energy } => {
                *age += 1;
                if *energy > 0 {
                    *energy -= 1;
                }
            }
            _ => (),
        }
    }
}
