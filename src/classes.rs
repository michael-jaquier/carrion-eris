use crate::player::PlayerAction;
use crate::CarrionError;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Classes {
    Warrior,
    Wizard,
    Sorcerer,
}

impl Classes {
    pub fn valid_classes() -> String {
        let all = vec![Classes::Warrior, Classes::Wizard, Classes::Sorcerer];
        all.iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join("\n ")
    }
    pub fn action(&self) -> PlayerAction {
        match self {
            Classes::Warrior => PlayerAction::Slash,
            Classes::Wizard => PlayerAction::MagicMissile,
            Classes::Sorcerer => PlayerAction::FireBall,
        }
    }

    pub fn hp_gain(&self, level: u32) -> u32 {
        match self {
            Classes::Warrior => 100 + (level * 25),
            Classes::Wizard => 75 + (level * 10),
            Classes::Sorcerer => 75 + (level * 15),
        }
    }
}

impl TryFrom<String> for Classes {
    type Error = CarrionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "warrior" => Ok(Classes::Warrior),
            "wizard" => Ok(Classes::Wizard),
            "sorcerer" => Ok(Classes::Sorcerer),
            _ => Err(CarrionError::ParseError(
                "Unable to parse class".to_string(),
            )),
        }
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Classes::Warrior => write!(f, "Warrior 🪖"),
            Classes::Wizard => write!(f, "Wizard 🧙"),
            Classes::Sorcerer => write!(f, "Sorcerer 🧙"),
        }
    }
}
