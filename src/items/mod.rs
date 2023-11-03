use std::collections::HashMap;

use crate::damage::ResistCategories;

use crate::{
    damage::DamageType,
    enemy::Enemy,
    item::{EquipmentSlot, IndividualItem, Rarity},
    unit::Attributes,
    EnemyEvents,
};

use rand::{prelude::Distribution, random, thread_rng, Rng};
use random_word::Lang;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use tracing::trace;

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

impl Distribution<AttributeChoices> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AttributeChoices {
        match rng.gen_range(0..6) {
            0 => AttributeChoices::Strength,
            1 => AttributeChoices::Dexterity,
            2 => AttributeChoices::Constitution,
            3 => AttributeChoices::Intelligence,
            4 => AttributeChoices::Wisdom,
            5 => AttributeChoices::Charisma,
            _ => panic!("Invalid choice"),
        }
    }
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
    fn new(points: u64) -> Self {
        if points == 0 {
            trace!("Generated item with 0 points")
        }
        let damage = DamageType::damage_type_hash_map();
        let resistance = ResistCategories::resist_category_hash_map();
        Self {
            points: points.max(1),
            dodge: 0,
            armor: 0,
            resistance,
            damage,
            action: 0,
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
        }
    }
    fn rarity(&self) -> Rarity {
        self.points.into()
    }

    fn distribute_points(&mut self, element: Option<DamageType>, attribute: Option<String>) {
        self.rarity = self.rarity();
        let mut rng = thread_rng();
        let mut points = rng.gen_range(self.points / 2..self.points);
        self.points = points;
        let mut action_rolls = self.points / 3750;

        while action_rolls > 0 {
            let action = rng.gen_bool(0.015) as u64;
            self.action += action;
            action_rolls -= 1;
        }

        points = points.saturating_sub(self.action * 3750);

        let mut attribute_points_to_give = self.points / 1000;
        while attribute_points_to_give > 0 {
            let chance_for_attribute = rng.gen_bool(0.05);
            if chance_for_attribute {
                if let Some(ref attr) = attribute {
                    self.attribute_bonus.add(attr, 1);
                } else {
                    let chosen: AttributeChoices = random();
                    match chosen {
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
            }
            attribute_points_to_give -= 1;
        }

        points = points.saturating_sub((self.attribute_bonus.sum() * 1000).try_into().unwrap());

        let damage_scaling = DamageType::iter().collect::<Vec<_>>().len();
        let resist_scaling = ResistCategories::iter().collect::<Vec<_>>().len();
        let sum_scaling = (damage_scaling + resist_scaling) * 5;

        while points > 0 {
            let choice: Choices = random();
            let points_cost: u64;
            match choice {
                Choices::Armor => {
                    let scale = (points / sum_scaling as u64).max(1);
                    let armor = rng.gen_range(0..scale);
                    self.armor += armor.max(1);
                    points_cost = armor;
                }
                Choices::Dodge => {
                    let scale = (points / sum_scaling as u64).max(1);
                    let dodge = rng.gen_range(0..scale);
                    self.dodge += dodge.max(1);
                    points_cost = dodge;
                }
                Choices::Resistance => {
                    let scale = (points / sum_scaling as u64).max(1) * resist_scaling as u64;
                    let resistance = rng.gen_range(0..scale).max(1);
                    let resist: ResistCategories = random();
                    self.resistance.insert(resist, resistance as i32);
                    points_cost = resistance;
                }
                Choices::Damage => {
                    let scale = (points / sum_scaling as u64).max(1) * damage_scaling as u64;
                    let set_element: DamageType;
                    if let Some(ele) = element {
                        set_element = ele;
                    } else {
                        set_element = random();
                    }
                    let damage = rng.gen_range(0..scale).max(1);
                    self.damage.insert(set_element, damage as i32);
                    points_cost = damage;
                }
            }
            points = points.saturating_sub(points_cost * 5);
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
        let mut base = Self::new(points);
        base.distribute_points(None, None);
        base
    }
}

impl From<Rarity> for GeneratedItem {
    fn from(rarity: Rarity) -> Self {
        // GeneratedItem can not be more rare than Legendary
        if rarity < Rarity::Epic {
            let points = Rarity::Epic.item_points();
            Self::from(points)
        } else {
            let points = rarity.item_points();
            Self::from(points)
        }
    }
}

impl From<&Enemy> for Vec<IndividualItem> {
    fn from(enemy: &Enemy) -> Self {
        let grade = enemy.kind.grade();
        let level = enemy.level;
        let probabilty_of_drop = (grade as u32) as f64 / 200.0;
        if thread_rng().gen_bool(probabilty_of_drop) {
            let points_range = (grade as u64)..(grade as u64 * level as u64 * 2);
            let points = thread_rng()
                .gen_range(points_range)
                .min(Rarity::Epic.item_points());
            vec![GeneratedItem::from(points).item()]
        } else {
            vec![]
        }
    }
}

impl From<(DamageType, String, u64)> for GeneratedItem {
    fn from((damage_type, attribute, points): (DamageType, String, u64)) -> Self {
        let mut base = Self::new(points);
        base.distribute_points(Some(damage_type), Some(attribute));
        base
    }
}

impl From<(DamageType, String, u64)> for IndividualItem {
    fn from((damage_type, name, points): (DamageType, String, u64)) -> Self {
        let generated: GeneratedItem = (damage_type, name, points).into();
        generated.item()
    }
}

#[cfg(test)]
mod test {

    use crate::{enemy::Mob, item::IndividualItem};

    use super::GeneratedItem;

    #[test]
    fn item_from_level_30_legendary_enemy() {
        let mut character = crate::character::Character::default();
        character.level = 30;
        let enemy = Mob::Eldragor;
        let enemy = enemy.generate(character.level);
        let item: Vec<IndividualItem> = (&enemy).into();
        for i in item.iter() {
            assert!(i.points >= 600 / 2, "Points is too low {item:?}");
            assert!(i.points <= 600 * 2, "Points is too high {item:?}");
        }
    }
    #[test]
    fn item_from_level_60_legendary_enemy() {
        let mut character = crate::character::Character::default();
        character.level = 60;
        let enemy = Mob::Eldragor;
        let enemy = enemy.generate(character.level);
        let item: Vec<IndividualItem> = (&enemy).into();
        for i in item.iter() {
            assert!(i.points >= 1900 / 2, "Points is too low {item:?}");
            assert!(i.points <= 1900 * 2, "Points is too high {item:?}");
        }
    }

    #[test]
    fn item_generation_range_for_rate() {
        let rarity = crate::item::Rarity::Rare;
        for n in 0..10000 {
            let item: GeneratedItem = rarity.into();
            let item = item.item();
            assert!(
                item.points >= rarity.item_points() / 2,
                "Points is too low {item:?} iteration {n}"
            );
            assert!(
                item.points <= rarity.item_points() * 2,
                "Points is too high {item:?} iteration {n}"
            );
            assert!(
                item.rarity == rarity,
                "Rarity is not correct {item:?} iteration {n}"
            );

            assert!(
                item.damage.values().all(|&v| v <= 2000),
                "Damage is too high {item:?} iteration {n}"
            );
            assert!(
                item.resistance.values().all(|&v| v <= 2000),
                "Resistance is too high {item:?} iteration {n}"
            );
            assert!(
                item.armor <= 1000,
                "Armor is too high {item:?} iteration {n}"
            );
            assert!(
                item.dodge <= 1000,
                "Dodge is too high {item:?} iteration {n}"
            );
            assert!(
                item.action <= 5,
                "Action is too high {item:?} iteration {n}"
            );
            assert!(
                item.attribute_bonus.sum() <= 1000,
                "Attribute is too high {item:?} iteration {n}"
            );
        }
    }
}
