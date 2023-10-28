use std::collections::HashMap;

use crate::damage::ResistCategories;
use crate::{
    damage::DamageType,
    enemy::Enemy,
    item::{EquipmentSlot, IndividualItem, Rarity},
    unit::Attributes,
    EnemyEvents,
};

use rand::{prelude::Distribution, random, seq::SliceRandom, thread_rng, Rng};
use random_word::Lang;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter)]
enum Choices {
    Armor,
    Dodge,
    Resistance,
    Damage,
}

impl Distribution<Choices> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choices {
        match rng.gen_range(0..40) {
            0..=9 => Choices::Armor,
            10..=12 => Choices::Dodge,
            13..=25 => Choices::Resistance,
            26..=40 => Choices::Damage,
            _ => panic!("Invalid choice"),
        }
    }
}

#[derive(EnumIter)]
enum AttributeChoices {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl From<String> for AttributeChoices {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "strength" => Self::Strength,
            "dexterity" => Self::Dexterity,
            "constitution" => Self::Constitution,
            "intelligence" => Self::Intelligence,
            "wisdom" => Self::Wisdom,
            "charisma" => Self::Charisma,
            _ => panic!("Invalid attribute"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneratedItem {
    points: u64,
    dodge: u64,
    armor: u64,
    resistance: HashMap<ResistCategories, i32>,
    damage: HashMap<DamageType, i32>,
    action: u64,
    attribute_bonus: Attributes,
    rarity: Rarity,
}

impl GeneratedItem {
    fn rarity(&self) -> Rarity {
        if self.points <= 100 {
            return Rarity::Common;
        }
        if self.points <= 300 {
            return Rarity::Uncommon;
        }
        if self.points <= 1500 {
            return Rarity::Rare;
        }
        if self.points <= 5000 {
            return Rarity::Epic;
        }
        Rarity::Legendary
    }

    fn distribute_points(&mut self) {
        self.rarity = self.rarity();
        let mut rng = thread_rng();
        let mut points = rng.gen_range(self.points / 2..self.points * 2);
        self.points = points;
        let mut action_rolls = self.points / 750;
        while action_rolls > 0 {
            let action = rng.gen_bool(0.015) as u64;
            self.action += action;
            action_rolls -= 1;
        }

        points = points.saturating_sub(self.action * 500);

        let mut attribute_points_to_give = self.points % 100;
        while attribute_points_to_give > 0 {
            let chance_for_attribute = rng.gen_bool(0.05);
            if chance_for_attribute {
                match AttributeChoices::iter()
                    .collect::<Vec<_>>()
                    .choose(&mut rng)
                    .expect("Failed to choose attribute")
                {
                    AttributeChoices::Strength => {
                        self.attribute_bonus.strength += 1;
                    }
                    AttributeChoices::Dexterity => {
                        self.attribute_bonus.dexterity += 1;
                    }
                    AttributeChoices::Constitution => {
                        self.attribute_bonus.constitution += 1;
                    }
                    AttributeChoices::Intelligence => {
                        self.attribute_bonus.intelligence += 1;
                    }
                    AttributeChoices::Wisdom => {
                        self.attribute_bonus.wisdom += 1;
                    }
                    AttributeChoices::Charisma => {
                        self.attribute_bonus.charisma += 1;
                    }
                }
            }
            attribute_points_to_give -= 1;
        }

        points = points.saturating_sub((self.attribute_bonus.sum() * 100).try_into().unwrap());

        let damage_scaling = DamageType::iter().collect::<Vec<_>>().len();
        let resist_scaling = ResistCategories::iter().collect::<Vec<_>>().len();
        let sum_scaling = (damage_scaling + resist_scaling) * 5;

        while points > 0 {
            let choice: Choices = random();
            match choice {
                Choices::Armor => {
                    let scale = (points / sum_scaling as u64).max(1);
                    let armor = rng.gen_range(0..scale);
                    self.armor += armor;
                    points = points.saturating_sub(armor);
                }
                Choices::Dodge => {
                    let scale = (points / sum_scaling as u64).max(1);
                    let dodge = rng.gen_range(0..scale);
                    self.dodge += dodge;
                    points = points.saturating_sub(dodge);
                }
                Choices::Resistance => {
                    let scale = (points / sum_scaling as u64).max(1) * resist_scaling as u64;
                    let resistance = rng.gen_range(0..scale);
                    let resist: ResistCategories = random();
                    self.resistance.insert(resist, resistance as i32);
                    points = points.saturating_sub(resistance);
                }
                Choices::Damage => {
                    let scale = (points / sum_scaling as u64).max(1) * damage_scaling as u64;
                    let damage = rng.gen_range(0..scale);
                    self.damage.insert(DamageType::Physical, damage as i32);
                    points = points.saturating_sub(damage);
                }
            }
        }
    }

    fn slot(&self) -> EquipmentSlot {
        random()
    }

    pub fn item(self) -> IndividualItem {
        let slot = self.slot();
        let rarity = self.rarity;
        let mut stat = String::new();
        if let Some(attribute) = self.attribute_bonus.max_stat() {
            match AttributeChoices::from(attribute) {
                AttributeChoices::Strength => {
                    stat.push_str("of strength");
                }
                AttributeChoices::Dexterity => {
                    stat.push_str("of dexterity");
                }
                AttributeChoices::Constitution => {
                    stat.push_str("of constitution");
                }
                AttributeChoices::Intelligence => {
                    stat.push_str("of intelligence");
                }
                AttributeChoices::Wisdom => {
                    stat.push_str("of wisdom");
                }
                AttributeChoices::Charisma => {
                    stat.push_str("of charisma");
                }
            }
        }
        let action_string = if self.action > 0 { "Unrelenting " } else { "" };

        let mut name = String::new();
        name += action_string;
        name += format!("{rarity:?} ").as_str();
        name += random_word::gen(Lang::En);
        name += format!(" {slot:?} ").as_str();
        name += stat.as_str();
        IndividualItem {
            name,
            description: String::from(""),
            slot: slot.clone(),
            armor: self.armor as i32,
            dodge: self.dodge as i32,
            resistance: self.resistance,
            damage: self.damage,
            attribute_bonus: self.attribute_bonus,
            action: self.action as i32,
            rarity,
            points: self.points,
        }
    }
}

impl From<u64> for GeneratedItem {
    fn from(points: u64) -> Self {
        let mut base = Self {
            points,
            dodge: 0,
            armor: 0,
            resistance: Default::default(),
            damage: Default::default(),
            action: 0,
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
        };
        base.distribute_points();
        base
    }
}

impl From<Rarity> for GeneratedItem {
    fn from(rarity: Rarity) -> Self {
        let points = match rarity {
            Rarity::Common => 100,
            Rarity::Uncommon => 300,
            Rarity::Rare => 600,
            Rarity::Epic => 1200,
            Rarity::Legendary => 2000,
            Rarity::VeryRare => 1700,
            Rarity::Artifact => 2200,
            Rarity::Wondrous => 2300,
            Rarity::Unique => 3000,
        };
        Self::from(points)
    }
}

impl From<&Enemy> for Vec<IndividualItem> {
    fn from(enemy: &Enemy) -> Self {
        let grade = enemy.kind.grade();
        let level = enemy.level;
        let probabilty_of_drop = (grade as u32) as f64 / 200.0;
        if thread_rng().gen_bool(probabilty_of_drop) {
            let points_range = 0..(grade as u64 * level as u64 * 2);
            let points = thread_rng().gen_range(points_range);
            vec![GeneratedItem::from(points).item()]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {

    use super::GeneratedItem;

    #[test]
    fn item_generation_range_for_rate() {
        let rarity = crate::item::Rarity::Rare;
        for n in 0..10000 {
            let item: GeneratedItem = rarity.into();
            let item = item.item();
            assert!(
                item.points >= 600 / 2,
                "Points is too low {item:?} iteration {n}"
            );
            assert!(
                item.points <= 600 * 2,
                "Points is too high {item:?} iteration {n}"
            );
            assert!(
                item.rarity == rarity,
                "Rarity is not correct {item:?} iteration {n}"
            );

            assert!(
                item.damage.values().all(|&v| v <= 100),
                "Damage is too high {item:?} iteration {n}"
            );
            assert!(
                item.resistance.values().all(|&v| v <= 100),
                "Resistance is too high {item:?} iteration {n}"
            );
            assert!(
                item.armor <= 100,
                "Armor is too high {item:?} iteration {n}"
            );
            assert!(
                item.dodge <= 100,
                "Dodge is too high {item:?} iteration {n}"
            );
            assert!(
                item.action <= 1,
                "Action is too high {item:?} iteration {n}"
            );
            assert!(
                item.attribute_bonus.sum() <= 100,
                "Attribute is too high {item:?} iteration {n}"
            );
        }
    }
}
