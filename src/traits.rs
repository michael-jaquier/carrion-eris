use std::collections::HashSet;
use crate::units::Attributes;
use crate::CarrionError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::dice::{Dice, Die, DieObject};
use crate::enemies::Alignment;

pub enum TraitMutation {
    FlatIncrease(Vec<DieObject>),
    FlatDecrease(Vec<DieObject>),
    MultiplicativeBonus(f64),


    Advantage,
    Disadvantage,

    Repeat(usize),
    AlignmentBonus(Alignment, TraitMutation),

    CriticalChance(f64),
    CriticalDamage(f64),
}

pub struct TraitMutations
{
    magic_attack: Vec<TraitMutation>,
    physical_attack: Vec<TraitMutation>,
    dodge: Vec<TraitMutation>,
    suppress: Vec<TraitMutation>,
    armor: Vec<TraitMutation>

}

impl TraitMutations
{
    pub fn push_magic_attack(&mut self, tr: TraitMutation)
    {
        self.magic_attack.push(tr)
    }
    pub fn push_dodge(&mut self, tr: TraitMutation)
    {
        self.dodge.push(tr)
    }
    pub fn push_physical_attack(&mut self, tr: TraitMutation)
    {
        self.physical_attack.push(tr)
    }
    pub fn push_armor(&mut self, tr: TraitMutation)
    {
        self.armor.push(tr)
    }
    pub fn push_suppress(&mut self, tr: TraitMutation)
    {
        self.suppress.push(tr)
    }

}

impl Default for TraitMutations {
    fn default() -> Self {
        Self {
            magic_attack: vec![],
            dodge: vec![],
            armor: vec![],
            physical_attack: vec![],
            suppress: vec![],
        }
    }
}

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

    pub fn apply_traits(&self, traits: HashSet<CharacterTraits>) -> TraitMutations
    {
        let mut trait_mutations = TraitMutations::default();
        for tr in traits {
            match tr {
                CharacterTraits::Robust => {
                    trait_mutations.push_armor(TraitMutation::MultiplicativeBonus(1.1));
                    trait_mutations.push_armor(TraitMutation::FlatIncrease(vec![Die::D4.into();1]))
                }
                CharacterTraits::Nimble => {
                    trait_mutations.push_dodge(TraitMutation::Advantage);
                    trait_mutations.push_dodge(TraitMutation::MultiplicativeBonus(1.2))
                }
                CharacterTraits::Genius => {
                    trait_mutations.push_magic_attack(TraitMutation::FlatIncrease(vec![Die::D4.into();3]));
                    trait_mutations.push_magic_attack(TraitMutation::MultiplicativeBonus(1.15));
                }
                CharacterTraits::Lucky => {
                    trait_mutations.push_armor(TraitMutation::Advantage);
                    trait_mutations.push_dodge(TraitMutation::Advantage);
                    trait_mutations.push_suppress(TraitMutation::Advantage);
                }
                CharacterTraits::FolkHero => {
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::ChaoticEvil, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::NeutralEvil, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::LawfulEvil, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));

                }

                CharacterTraits::Charismatic => {}
                CharacterTraits::Strong => {
                    trait_mutations.push_physical_attack(TraitMutation::FlatIncrease(vec![Die::D4.into();3]));
                    trait_mutations.push_physical_attack(TraitMutation::MultiplicativeBonus(1.15));
                }

                CharacterTraits::Hermit => {
                    trait_mutations.push_suppress(TraitMutation::Advantage);
                    trait_mutations.push_suppress(TraitMutation::MultiplicativeBonus(1.2));
                }

                CharacterTraits::Addict => {
                    trait_mutations.push_dodge(TraitMutation::FlatDecrease(vec![Die::D6.into();2]));

                }

                CharacterTraits::Cursed => {
                    trait_mutations.push_suppress(TraitMutation::FlatDecrease(vec![Die::D6.into();2]));
                    trait_mutations.magic_attack(TraitMutation::CriticalDamage(0.85));
                    trait_mutations.magic_attack(TraitMutation::MultiplicativeBonus(0.9))

                }
                CharacterTraits::Unlucky => {
                    trait_mutations.push_suppress(TraitMutation::Disadvantage);
                    trait_mutations.push_dodge(TraitMutation::Disadvantage);
                }
                CharacterTraits::Righteous => {
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::ChaoticEvil, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::ChaoticNeutral, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));
                    trait_mutations.push_magic_attack(TraitMutation::AlignmentBonus(Alignment::ChaoticGood, TraitMutation::FlatIncrease(vec![Die::D6.into();2])));

                }
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    trait_mutations.push_magic_attack(TraitMutation::CriticalDamage(1.35));
                    trait_mutations.push_physical_attack(TraitMutation::CriticalDamage(1.35));
                    trait_mutations.push_magic_attack(TraitMutation::CriticalChance(1.35));
                    trait_mutations.push_physical_attack(TraitMutation::CriticalChance(1.35));
                }
            }
        }


        trait_mutations
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
