use crate::unit::Alignment;
use crate::unit::Attributes;

use eris_macro::{ErisDisplayEmoji, ErisValidEnum};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::damage::{Damage, DamageBuilder, DamageType, ResistCategories};
use std::hash::Hash;

#[derive(Debug, Clone)]
pub enum TraitMutation {
    FlatIncrease(i32),
    FlatDecrease(i32),
    MultiplicativeBonus(f64),

    ActionBonus(i32),
    AlignmentBonus(Alignment, i32),
    CriticalMultiplier(f64),
    CriticalChance(f64),
}

#[derive(Clone, Default)]
pub struct TraitMutations {
    damage: HashMap<DamageType, Damage>,
    stat_damage: HashMap<String, Damage>,
    alignment_damage: HashMap<Alignment, Damage>,
    dodge: i32,
    suppress: HashMap<ResistCategories, i32>,
    armor: i32,
    actions: i32,
}

impl TraitMutations {
    fn set_actions(&mut self, tr: TraitMutation) {
        if let TraitMutation::ActionBonus(times) = tr {
            self.actions += times;
        }
    }

    fn set_dodge(&mut self, tr: TraitMutation) {
        match tr {
            TraitMutation::FlatIncrease(e) => self.dodge += e,
            TraitMutation::FlatDecrease(e) => self.dodge -= e,
            TraitMutation::MultiplicativeBonus(e) => self.dodge = (self.dodge as f64 * e) as i32,
            _ => {}
        }
    }

    fn set_armor(&mut self, tr: TraitMutation) {
        match tr {
            TraitMutation::FlatIncrease(e) => self.armor += e,
            TraitMutation::FlatDecrease(e) => self.armor -= e,
            TraitMutation::MultiplicativeBonus(e) => self.armor = (self.armor as f64 * e) as i32,
            _ => {}
        }
    }

    fn set_suppress(&mut self, resist: ResistCategories, tr: TraitMutation) {
        match tr {
            TraitMutation::FlatIncrease(e) => {
                self.suppress
                    .entry(resist)
                    .and_modify(|d| *d += e)
                    .or_insert(e);
            }
            TraitMutation::FlatDecrease(e) => {
                self.suppress
                    .entry(resist)
                    .and_modify(|d| *d -= e)
                    .or_insert(-e);
            }
            TraitMutation::MultiplicativeBonus(e) => {
                self.suppress
                    .entry(resist)
                    .and_modify(|d| *d = (*d as f64 * e) as i32)
                    .or_insert(0);
            }
            _ => {}
        }
    }

    pub fn action_points(&self) -> i32 {
        self.actions
    }

    pub fn get_armor(&self) -> i32 {
        self.armor
    }
    pub fn get_dodge(&self) -> i32 {
        self.dodge
    }

    pub fn get_suppress(&self, resist: ResistCategories) -> i32 {
        *self.suppress.get(&resist).unwrap_or(&0)
    }

    pub fn get_all_suppress(&self) -> HashMap<ResistCategories, i32> {
        self.suppress.clone()
    }

    pub fn get_damage(&self, dtype: DamageType) -> Damage {
        let universal = self
            .damage
            .get(&DamageType::Universal)
            .unwrap_or(&Damage::zero(dtype))
            .clone();
        match dtype.resist_category() {
            ResistCategories::Elemental => {
                let dtype_damage = self
                    .damage
                    .get(&dtype)
                    .unwrap_or(&Damage::zero(dtype))
                    .clone();
                let prismatic = self
                    .damage
                    .get(&DamageType::Prismatic)
                    .unwrap_or(&Damage::zero(dtype))
                    .clone();
                dtype_damage + universal + prismatic
            }
            ResistCategories::Universal => universal,
            _ => {
                let dtype_damage = self
                    .damage
                    .get(&dtype)
                    .unwrap_or(&Damage::zero(dtype))
                    .clone();
                dtype_damage + universal
            }
        }
    }

    fn set_alignment_damage(&mut self, alignment: Alignment, tr: TraitMutation) {
        let damage = DamageBuilder::default()
            .damage(0)
            .multiplier(0.0)
            .crit_chance(0.0)
            .critical_multiplier(0.0)
            .number_of_hits(0)
            .dtype(DamageType::Universal)
            .build()
            .unwrap();
        match tr {
            TraitMutation::FlatIncrease(e) => {
                self.alignment_damage
                    .entry(alignment)
                    .and_modify(|d| d.damage += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::FlatDecrease(e) => {
                self.alignment_damage
                    .entry(alignment)
                    .and_modify(|d| d.damage += -e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalChance(e) => {
                self.alignment_damage
                    .entry(alignment)
                    .and_modify(|d| d.crit_chance += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalMultiplier(e) => {
                self.alignment_damage
                    .entry(alignment)
                    .and_modify(|d| d.critical_multiplier += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::MultiplicativeBonus(e) => {
                self.alignment_damage
                    .entry(alignment)
                    .and_modify(|d| d.multiplier += e)
                    .or_insert(damage.clone());
            }
            _ => {}
        }
    }

    fn set_stat_damage(&mut self, stat: String, tr: TraitMutation) {
        let damage = DamageBuilder::default()
            .damage(0)
            .multiplier(0.0)
            .crit_chance(0.0)
            .critical_multiplier(0.0)
            .number_of_hits(0)
            .dtype(DamageType::Universal)
            .build()
            .unwrap();
        match tr {
            TraitMutation::FlatIncrease(e) => {
                self.stat_damage
                    .entry(stat)
                    .and_modify(|d| d.damage += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::FlatDecrease(e) => {
                self.stat_damage
                    .entry(stat)
                    .and_modify(|d| d.damage += -e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalChance(e) => {
                self.stat_damage
                    .entry(stat)
                    .and_modify(|d| d.crit_chance += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalMultiplier(e) => {
                self.stat_damage
                    .entry(stat)
                    .and_modify(|d| d.critical_multiplier += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::MultiplicativeBonus(e) => {
                self.stat_damage
                    .entry(stat)
                    .and_modify(|d| d.multiplier += e)
                    .or_insert(damage.clone());
            }
            _ => {}
        }
    }

    fn set_damage(&mut self, dtype: DamageType, tr: TraitMutation) {
        let damage = DamageBuilder::default()
            .damage(0)
            .multiplier(0.0)
            .crit_chance(0.0)
            .critical_multiplier(0.0)
            .number_of_hits(0)
            .dtype(dtype)
            .build()
            .unwrap();
        match tr {
            TraitMutation::FlatIncrease(e) => {
                self.damage
                    .entry(dtype)
                    .and_modify(|d| d.damage += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::FlatDecrease(e) => {
                self.damage
                    .entry(dtype)
                    .and_modify(|d| d.damage += -e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalChance(e) => {
                self.damage
                    .entry(dtype)
                    .and_modify(|d| d.crit_chance += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::CriticalMultiplier(e) => {
                self.damage
                    .entry(dtype)
                    .and_modify(|d| d.critical_multiplier += e)
                    .or_insert(damage.clone());
            }
            TraitMutation::MultiplicativeBonus(e) => {
                self.damage
                    .entry(dtype)
                    .and_modify(|d| d.multiplier += e)
                    .or_insert(damage.clone());
            }
            _ => {}
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, ErisValidEnum, ErisDisplayEmoji,
)]
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

impl CharacterTraits {
    pub fn attribute_mutator(&self, character_attributes: &mut Attributes) {
        match self {
            CharacterTraits::Robust => {
                character_attributes.strength += 1;
                character_attributes.constitution += 1;
            }
            CharacterTraits::Nimble => {
                character_attributes.dexterity += 3;
            }
            CharacterTraits::Genius => {
                character_attributes.intelligence += 3;
            }
            CharacterTraits::Lucky => {}
            CharacterTraits::FolkHero => {
                character_attributes.charisma += 2;
                character_attributes.constitution += 2;
            }
            CharacterTraits::Charismatic => {
                character_attributes.charisma += 3;
            }
            CharacterTraits::Strong => {
                character_attributes.strength += 3;
            }
            CharacterTraits::Hermit => {
                character_attributes.wisdom += 3;
            }
            CharacterTraits::Addict => {
                character_attributes.constitution += 3;
                character_attributes.intelligence += 5;
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
                    trait_mutations.set_armor(TraitMutation::FlatIncrease(10));
                    trait_mutations
                        .set_suppress(ResistCategories::Physical, TraitMutation::FlatIncrease(30));
                }
                CharacterTraits::Nimble => {
                    let traits = vec![
                        TraitMutation::FlatIncrease(30),
                        TraitMutation::MultiplicativeBonus(1.2),
                    ];
                    traits
                        .iter()
                        .for_each(|f| trait_mutations.set_dodge(f.clone()));
                }
                CharacterTraits::Genius => {
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::FlatIncrease(20),
                    );
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::CriticalChance(0.1),
                    );
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::CriticalMultiplier(0.5),
                    );
                }
                CharacterTraits::Lucky => {
                    trait_mutations.set_dodge(TraitMutation::FlatIncrease(20));
                    trait_mutations.set_suppress(
                        ResistCategories::Prismatic,
                        TraitMutation::MultiplicativeBonus(1.2),
                    );
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::CriticalChance(0.2))
                }
                CharacterTraits::FolkHero => {
                    let mutations = vec![
                        TraitMutation::FlatIncrease(20),
                        TraitMutation::CriticalChance(0.1),
                        TraitMutation::CriticalMultiplier(0.5),
                    ];
                    let alignments = vec![
                        Alignment::LawfulEvil,
                        Alignment::LawfulGood,
                        Alignment::LawfulNeutral,
                    ];
                    for m in mutations {
                        for a in &alignments {
                            trait_mutations.set_alignment_damage(*a, m.clone());
                        }
                    }
                }
                CharacterTraits::Charismatic => {
                    trait_mutations
                        .set_stat_damage("charisma".to_owned(), TraitMutation::FlatIncrease(20));
                    trait_mutations
                        .set_stat_damage("charisma".to_owned(), TraitMutation::CriticalChance(0.1));
                    trait_mutations.set_stat_damage(
                        "charisma".to_owned(),
                        TraitMutation::CriticalMultiplier(0.5),
                    );
                }
                CharacterTraits::Strong => {
                    trait_mutations
                        .set_damage(DamageType::Physical, TraitMutation::FlatIncrease(5));
                }

                CharacterTraits::Hermit => {
                    trait_mutations.set_suppress(
                        ResistCategories::Prismatic,
                        TraitMutation::MultiplicativeBonus(1.2),
                    );
                }

                CharacterTraits::Addict => {
                    trait_mutations.set_dodge(TraitMutation::FlatDecrease(20))
                }

                CharacterTraits::Cursed => {
                    trait_mutations
                        .set_suppress(ResistCategories::Universal, TraitMutation::FlatDecrease(20));
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::FlatIncrease(5));
                }
                CharacterTraits::Unlucky => {}
                CharacterTraits::Righteous => {
                    let traits = vec![
                        TraitMutation::FlatIncrease(20),
                        TraitMutation::CriticalChance(0.1),
                        TraitMutation::CriticalMultiplier(0.5),
                    ];
                    let alignments = vec![
                        Alignment::ChaoticEvil,
                        Alignment::ChaoticGood,
                        Alignment::ChaoticNeutral,
                    ];
                    for m in traits {
                        for a in &alignments {
                            trait_mutations.set_alignment_damage(*a, m.clone());
                        }
                    }
                }
                CharacterTraits::Greedy => {
                    trait_mutations.set_suppress(
                        ResistCategories::Prismatic,
                        TraitMutation::MultiplicativeBonus(1.2),
                    );
                    trait_mutations.set_damage(
                        DamageType::Universal,
                        TraitMutation::AlignmentBonus(Alignment::LawfulGood, 100),
                    )
                }
                CharacterTraits::Keen => {
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::CriticalChance(0.25));
                    trait_mutations.set_damage(
                        DamageType::Universal,
                        TraitMutation::CriticalMultiplier(1.5),
                    );
                }
                CharacterTraits::Energetic => {
                    trait_mutations.set_actions(TraitMutation::ActionBonus(1));
                }
            }
        }

        trait_mutations
    }
}
