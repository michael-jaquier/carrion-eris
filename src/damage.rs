use std::collections::HashMap;

use derive_builder::Builder;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use strum::{EnumIter, IntoEnumIterator};

use crate::unit::Alignment;
use crate::BattleInfo;
use crate::{armor_scaling, character::Character, dodge_scaling, enemy::Enemy, resistance_scaling};
#[derive(Default, Debug)]

pub struct Defense {
    dodge: i32,
    armor: i32,
    suppress: HashMap<ResistCategories, i32>,
}

impl Defense {
    pub fn new_enemy(enemy: &Enemy) -> Self {
        let mut suppress = ResistCategories::resist_category_hash_map();
        suppress.insert(ResistCategories::Universal, enemy.resistance);
        let armor = enemy.defense;
        let dodge = crate::EnemyEvents::grade(&enemy.kind) as i32;
        Self {
            dodge,
            armor,
            suppress,
        }
    }

    pub fn new(character: &Character) -> Self {
        let suppress: HashMap<_, _> = character
            .mutations()
            .get_all_suppress()
            .iter()
            .chain(character.equipment.resistance().iter())
            .map(|(key, value)| (*key, *value))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                *acc.entry(key).or_insert(0) += value;
                acc
            });
        let armor = character.equipment.armor() + character.mutations().get_armor();
        let dodge = character.equipment.dodge() + character.mutations().get_dodge();

        Self {
            dodge,
            armor,
            suppress,
        }
    }

    pub fn dodge(&self) -> bool {
        thread_rng().gen_bool(dodge_scaling(self.dodge).max(85.0) / 100.0)
    }

    pub fn physical_mitigation(&self) -> f64 {
        armor_scaling(self.armor)
    }
    pub fn magical_suppress(&self, resist: ResistCategories) -> f64 {
        let xx = |resist: ResistCategories| -> f64 {
            (resistance_scaling(*self.suppress.get(&resist).unwrap_or(&0))
                + resistance_scaling(
                    *self
                        .suppress
                        .get(&ResistCategories::Universal)
                        .unwrap_or(&0),
                )) as f64
        };

        match resist {
            ResistCategories::Elemental => {
                xx(ResistCategories::Elemental) + xx(ResistCategories::Prismatic)
            }
            ResistCategories::Physical => xx(ResistCategories::Physical),
            ResistCategories::NonElemental => xx(ResistCategories::NonElemental),
            ResistCategories::Boss => xx(ResistCategories::Boss),
            ResistCategories::Prismatic => xx(ResistCategories::Prismatic),
            ResistCategories::Universal => resistance_scaling(
                *self
                    .suppress
                    .get(&ResistCategories::Universal)
                    .unwrap_or(&0),
            ),
        }
    }

    pub fn defense(&self, resist: ResistCategories) -> f64 {
        match resist {
            ResistCategories::Physical => (self.physical_mitigation()).min(99.8),
            _ => self.magical_suppress(resist).min(99.8),
        }
    }
}

impl From<&mut Character> for Defense {
    fn from(character: &mut Character) -> Self {
        Self::new(character)
    }
}

impl From<&Character> for Defense {
    fn from(character: &Character) -> Self {
        Self::new(character)
    }
}

impl From<&Enemy> for Defense {
    fn from(enemy: &Enemy) -> Self {
        Self::new_enemy(enemy)
    }
}

#[cfg(test)]
mod test {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, EnumIter)]
pub enum ResistCategories {
    Elemental,
    Physical,
    NonElemental,
    Boss,
    Prismatic,
    Universal,
}

impl ResistCategories {
    pub fn resist_category_hash_map() -> HashMap<ResistCategories, i32> {
        let mut hash = HashMap::new();
        for dtype in ResistCategories::iter() {
            hash.insert(dtype, 0);
        }
        hash
    }
}

impl Distribution<ResistCategories> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ResistCategories {
        match rng.gen_range(0..=40) {
            0..=20 => ResistCategories::Elemental,
            21..=25 => ResistCategories::Physical,
            26..=39 => ResistCategories::NonElemental,
            _ => ResistCategories::Prismatic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, EnumIter, Default)]
pub enum DamageType {
    Fire,
    Water,
    Earth,
    Air,
    Light,
    Dark,
    Iron,
    Arcane,
    Holy,
    NonElemental,
    Physical,
    Hope,
    Despair,
    Existential,
    Boss,
    Prismatic,
    Healing,
    #[default]
    Universal,
}

impl Distribution<DamageType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DamageType {
        // Select from any damage type except universal
        let damage_types: Vec<DamageType> = DamageType::iter()
            .filter(|x| *x != DamageType::Universal)
            .collect();
        *damage_types.choose(rng).unwrap()
    }
}

impl DamageType {
    pub fn damage_type_hash_map() -> HashMap<DamageType, i32> {
        let mut hash = HashMap::new();
        for dtype in DamageType::iter() {
            hash.insert(dtype, 0);
        }
        hash
    }

    pub fn resist_category(&self) -> ResistCategories {
        match self {
            DamageType::Fire => ResistCategories::Elemental,
            DamageType::Water => ResistCategories::Elemental,
            DamageType::Earth => ResistCategories::Elemental,
            DamageType::Air => ResistCategories::Elemental,
            DamageType::Light => ResistCategories::NonElemental,
            DamageType::Dark => ResistCategories::NonElemental,
            DamageType::Iron => ResistCategories::NonElemental,
            DamageType::Arcane => ResistCategories::NonElemental,
            DamageType::Holy => ResistCategories::NonElemental,
            DamageType::NonElemental => ResistCategories::NonElemental,
            DamageType::Physical => ResistCategories::Physical,
            DamageType::Hope => ResistCategories::NonElemental,
            DamageType::Despair => ResistCategories::NonElemental,
            DamageType::Existential => ResistCategories::NonElemental,
            DamageType::Boss => ResistCategories::Boss,
            DamageType::Prismatic => ResistCategories::Prismatic,
            DamageType::Healing => ResistCategories::NonElemental,
            DamageType::Universal => ResistCategories::Universal,
        }
    }
}

impl From<&str> for DamageType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fire" => DamageType::Fire,
            "water" => DamageType::Water,
            "earth" => DamageType::Earth,
            "air" => DamageType::Air,
            "light" => DamageType::Light,
            "dark" => DamageType::Dark,
            "iron" => DamageType::Iron,
            "arcane" => DamageType::Arcane,
            "holy" => DamageType::Holy,
            "nonelemental" => DamageType::NonElemental,
            "physical" => DamageType::Physical,
            "hope" => DamageType::Hope,
            "despair" => DamageType::Despair,
            "existential" => DamageType::Existential,
            "boss" => DamageType::Boss,
            "prismatic" => DamageType::Prismatic,
            "healing" => DamageType::Healing,
            "universal" => DamageType::Universal,
            _ => panic!("Invalid Damage Type {s:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash, EnumIter)]
pub enum UniqueDamageEffect {
    Poison,
    Bleed,
    Burn,
    Shock,
    Curse,
    Regenerate,
    Invigorate,
    Enrage,
    Berserk,
    Vampire,
    Death,
}

impl UniqueDamageEffect {
    pub fn apply(
        &self,
        player: &Character,
        enemy: &Enemy,
        self_damage: &Damage,
        battle_info: &mut BattleInfo,
    ) -> Damage {
        let mut damage = Damage::zero(self_damage.dtype);
        use UniqueDamageEffect::*;
        match self {
            Poison => {
                damage.damage += enemy.health / 10;
                battle_info.custom_text = Some("Poisoned".to_string());
            }
            Bleed => {
                damage.damage += (enemy.health / 10) * self_damage.number_of_hits as i32;
                battle_info.custom_text = Some("Bleeding".to_string());
            }
            Burn => {
                damage.damage += enemy.health / 10;
                battle_info.custom_text = Some("Burning".to_string());
            }
            Shock => {
                damage.multiplier = 1.5;
                battle_info.custom_text = Some("Shocked".to_string());
            }
            Curse => {
                damage.multiplier = 1.5;
                battle_info.custom_text = Some("Cursed".to_string());
            }
            Regenerate => {
                battle_info.player_healing += (player.max_hp / 3u32) as i32;
                battle_info.player_healing =
                    battle_info.player_healing.min(battle_info.enemy_damage);
                battle_info.custom_text = Some("Regenerating".to_string());
            }
            Invigorate => {
                battle_info.player_healing += (player.max_hp) as i32;
                battle_info.custom_text = Some("Invigorating".to_string());
            }
            Enrage => {
                damage.crit_chance = 0.25;
                damage.critical_multiplier += 1.5;
                battle_info.custom_text = Some("Enraged".to_string());
            }
            Berserk => {
                damage.number_of_hits += 2;
                damage.number_of_hits = (self_damage.number_of_hits as f64 * 1.25) as u32;
                battle_info.custom_text = Some("Berserk".to_string());
            }
            Vampire => {
                battle_info.player_healing += damage.damage() / 4;
                battle_info.custom_text = Some("Vampiric".to_string());
            }
            Death => {
                battle_info.enemy_killed = true;
                battle_info.custom_text = Some("Instant Death".to_string());
            }
        }
        damage
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Builder)]
pub struct Damage {
    #[builder(default = "0")]
    pub damage: i32,
    pub dtype: DamageType,
    #[builder(default = "1.0")]
    pub multiplier: f64,
    #[builder(default = "1.3")]
    pub critical_multiplier: f64,
    #[builder(default = "0.05")]
    pub crit_chance: f64,
    #[builder(default = "1")]
    pub number_of_hits: u32,
    #[builder(setter(strip_option), default)]
    pub alignment: Option<Alignment>,
    #[builder(default)]
    pub unique_effect: Vec<UniqueDamageEffect>,
}

impl Damage {
    pub fn damage(&self) -> i32 {
        let mut damage = 0;

        for _ in 0..self.number_of_hits {
            damage += self.damage;
            if thread_rng().gen_bool(self.crit_chance) {
                damage += (damage as f64 * self.critical_multiplier) as i32;
            }
        }
        damage += (damage as f64 * self.multiplier) as i32;
        damage
    }

    pub fn appy_unique_effects(
        &mut self,
        player: &Character,
        enemy: &Enemy,
        battle_info: &mut BattleInfo,
    ) {
        let mut damage = Damage::zero(self.dtype);
        for effect in self.unique_effect.iter() {
            let mutated_damage = effect.apply(player, enemy, self, battle_info);
            damage += mutated_damage;
        }
        *self += damage;
    }

    pub fn dtype(&self) -> DamageType {
        self.dtype
    }

    pub fn zero(dtype: DamageType) -> Damage {
        DamageBuilder::default()
            .crit_chance(0.0)
            .critical_multiplier(0.0)
            .damage(0)
            .dtype(dtype)
            .multiplier(0.0)
            .number_of_hits(0)
            .build()
            .unwrap()
    }
}

impl AddAssign for Damage {
    fn add_assign(&mut self, rhs: Self) {
        self.damage += rhs.damage / (self.number_of_hits as i32).max(1);
        self.multiplier += rhs.multiplier;
        self.critical_multiplier += rhs.critical_multiplier;
        self.crit_chance += rhs.crit_chance;
        self.number_of_hits += rhs.number_of_hits;
        if self.crit_chance >= 1.0 {
            let diff = self.crit_chance - 1.0;
            self.critical_multiplier += diff * 100.0;
            self.crit_chance = 0.99;
        }
    }
}

impl Add for Damage {
    type Output = Damage;

    fn add(self, rhs: Self) -> Self::Output {
        let mut damage = self;
        damage += rhs;
        damage
    }
}
