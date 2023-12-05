use crate::{skill::Skill, AttributeScaling};

use eris_macro::{AttributeScaling, ErisDisplayEmoji, ErisValidEnum};

use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
    Copy,
    ErisValidEnum,
    ErisDisplayEmoji,
    AttributeScaling,
    Hash,
    Eq,
)]
pub enum Classes {
    #[stat("strength")]
    #[emoji("⚔️")]
    Warrior,
    #[stat("intelligence")]
    #[emoji("🧙‍♂️")]
    Wizard,
    #[stat("wisdom")]
    #[emoji("🧙‍♀️")]
    Sorcerer,
    #[stat("charisma")]
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
    pub fn scaling(&self) -> String {
        AttributeScaling::scaling(self).unwrap()
    }
}
