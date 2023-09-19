use crate::dice::{Dice, Die, DieObject};
use crate::enemies::Alignment;
use crate::units::Attributes;
use crate::CarrionError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use tracing::warn;

pub enum TraitMutation {
    FlatIncrease(Vec<DieObject>),
    FlatDecrease(Vec<DieObject>),
    MultiplicativeBonus(f64),

    Advantage,
    Disadvantage,

    Repeat(usize),
    AlignmentBonus(Alignment, Box<TraitMutation>),

    CriticalAdvantage,
    CriticalDamage(f64),
}

pub struct TraitMutations {
    magic_attack: Vec<TraitMutation>,
    physical_attack: Vec<TraitMutation>,
    dodge: Vec<TraitMutation>,
    suppress: Vec<TraitMutation>,
    armor: Vec<TraitMutation>,
}

impl Display for TraitMutation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TraitMutation::FlatIncrease(dice) => write!(f, "FlatIncrease({:?})", dice),
            TraitMutation::FlatDecrease(dice) => write!(f, "FlatDecrease({:?})", dice),
            TraitMutation::MultiplicativeBonus(multiplier) => {
                write!(f, "MultiplicativeBonus({})", multiplier)
            }
            TraitMutation::Advantage => write!(f, "Advantage"),
            TraitMutation::Disadvantage => write!(f, "Disadvantage"),
            TraitMutation::Repeat(times) => write!(f, "Repeat({})", times),
            TraitMutation::AlignmentBonus(alignment, mutation) => {
                write!(f, "AlignmentBonus({:?}, {})", alignment, mutation)
            }
            TraitMutation::CriticalAdvantage => write!(f, "CriticalAdvantage"),
            TraitMutation::CriticalDamage(damage) => write!(f, "CriticalDamage({})", damage),
        }
    }
}

impl TraitMutations {
    pub fn set_magic_attack(&mut self, tr: TraitMutation) {
        self.magic_attack.push(tr)
    }
    fn dodge_check(&self, tr: &TraitMutation) -> bool {
        let dice_check =
            |dice: &Vec<DieObject>| dice.iter().map(|d| d.get_sides() == 20).all(|d| d);

        match tr {
            TraitMutation::FlatIncrease(dice) => {
                if !dice_check(&dice) {
                    warn!("Invalid Mutation for dodge {}. Skipping", tr);
                    return false;
                }
            }
            TraitMutation::FlatDecrease(dice) => {
                if !dice_check(&dice) {
                    warn!("Invalid Mutation for dodge {}. Skipping", tr);
                    return false;
                }
            }
            TraitMutation::Repeat(_) => {
                warn!("Invalid Mutation for dodge {}. Skipping", tr);
                return false;
            }

            TraitMutation::CriticalDamage(_) => {
                warn!("Invalid Mutation for dodge {}. Skipping", tr);
                return false;
            }
            TraitMutation::CriticalAdvantage => {
                warn!("Invalid Mutation for dodge {}. Skipping", tr);
                return false;
            }
            TraitMutation::AlignmentBonus(_, _) => {
                warn!("Invalid Mutation for dodge {}. Skipping", tr);
                return false;
            }
            _ => return true,
        }
        true
    }
    pub fn set_dodge(&mut self, tr: TraitMutation) {
        let valid_dodge = match &tr {
            _ => self.dodge_check(&tr),
        };
        if !valid_dodge {
            return;
        }

        self.dodge.push(tr)
    }
    pub fn set_physical_attack(&mut self, tr: TraitMutation) {
        self.physical_attack.push(tr)
    }
    pub fn set_armor(&mut self, tr: TraitMutation) {
        self.armor.push(tr)
    }
    pub fn set_suppress(&mut self, tr: TraitMutation) {
        self.suppress.push(tr)
    }

    pub fn get_armor(&self) -> &Vec<TraitMutation> {
        &self.armor
    }
    pub fn get_dodge(&self) -> &Vec<TraitMutation> {
        &self.dodge
    }
    pub fn get_physical_attack(&self) -> &Vec<TraitMutation> {
        &self.physical_attack
    }
    pub fn get_magical_attack(&self) -> &Vec<TraitMutation> {
        &self.magic_attack
    }
    pub fn get_suppress(&self) -> &Vec<TraitMutation> {
        &self.suppress
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

    pub fn apply_traits(traits: &HashSet<CharacterTraits>) -> TraitMutations {
        let mut trait_mutations = TraitMutations::default();
        for tr in traits {
            match tr {
                CharacterTraits::Robust => {
                    trait_mutations.set_armor(TraitMutation::MultiplicativeBonus(1.1));
                    trait_mutations.set_armor(TraitMutation::FlatIncrease(vec![Die::D4.into(); 1]))
                }
                CharacterTraits::Nimble => {
                    trait_mutations.set_dodge(TraitMutation::Advantage);
                    trait_mutations.set_dodge(TraitMutation::MultiplicativeBonus(1.2))
                }
                CharacterTraits::Genius => {
                    trait_mutations.set_magic_attack(TraitMutation::FlatIncrease(vec![
                        Die::D4
                            .into();
                        3
                    ]));
                    trait_mutations.set_magic_attack(TraitMutation::MultiplicativeBonus(1.15));
                }
                CharacterTraits::Lucky => {
                    trait_mutations.set_armor(TraitMutation::Advantage);
                    trait_mutations.set_dodge(TraitMutation::Advantage);
                    trait_mutations.set_suppress(TraitMutation::Advantage);
                }
                CharacterTraits::FolkHero => {
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticEvil,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::NeutralEvil,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::LawfulEvil,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                }

                CharacterTraits::Charismatic => {}
                CharacterTraits::Strong => {
                    trait_mutations.set_physical_attack(TraitMutation::FlatIncrease(vec![
                        Die::D4
                            .into(
                            );
                        3
                    ]));
                    trait_mutations.set_physical_attack(TraitMutation::MultiplicativeBonus(1.15));
                }

                CharacterTraits::Hermit => {
                    trait_mutations.set_suppress(TraitMutation::Advantage);
                    trait_mutations.set_suppress(TraitMutation::MultiplicativeBonus(1.2));
                }

                CharacterTraits::Addict => {
                    trait_mutations
                        .set_dodge(TraitMutation::FlatDecrease(vec![Die::D20.into(); 1]));
                }

                CharacterTraits::Cursed => {
                    trait_mutations
                        .set_suppress(TraitMutation::FlatDecrease(vec![Die::D6.into(); 2]));
                    trait_mutations.set_magic_attack(TraitMutation::CriticalDamage(0.85));
                    trait_mutations.set_magic_attack(TraitMutation::MultiplicativeBonus(0.9))
                }
                CharacterTraits::Unlucky => {
                    trait_mutations.set_suppress(TraitMutation::Disadvantage);
                    trait_mutations.set_dodge(TraitMutation::Disadvantage);
                }
                CharacterTraits::Righteous => {
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticEvil,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticNeutral,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticGood,
                        Box::from(TraitMutation::FlatIncrease(vec![Die::D6.into(); 2])),
                    ));
                }
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    trait_mutations.set_magic_attack(TraitMutation::CriticalDamage(1.35));
                    trait_mutations.set_physical_attack(TraitMutation::CriticalDamage(1.35));
                    trait_mutations.set_magic_attack(TraitMutation::CriticalAdvantage);
                    trait_mutations.set_physical_attack(TraitMutation::CriticalAdvantage);
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
