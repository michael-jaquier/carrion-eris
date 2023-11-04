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
    fn new() -> Self {
        Self::default()
    }

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

    pub fn get_damage(&self, dtype: DamageType, stat: String, alignment: Alignment) -> Damage {
        let mut damage = self.get_elemental_damage(dtype);
        damage += self.get_attribute_damage(stat);
        damage += self.get_alignment_damage(alignment);
        damage
    }

    pub fn get_elemental_damage(&self, dtype: DamageType) -> Damage {
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

    fn get_attribute_damage(&self, attribute: String) -> Damage {
        let attribute_damage = self
            .stat_damage
            .get(&attribute)
            .unwrap_or(&Damage::zero(DamageType::Universal))
            .clone();
        attribute_damage
    }
    fn get_alignment_damage(&self, alignment: Alignment) -> Damage {
        let alignment_damage = self
            .alignment_damage
            .get(&alignment)
            .unwrap_or(&Damage::zero(DamageType::Universal))
            .clone();
        alignment_damage
    }

    fn set_alignment_damage(&mut self, alignment: Alignment, tr: TraitMutation) {
        let mut damage = DamageBuilder::default()
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
                damage.damage += e;
            }
            TraitMutation::FlatDecrease(e) => {
                damage.damage += -e;
            }
            TraitMutation::CriticalChance(e) => {
                damage.crit_chance += e;
            }
            TraitMutation::CriticalMultiplier(e) => {
                damage.critical_multiplier += e;
            }
            TraitMutation::MultiplicativeBonus(e) => {
                damage.multiplier += e;
            }

            _ => {}
        }
        self.alignment_damage
            .entry(alignment)
            .and_modify(|d| *d += damage.clone())
            .or_insert(damage.clone());
    }

    fn set_stat_damage(&mut self, stat: String, tr: TraitMutation) {
        let mut damage = DamageBuilder::default()
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
                damage.damage += e;
            }
            TraitMutation::FlatDecrease(e) => {
                damage.damage += -e;
            }
            TraitMutation::CriticalChance(e) => {
                damage.crit_chance += e;
            }
            TraitMutation::CriticalMultiplier(e) => {
                damage.critical_multiplier += e;
            }
            TraitMutation::MultiplicativeBonus(e) => {
                damage.multiplier += e;
            }
            _ => {}
        }
        self.stat_damage
            .entry(stat)
            .and_modify(|d| *d += damage.clone())
            .or_insert(damage.clone());
    }

    fn set_damage(&mut self, dtype: DamageType, tr: TraitMutation) {
        let mut damage = DamageBuilder::default()
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
                damage.damage += e;
            }
            TraitMutation::FlatDecrease(e) => {
                damage.damage += -e;
            }
            TraitMutation::CriticalChance(e) => {
                damage.crit_chance += e;
            }
            TraitMutation::CriticalMultiplier(e) => {
                damage.critical_multiplier += e;
            }
            TraitMutation::MultiplicativeBonus(e) => {
                damage.multiplier += e;
            }
            _ => {}
        }
        self.damage
            .entry(dtype)
            .and_modify(|d| *d += damage.clone())
            .or_insert(damage.clone());
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
        let mut trait_mutations = TraitMutations::new();
        for tr in traits {
            match tr {
                CharacterTraits::Robust => {
                    trait_mutations.set_armor(TraitMutation::MultiplicativeBonus(1.9));
                    trait_mutations.set_armor(TraitMutation::FlatIncrease(1000));
                    trait_mutations
                        .set_suppress(ResistCategories::Physical, TraitMutation::FlatIncrease(300));
                }
                CharacterTraits::Nimble => {
                    let traits = vec![
                        TraitMutation::FlatIncrease(300),
                        TraitMutation::MultiplicativeBonus(1.2),
                    ];
                    traits
                        .iter()
                        .for_each(|f| trait_mutations.set_dodge(f.clone()));
                }
                CharacterTraits::Genius => {
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::FlatIncrease(200),
                    );
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::CriticalChance(0.1),
                    );
                    trait_mutations.set_stat_damage(
                        "intelligence".to_owned(),
                        TraitMutation::CriticalMultiplier(1.5),
                    );
                }
                CharacterTraits::Lucky => {
                    trait_mutations.set_dodge(TraitMutation::FlatIncrease(200));
                    trait_mutations.set_suppress(
                        ResistCategories::Prismatic,
                        TraitMutation::MultiplicativeBonus(1.8),
                    );
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::CriticalChance(1.7))
                }
                CharacterTraits::FolkHero => {
                    let mutations = vec![
                        TraitMutation::FlatIncrease(320),
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
                        .set_stat_damage("charisma".to_owned(), TraitMutation::FlatIncrease(120));
                    trait_mutations
                        .set_stat_damage("charisma".to_owned(), TraitMutation::CriticalChance(0.3));
                    trait_mutations.set_stat_damage(
                        "charisma".to_owned(),
                        TraitMutation::CriticalMultiplier(2.5),
                    );
                }
                CharacterTraits::Strong => {
                    trait_mutations
                        .set_damage(DamageType::Physical, TraitMutation::FlatIncrease(255));
                }

                CharacterTraits::Hermit => {
                    trait_mutations.set_suppress(
                        ResistCategories::Prismatic,
                        TraitMutation::MultiplicativeBonus(2.2),
                    );
                }

                CharacterTraits::Addict => {
                    trait_mutations.set_dodge(TraitMutation::FlatDecrease(200))
                }

                CharacterTraits::Cursed => {
                    trait_mutations.set_suppress(
                        ResistCategories::Universal,
                        TraitMutation::FlatDecrease(2000),
                    );
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::FlatIncrease(250));
                }
                CharacterTraits::Unlucky => {}
                CharacterTraits::Righteous => {
                    let traits = vec![
                        TraitMutation::FlatIncrease(200),
                        TraitMutation::CriticalChance(0.05),
                        TraitMutation::CriticalMultiplier(1.5),
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
                        TraitMutation::MultiplicativeBonus(1.9),
                    );
                    trait_mutations.set_alignment_damage(
                        Alignment::LawfulGood,
                        TraitMutation::FlatIncrease(1000),
                    )
                }
                CharacterTraits::Keen => {
                    trait_mutations
                        .set_damage(DamageType::Universal, TraitMutation::CriticalChance(0.10));
                    trait_mutations.set_damage(
                        DamageType::Universal,
                        TraitMutation::CriticalMultiplier(2.5),
                    );
                }
                CharacterTraits::Energetic => {
                    trait_mutations.set_actions(TraitMutation::ActionBonus(3));
                }
            }
        }

        trait_mutations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trait_mutatations_apply() {
        let mut trait_mutations = TraitMutations::new();
        let traits = vec![
            TraitMutation::FlatIncrease(10),
            TraitMutation::FlatDecrease(5),
        ];
        for t in traits {
            trait_mutations.set_armor(t.clone());
            trait_mutations.set_suppress(ResistCategories::Boss, t.clone());
            trait_mutations.set_damage(DamageType::Universal, t.clone());
            trait_mutations.set_alignment_damage(Alignment::LawfulGood, t.clone());
            trait_mutations.set_stat_damage("strength".to_string(), t.clone());
        }
        assert!(trait_mutations.get_armor() == 5);
        assert!(trait_mutations.get_suppress(ResistCategories::Boss) == 5);
        let damage = trait_mutations.get_damage(
            DamageType::Universal,
            "strength".to_string(),
            Alignment::LawfulGood,
        );
        assert!(damage.damage == 15);
    }
}
