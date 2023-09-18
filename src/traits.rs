use crate::units::Attributes;
use crate::CarrionError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash)]
pub enum CharacterTraits {
    Robust,
    Nimble,
    Genius,
    Lucky,
    FolkHero,
    Charismatic,
    Strong,
    Hermit,
    Addict,
    Cursed,
    Unlucky,
    Righteous,
    Greedy,
    Keen,
}

impl Display for CharacterTraits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterTraits::Robust => write!(f, "Robust"),
            CharacterTraits::Nimble => write!(f, "Nimble"),
            CharacterTraits::Genius => write!(f, "Genius"),
            CharacterTraits::Lucky => write!(f, "Lucky"),
            CharacterTraits::FolkHero => write!(f, "FolkHero"),
            CharacterTraits::Charismatic => write!(f, "Charismatic"),
            CharacterTraits::Strong => write!(f, "Strong"),
            CharacterTraits::Hermit => write!(f, "Hermit"),
            CharacterTraits::Addict => write!(f, "Addict"),
            CharacterTraits::Cursed => write!(f, "Cursed"),
            CharacterTraits::Unlucky => write!(f, "Unlucky"),
            CharacterTraits::Righteous => write!(f, "Righteous"),
            CharacterTraits::Greedy => write!(f, "Greedy"),
            CharacterTraits::Keen => write!(f, "Keen"),
        }
    }
}

impl CharacterTraits {
    pub fn attribute_mutator(&self, character_attributes: &mut Attributes) {
        match self {
            CharacterTraits::Robust => {
                character_attributes.strength.plus(1);
                character_attributes.constitution.plus(3);
            }
            CharacterTraits::Nimble => {
                character_attributes.dexterity.plus(3);
            }
            CharacterTraits::Genius => {
                character_attributes.intelligence.plus(3);
            }
            CharacterTraits::Lucky => {}
            CharacterTraits::FolkHero => {
                character_attributes.charisma.plus(2);
                character_attributes.constitution.plus(1);
            }
            CharacterTraits::Charismatic => {
                character_attributes.charisma.plus(3);
            }
            CharacterTraits::Strong => {
                character_attributes.strength.plus(3);
            }
            CharacterTraits::Hermit => {
                character_attributes.wisdom.plus(3);
            }
            CharacterTraits::Addict => {
                character_attributes.constitution.minus(3);
                character_attributes.intelligence.plus(5);
            }
            CharacterTraits::Cursed => {}
            CharacterTraits::Unlucky => {}
            CharacterTraits::Righteous => {}
            CharacterTraits::Greedy => {}
            CharacterTraits::Keen => {}
        }
    }

    pub fn valid_traits() -> String {
        let all = vec![
            CharacterTraits::Robust,
            CharacterTraits::Nimble,
            CharacterTraits::Genius,
            CharacterTraits::Lucky,
            CharacterTraits::FolkHero,
            CharacterTraits::Charismatic,
            CharacterTraits::Strong,
            CharacterTraits::Hermit,
            CharacterTraits::Addict,
            CharacterTraits::Cursed,
            CharacterTraits::Unlucky,
            CharacterTraits::Righteous,
            CharacterTraits::Greedy,
            CharacterTraits::Keen,
        ];
        all.iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("\n ")
    }
}

impl TryFrom<String> for CharacterTraits {
    type Error = CarrionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "robust" => Ok(CharacterTraits::Robust),
            "nimble" => Ok(CharacterTraits::Nimble),
            "genius" => Ok(CharacterTraits::Genius),
            "lucky" => Ok(CharacterTraits::Lucky),
            "folkhero" => Ok(CharacterTraits::FolkHero),
            "charismatic" => Ok(CharacterTraits::Charismatic),
            "strong" => Ok(CharacterTraits::Strong),
            "hermit" => Ok(CharacterTraits::Hermit),
            "addict" => Ok(CharacterTraits::Addict),
            "cursed" => Ok(CharacterTraits::Cursed),
            "unlucky" => Ok(CharacterTraits::Unlucky),
            "righteous" => Ok(CharacterTraits::Righteous),
            "greedy" => Ok(CharacterTraits::Greedy),
            "keen" => Ok(CharacterTraits::Keen),
            _ => Err(CarrionError::ParseError(
                "Unable to parse character trait".to_string(),
            )),
        }
    }
}
