use crate::dice::{AdvantageState, Die, DieObject};
use crate::units::Alignment;
use crate::units::Attributes;
use crate::CarrionError;
use eris_macro::ErisValidEnum;
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

    ActionBonus(u32),
    // Alignment bonus is just damage or something related to more dice
    AlignmentBonus(Alignment, Vec<DieObject>),

    CriticalAdvantage,
    CriticalMultiplier(f64),
}

#[derive(Default)]
pub struct TraitMutations {
    magic_attack: Vec<TraitMutation>,
    physical_attack: Vec<TraitMutation>,
    dodge: Vec<TraitMutation>,
    suppress: Vec<TraitMutation>,
    armor: Vec<TraitMutation>,
    actions: Vec<TraitMutation>,
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
            TraitMutation::ActionBonus(times) => write!(f, "Action Bonus({})", times),
            TraitMutation::AlignmentBonus(alignment, mutation) => {
                write!(f, "AlignmentBonus({:?}, {:?})", alignment, mutation)
            }
            TraitMutation::CriticalAdvantage => write!(f, "CriticalAdvantage"),
            TraitMutation::CriticalMultiplier(damage) => {
                write!(f, "CriticalMultiplier({})", damage)
            }
        }
    }
}

impl TraitMutations {
    pub(crate) fn multi(mutations: &Vec<TraitMutation>) -> f64 {
        let mut multi = 1.0;
        for m in mutations {
            if let TraitMutation::MultiplicativeBonus(e) = m {
                multi *= e;
            }
        }
        multi
    }

    pub(crate) fn critical_advantage(mutations: &Vec<TraitMutation>) -> AdvantageState {
        let mut critical_advantage = AdvantageState::None;
        for m in mutations {
            if let TraitMutation::CriticalAdvantage = m {
                critical_advantage = AdvantageState::Advantage;
            }
        }
        critical_advantage
    }

    pub(crate) fn critical_multiplier(mutations: &Vec<TraitMutation>) -> f64 {
        let mut critical_multiplier = 1.0;
        for m in mutations {
            if let TraitMutation::CriticalMultiplier(e) = m {
                critical_multiplier *= e;
            }
        }
        critical_multiplier
    }
    pub(crate) fn advantage(mutations: &Vec<TraitMutation>) -> i32 {
        let mut advantage: i32 = 0;
        let map_advantage = |x: AdvantageState| match x {
            AdvantageState::Advantage => 1,
            AdvantageState::Disadvantage => -1,
            _ => 0,
        };
        for m in mutations {
            match m {
                TraitMutation::Advantage => {
                    advantage = map_advantage(AdvantageState::Advantage);
                }
                TraitMutation::Disadvantage => {
                    advantage = map_advantage(AdvantageState::Disadvantage);
                }
                _ => {}
            }
        }

        advantage
    }
    pub fn set_magic_attack(&mut self, tr: TraitMutation) {
        self.magic_attack.push(tr)
    }

    fn set_actions(&mut self, tr: TraitMutation) {
        self.actions.push(tr)
    }

    pub fn action_points(&self) -> u32 {
        let mut action_points = 0;
        for tr in &self.actions {
            if let TraitMutation::ActionBonus(times) = tr {
                action_points += times;
            }
        }
        action_points
    }
    fn dodge_check(&self, tr: &TraitMutation) -> bool {
        let dice_check =
            |dice: &Vec<DieObject>| dice.iter().map(|d| d.get_sides() == 20).all(|d| d);

        match tr {
            TraitMutation::FlatIncrease(dice) => {
                if !dice_check(dice) {
                    warn!("Invalid Mutation for dodge {}. Skipping", tr);
                    return false;
                }
            }
            TraitMutation::FlatDecrease(dice) => {
                if !dice_check(dice) {
                    warn!("Invalid Mutation for dodge {}. Skipping", tr);
                    return false;
                }
            }
            TraitMutation::ActionBonus(_) => {
                warn!("Invalid Mutation for dodge {}. Skipping", tr);
                return false;
            }

            TraitMutation::CriticalMultiplier(_) => {
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
        let valid_dodge = self.dodge_check(&tr);
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, ErisValidEnum)]
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
    Energetic,
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
            CharacterTraits::Energetic => write!(f, "Energetic"),
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
            CharacterTraits::Energetic => {}
        }
    }

    pub fn apply_traits(traits: &HashSet<CharacterTraits>) -> TraitMutations {
        let mut trait_mutations = TraitMutations::default();
        for tr in traits {
            match tr {
                CharacterTraits::Robust => {
                    trait_mutations.set_armor(TraitMutation::MultiplicativeBonus(1.1));
                    trait_mutations.set_armor(TraitMutation::FlatIncrease(vec![Die::D4.into(); 5]))
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
                        vec![Die::D6.into(); 2],
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::NeutralEvil,
                        vec![Die::D6.into(); 2],
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::LawfulEvil,
                        vec![Die::D6.into(); 2],
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
                    trait_mutations.set_magic_attack(TraitMutation::CriticalMultiplier(0.85));
                    trait_mutations.set_magic_attack(TraitMutation::MultiplicativeBonus(0.9))
                }
                CharacterTraits::Unlucky => {
                    trait_mutations.set_suppress(TraitMutation::Disadvantage);
                    trait_mutations.set_dodge(TraitMutation::Disadvantage);
                }
                CharacterTraits::Righteous => {
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticEvil,
                        vec![Die::D6.into(); 2],
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticNeutral,
                        vec![Die::D6.into(); 2],
                    ));
                    trait_mutations.set_magic_attack(TraitMutation::AlignmentBonus(
                        Alignment::ChaoticGood,
                        vec![Die::D6.into(); 2],
                    ));
                }
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    trait_mutations.set_magic_attack(TraitMutation::CriticalMultiplier(1.35));
                    trait_mutations.set_physical_attack(TraitMutation::CriticalMultiplier(1.35));
                    trait_mutations.set_magic_attack(TraitMutation::CriticalAdvantage);
                    trait_mutations.set_physical_attack(TraitMutation::CriticalAdvantage);
                }
                CharacterTraits::Energetic => {
                    trait_mutations.set_actions(TraitMutation::ActionBonus(1));
                }
            }
        }

        trait_mutations
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
            "energetic" => Ok(CharacterTraits::Energetic),
            _ => Err(CarrionError::ParseError(
                "Unable to parse character trait".to_string(),
            )),
        }
    }
}
