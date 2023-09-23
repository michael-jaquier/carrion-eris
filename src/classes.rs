use crate::skills::Skill;
use crate::CarrionError;
use eris_macro::{ErisDisplayEmoji, ErisValidEnum};
use heck::ToTitleCase;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
            _ => Skill::Slash,
        }
    }
}
