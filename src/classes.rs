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
}

impl Classes {
    pub fn action(&self) -> Skill {
        match self {
            Classes::Warrior => Skill::Slash,
            Classes::Wizard => Skill::MagicMissile,
            Classes::Sorcerer => Skill::FireBall,
        }
    }
}
