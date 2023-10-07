use crate::skills::Skill;

use eris_macro::{ErisDisplayEmoji, ErisValidEnum};

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Copy, ErisValidEnum, ErisDisplayEmoji,
)]
pub enum Classes {
    #[emoji("⚔️")]
    Warrior,
    #[emoji("🧙‍♂️")]
    Wizard,
    #[emoji("🧙‍♀️")]
    Sorcerer,
    #[emoji("🛡️")]
    Paladin,
}

impl Classes {
    pub fn armor_scaling(&self) -> f64 {
        match self {
            Classes::Warrior => 0.8,
            Classes::Wizard => 0.1,
            Classes::Sorcerer => 0.1,
            Classes::Paladin => 0.6,
        }
    }
    pub fn action(&self) -> Skill {
        match self {
            Classes::Warrior => Skill::Slash,
            Classes::Wizard => Skill::MagicMissile,
            Classes::Sorcerer => Skill::FireBall,
            Classes::Paladin => Skill::Rapture,
        }
    }
}
