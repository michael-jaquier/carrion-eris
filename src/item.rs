use crate::character::Character;
use crate::unit::Attributes;
use crate::BattleInfo;
use eris_macro::{ErisDisplayEmoji, ErisValidEnum};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::hash::Hash;
use std::ops::{Add, AddAssign};
use tracing::{info, trace};

use crate::damage::{DamageType, ResistCategories};

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, ErisValidEnum, ErisDisplayEmoji,
)]
pub enum EquipmentSlot {
    #[emoji("🎩")]
    Helmet,
    #[emoji("👕")]
    Armor,
    #[emoji("👖")]
    Legs,
    #[emoji("👞")]
    Feet,
    #[emoji("🧤")]
    Hands,
    #[emoji("🗡️")]
    Weapon,
    #[emoji("🛡️")]
    Shield,
    #[emoji("💍")]
    Ring,
    #[emoji("💎")]
    Amulet,
    #[emoji("🎲")]
    WondrousItem,
}

impl Distribution<EquipmentSlot> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EquipmentSlot {
        match rng.gen_range(0..1000) {
            0..=100 => EquipmentSlot::Helmet,
            101..=301 => EquipmentSlot::Armor,
            302..=403 => EquipmentSlot::Legs,
            404..=504 => EquipmentSlot::Feet,
            505..=605 => EquipmentSlot::Hands,
            606..=706 => EquipmentSlot::Weapon,
            707..=807 => EquipmentSlot::Shield,
            808..=938 => EquipmentSlot::Ring,
            939..=980 => EquipmentSlot::Amulet,
            _ => EquipmentSlot::WondrousItem,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Rarity {
    Common = 150,
    Uncommon = 100,
    Rare = 50,
    VeryRare = 15,
    Epic = 7,
    Legendary = 5,
    Artifact = 4,
    Wondrous = 3,
    Unique = 1,
}

impl Rarity {
    pub fn item_points(&self) -> u64 {
        use Rarity::*;
        match self {
            Common => 1000,
            Uncommon => 2000,
            Rare => 10_000,
            VeryRare => 20_000,
            Epic => 40_000,
            Legendary => 180_000,
            Artifact => 1_500_000,
            Wondrous => 10_000_000,
            Unique => 52_000_000,
        }
    }
    // Return one rarity lower than the provided one
    pub fn one_less(&self) -> Self {
        use Rarity::*;
        match self {
            Common => Common,
            Uncommon => Common,
            Rare => Uncommon,
            VeryRare => Rare,
            Epic => VeryRare,
            Legendary => Epic,
            Artifact => Legendary,
            Wondrous => Artifact,
            Unique => Wondrous,
        }
    }
}

impl From<u64> for Rarity {
    fn from(rarity: u64) -> Self {
        match rarity {
            0..=1000 => Rarity::Common,
            1001..=2000 => Rarity::Uncommon,
            2001..=10_000 => Rarity::Rare,
            10_001..=20_000 => Rarity::VeryRare,
            20_001..=40_000 => Rarity::Epic,
            40_001..=180_000 => Rarity::Legendary,
            180_001..=500_000 => Rarity::Artifact,
            500_001..=1_000_000 => Rarity::Wondrous,
            _ => Rarity::Unique,
        }
    }
}

impl From<String> for Rarity {
    fn from(rarity: String) -> Self {
        match rarity.to_lowercase().as_str() {
            "common" => Rarity::Common,
            "uncommon" => Rarity::Uncommon,
            "rare" => Rarity::Rare,
            "veryrare" => Rarity::VeryRare,
            "epic" => Rarity::Epic,
            "legendary" => Rarity::Legendary,
            "artifact" => Rarity::Artifact,
            "wondrous" => Rarity::Wondrous,
            "unique" => Rarity::Unique,
            _ => panic!("Invalid Rarity {}", rarity),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Default)]
pub struct Items {
    items: HashSet<IndividualItem>,
    pub(crate) gold: u64,
}

impl From<&BattleInfo> for Items {
    fn from(battle_info: &BattleInfo) -> Self {
        let mut gold = battle_info.gold_gained;
        let mut items = HashSet::new();
        for item in battle_info.item_gained.clone() {
            match items.insert(item.clone()) {
                true => {
                    gold += item.rarity as u64;
                }
                false => {}
            }
        }
        Self { items, gold }
    }
}

impl AddAssign for Items {
    fn add_assign(&mut self, rhs: Self) {
        self.gold += rhs.gold;
        for item in rhs.items {
            match self.items.insert(item.clone()) {
                true => {
                    self.gold += item.rarity as u64;
                }
                false => {}
            }
        }
    }
}

impl Items {
    pub fn new(items: HashSet<IndividualItem>, gold: u64) -> Self {
        Self { items, gold }
    }

    pub fn remove(&mut self, item: &IndividualItem) -> bool {
        self.items.remove(item)
    }

    pub fn push(&mut self, item: IndividualItem) {
        match { self.items.insert(item.clone()) } {
            true => {
                self.gold += item.rarity as u64;
            }
            false => {}
        }
    }
    pub fn iter(&self) -> Iter<'_, IndividualItem> {
        self.items.iter()
    }

    pub fn slot(&self, slot: EquipmentSlot) -> Self {
        let items = self
            .items
            .iter()
            .filter(|item| item.slot == slot)
            .cloned()
            .collect();
        Self {
            items,
            gold: self.gold,
        }
    }

    pub fn take(&mut self, have_item: String) -> Option<IndividualItem> {
        let mut item_to_return = None;
        let item = self.items.iter().find(|item| item.name == have_item);
        if let Some(item) = item {
            item_to_return = Some(item.clone());
        }

        if let Some(item) = item_to_return.clone() {
            self.items.remove(&item);
        }
        item_to_return
    }

    pub fn sell(&mut self, slot: Option<EquipmentSlot>) -> &mut Items {
        if let Some(slot) = slot {
            for item in &self.items {
                if item.slot == slot {
                    self.gold += item.rarity as u64;
                }
            }
            self.items.retain(|item| item.slot != slot);
        } else {
            for item in &self.items {
                self.gold += item.rarity as u64;
            }
            self.items.clear();
        }
        self
    }

    pub fn sell_with_knowledge(
        &mut self,
        slot: Option<&EquipmentSlot>,
        known_items: Option<&Items>,
    ) {
        let known: Option<HashSet<_>> = known_items.map(|known| {
            self.items
                .drain()
                .filter(|item| !known.items.contains(item))
                .collect()
        });

        if let Some(slot) = slot {
            for item in &self.items {
                if item.slot == *slot {
                    self.gold += item.rarity as u64;
                }
            }
            self.items.retain(|item| item.slot != *slot);
        } else {
            for item in &self.items {
                self.gold += item.rarity as u64;
            }
            self.items.clear();
        }

        if let Some(known) = known {
            known.iter().for_each(|item| {
                self.items.insert(item.clone());
            });
        }
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("\n```\n");
        let mut helms = EquipmentSlot::Helmet.to_string();
        let mut armors = EquipmentSlot::Armor.to_string();
        let mut legs = EquipmentSlot::Legs.to_string();
        let mut feet = EquipmentSlot::Feet.to_string();
        let mut hands = EquipmentSlot::Hands.to_string();
        let mut amulets = EquipmentSlot::Amulet.to_string();
        let mut rings = EquipmentSlot::Ring.to_string();
        let mut weapons = EquipmentSlot::Weapon.to_string();
        let mut shields = EquipmentSlot::Shield.to_string();
        let mut wondrous_items = EquipmentSlot::WondrousItem.to_string();

        for item in &self.items {
            let slot = &item.slot;
            match slot {
                EquipmentSlot::Helmet => {
                    helms.push_str("\n\t\t⮁\t");
                    helms.push_str(&item.name);
                }
                EquipmentSlot::Armor => {
                    armors.push_str("\n\t\t⮁\t");
                    armors.push_str(&item.name);
                }
                EquipmentSlot::Legs => {
                    legs.push_str("\n\t\t⮁\t");
                    legs.push_str(&item.name);
                }
                EquipmentSlot::Feet => {
                    feet.push_str("\n\t\t⮁\t");
                    feet.push_str(&item.name);
                }
                EquipmentSlot::Hands => {
                    hands.push_str("\n\t\t⮁\t");
                    hands.push_str(&item.name);
                }
                EquipmentSlot::Weapon => {
                    weapons.push_str("\n\t\t⮁\t");
                    weapons.push_str(&item.name);
                }
                EquipmentSlot::Shield => {
                    shields.push_str("\n\t\t⮁\t");
                    shields.push_str(&item.name);
                }
                EquipmentSlot::Ring => {
                    rings.push_str("\n\t\t⮁\t");
                    rings.push_str(&item.name);
                }
                EquipmentSlot::Amulet => {
                    amulets.push_str("\n\t\t⮁\t");
                    amulets.push_str(&item.name);
                }
                EquipmentSlot::WondrousItem => {
                    wondrous_items.push_str("\n\t\t⮁\t");
                    wondrous_items.push_str(&item.name);
                }
            }
        }

        if helms.len() > EquipmentSlot::Helmet.to_string().len() {
            string.push('\n');
            string.push_str(&helms);
        }
        if armors.len() > EquipmentSlot::Armor.to_string().len() {
            string.push('\n');
            string.push_str(&armors);
        }
        if legs.len() > EquipmentSlot::Legs.to_string().len() {
            string.push('\n');
            string.push_str(&legs);
        }
        if feet.len() > EquipmentSlot::Feet.to_string().len() {
            string.push('\n');
            string.push_str(&feet);
        }
        if hands.len() > EquipmentSlot::Hands.to_string().len() {
            string.push('\n');
            string.push_str(&hands);
        }
        if weapons.len() > EquipmentSlot::Weapon.to_string().len() {
            string.push('\n');
            string.push_str(&weapons);
        }
        if shields.len() > EquipmentSlot::Shield.to_string().len() {
            string.push('\n');
            string.push_str(&shields);
        }
        if rings.len() > EquipmentSlot::Ring.to_string().len() {
            string.push('\n');
            string.push_str(&rings);
        }
        if amulets.len() > EquipmentSlot::Amulet.to_string().len() {
            string.push('\n');
            string.push_str(&amulets);
        }
        if wondrous_items.len() > EquipmentSlot::WondrousItem.to_string().len() {
            string.push('\n');
            string.push_str(&wondrous_items);
        }

        string.push('\n');
        string.push_str("💰\t");
        string.push_str("Gold: ");
        string.push_str(&self.gold.to_string());
        string.push_str("\n```\n");
        write!(f, "{}", string)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct NameMe {
    item: Option<IndividualItem>,
    slot: EquipmentSlot,
    player_equipped: bool,
}

impl NameMe {
    pub fn new(slot: EquipmentSlot) -> Self {
        Self {
            item: None,
            slot,
            player_equipped: false,
        }
    }

    pub fn item(&self) -> Option<&IndividualItem> {
        self.item.as_ref()
    }

    pub fn auto_equip(&mut self, new_item: IndividualItem) -> Option<IndividualItem> {
        if self.player_equipped {
            return Some(new_item);
        }

        if self.item.is_some() {
            // Replace current item rarity goes lower as it goes up
            // Use rarity here since it has a higher ceiling
            if self.item.as_ref().unwrap().rarity >= new_item.rarity {
                let i = self.item.clone();
                self.item = Some(new_item);
                i
            } else {
                Some(new_item)
            }
        } else {
            self.item = Some(new_item);
            None
        }
    }

    pub fn equip(&mut self, new_item: IndividualItem) -> Option<IndividualItem> {
        self.player_equipped = true;

        if self.item.is_some() {
            let i = self.item.clone();
            self.item = Some(new_item);
            i
        } else {
            self.item = Some(new_item);
            None
        }
    }

    pub fn damage(&self) -> HashMap<DamageType, i32> {
        if let Some(item) = &self.item {
            item.damage.clone()
        } else {
            DamageType::damage_type_hash_map()
        }
    }

    pub fn armor(&self) -> i32 {
        if let Some(item) = &self.item {
            item.armor
        } else {
            0
        }
    }

    pub fn dodge(&self) -> i32 {
        if let Some(item) = &self.item {
            item.dodge
        } else {
            0
        }
    }

    pub fn action(&self) -> i32 {
        if let Some(item) = &self.item {
            item.action
        } else {
            0
        }
    }

    pub fn attribute(&self) -> Attributes {
        if let Some(item) = &self.item {
            item.attribute_bonus.clone()
        } else {
            Attributes::zero()
        }
    }

    pub fn resistance(&self) -> HashMap<ResistCategories, i32> {
        if let Some(item) = &self.item {
            item.resistance.clone()
        } else {
            ResistCategories::resist_category_hash_map()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Equipment {
    helmet: NameMe,
    armor: NameMe,
    legs: NameMe,
    feet: NameMe,
    hands: NameMe,
    weapon: NameMe,
    shield: NameMe,
    ring: Vec<NameMe>,
    amulet: NameMe,
    wondrous_item: Vec<NameMe>,
}

impl Equipment {
    pub fn boost(&mut self, items: Items, character: Character) -> HashSet<IndividualItem> {
        let items_to_return: HashSet<IndividualItem> = items
            .iter()
            .filter(|item| item.rarity <= Rarity::Artifact)
            .cloned()
            .collect();
        for item in items.items {
            match item.slot {
                EquipmentSlot::Helmet => {
                    if let Some(helmet) = self.helmet.item.as_mut() {
                        helmet.boost(item, &character);
                    }
                }
                EquipmentSlot::Armor => {
                    if let Some(armor) = self.armor.item.as_mut() {
                        armor.boost(item, &character);
                    }
                }
                EquipmentSlot::Legs => {
                    if let Some(legs) = self.legs.item.as_mut() {
                        legs.boost(item, &character);
                    }
                }
                EquipmentSlot::Feet => {
                    if let Some(feet) = self.feet.item.as_mut() {
                        feet.boost(item, &character);
                    }
                }
                EquipmentSlot::Hands => {
                    if let Some(hands) = self.hands.item.as_mut() {
                        hands.boost(item, &character);
                    }
                }
                EquipmentSlot::Weapon => {
                    if let Some(weapon) = self.weapon.item.as_mut() {
                        weapon.boost(item, &character);
                    }
                }
                EquipmentSlot::Shield => {
                    if let Some(shield) = self.shield.item.as_mut() {
                        shield.boost(item, &character);
                    }
                }
                EquipmentSlot::Ring => {
                    for ring in &mut self.ring {
                        if let Some(ring) = ring.item.as_mut() {
                            ring.boost(item.clone(), &character);
                        }
                    }
                }
                EquipmentSlot::Amulet => {
                    if let Some(amulet) = self.amulet.item.as_mut() {
                        amulet.boost(item, &character);
                    }
                }
                EquipmentSlot::WondrousItem => {
                    for wonder in &mut self.wondrous_item {
                        if let Some(wonder) = wonder.item.as_mut() {
                            wonder.boost(item.clone(), &character);
                        }
                    }
                }
            }
        }
        items_to_return
    }

    pub fn equip(&mut self, new_item: IndividualItem) -> Option<IndividualItem> {
        let item = new_item.clone();
        match &item.slot {
            EquipmentSlot::Helmet => self.helmet.equip(new_item.clone()),
            EquipmentSlot::Armor => self.armor.equip(new_item.clone()),
            EquipmentSlot::Legs => self.legs.equip(new_item.clone()),
            EquipmentSlot::Feet => self.feet.equip(new_item.clone()),
            EquipmentSlot::Hands => self.hands.equip(new_item.clone()),
            EquipmentSlot::Weapon => self.weapon.equip(new_item.clone()),
            EquipmentSlot::Shield => self.shield.equip(new_item.clone()),
            EquipmentSlot::Ring => {
                if let Some((index, _)) = self
                    .ring
                    .iter()
                    .enumerate()
                    .find(|(_index, ring)| !ring.player_equipped)
                {
                    self.ring[index].equip(new_item.clone())
                } else {
                    let old_ring = self.ring[0].equip(new_item.clone());
                    let one = self.ring[0].clone();
                    let two = self.ring[1].clone();
                    // Cannot mem::swap
                    self.ring[0] = two;
                    self.ring[1] = one;
                    old_ring
                }
            }
            EquipmentSlot::Amulet => self.amulet.equip(new_item.clone()),
            EquipmentSlot::WondrousItem => {
                if let Some((index, _)) = self
                    .wondrous_item
                    .iter()
                    .enumerate()
                    .find(|(_index, wonder)| !wonder.player_equipped)
                {
                    self.wondrous_item[index].equip(new_item.clone())
                } else {
                    let old_wonder = self.wondrous_item[0].equip(new_item.clone());
                    let one = self.wondrous_item[0].clone();
                    let two = self.wondrous_item[1].clone();
                    let three = self.wondrous_item[2].clone();
                    // Cannot mem::swap
                    self.wondrous_item[0] = two;
                    self.wondrous_item[1] = three;
                    self.wondrous_item[2] = one;
                    old_wonder
                }
            }
        }
    }

    pub(crate) fn auto_equip(&mut self, new_item: IndividualItem) -> Option<IndividualItem> {
        let item = new_item.clone();
        match item.slot {
            EquipmentSlot::Helmet => self.helmet.auto_equip(new_item),
            EquipmentSlot::Armor => self.armor.auto_equip(new_item),
            EquipmentSlot::Legs => self.legs.auto_equip(new_item),
            EquipmentSlot::Feet => self.feet.auto_equip(new_item),
            EquipmentSlot::Hands => self.hands.auto_equip(new_item),
            EquipmentSlot::Weapon => self.weapon.auto_equip(new_item),
            EquipmentSlot::Shield => self.shield.auto_equip(new_item),
            EquipmentSlot::Ring => {
                let lowest_max_point_item = self
                    .ring
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, item)| item.item().map(|x| x.rarity.item_points()))
                    .unwrap()
                    .0;
                self.ring[lowest_max_point_item].auto_equip(new_item)
            }
            EquipmentSlot::Amulet => self.amulet.auto_equip(new_item),
            EquipmentSlot::WondrousItem => {
                let lowest_max_points_item = self
                    .wondrous_item
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, item)| item.item().map(|x| x.rarity.item_points()))
                    .unwrap()
                    .0;
                self.wondrous_item[lowest_max_points_item].auto_equip(new_item)
            }
        }
    }

    pub fn dodge(&self) -> i32 {
        let mut base = 0;
        base += self.armor.dodge();
        base += self.helmet.dodge();
        base += self.legs.dodge();
        base += self.feet.dodge();
        base += self.hands.dodge();
        base += self.shield.dodge();
        base += self.amulet.dodge();
        base += self.weapon.dodge();
        base += self.ring.iter().fold(0, |mut acc, ring| {
            acc += ring.dodge();
            acc
        });
        base += self.wondrous_item.iter().fold(0, |mut acc, item| {
            acc += item.dodge();
            acc
        });
        base
    }
    pub fn armor(&self) -> i32 {
        let mut base = 0;

        base += self.armor.armor();
        base += self.helmet.armor();
        base += self.legs.armor();
        base += self.feet.armor();
        base += self.hands.armor();
        base += self.shield.armor();
        base += self.amulet.armor();
        base += self.weapon.armor();
        base += self.ring.iter().fold(0, |mut acc, ring| {
            acc += ring.armor();
            acc
        });
        base += self.wondrous_item.iter().fold(0, |mut acc, item| {
            acc += item.armor();
            acc
        });
        base
    }
    pub fn resistance(&self) -> HashMap<ResistCategories, i32> {
        let mut resistance = ResistCategories::resist_category_hash_map();
        self.armor
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.helmet
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.legs
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.feet
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.hands
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.shield
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.amulet
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        self.weapon
            .resistance()
            .iter()
            .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        for ring in &self.ring {
            ring.resistance()
                .iter()
                .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        }
        for wonder in &self.wondrous_item {
            wonder
                .resistance()
                .iter()
                .for_each(|(k, v)| *resistance.get_mut(k).unwrap() += v);
        }
        resistance
    }

    pub fn attribute(&self) -> Attributes {
        let mut base = Attributes::zero();
        base += self.armor.attribute();
        base += self.helmet.attribute();
        base += self.legs.attribute();
        base += self.feet.attribute();
        base += self.hands.attribute();
        base += self.shield.attribute();
        base += self.amulet.attribute();
        base += self.weapon.attribute();
        base += self.ring.iter().fold(Attributes::zero(), |mut acc, ring| {
            acc += ring.attribute();
            acc
        });
        base += self
            .wondrous_item
            .iter()
            .fold(Attributes::zero(), |mut acc, item| {
                acc += item.attribute();
                acc
            });

        base
    }

    pub fn action_points(&self) -> i32 {
        let mut base = 0;
        base += self.armor.action();
        base += self.helmet.action();
        base += self.legs.action();
        base += self.feet.action();
        base += self.hands.action();
        base += self.shield.action();
        base += self.amulet.action();
        base += self.weapon.action();
        base += self.ring.iter().fold(0, |mut acc, ring| {
            acc += ring.action();
            acc
        });
        base += self.wondrous_item.iter().fold(0, |mut acc, item| {
            acc += item.action();
            acc
        });

        base
    }
    pub fn damage(&self) -> HashMap<DamageType, i32> {
        let mut base = DamageType::damage_type_hash_map();
        self.weapon
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.helmet
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.legs
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.feet
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.hands
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.shield
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        self.amulet
            .damage()
            .iter()
            .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        for ring in &self.ring {
            ring.damage()
                .iter()
                .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        }
        for wonder in &self.wondrous_item {
            wonder
                .damage()
                .iter()
                .for_each(|(k, v)| *base.get_mut(k).unwrap() += v);
        }

        base
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            helmet: NameMe::new(EquipmentSlot::Helmet),
            armor: NameMe::new(EquipmentSlot::Armor),
            legs: NameMe::new(EquipmentSlot::Legs),
            feet: NameMe::new(EquipmentSlot::Feet),
            hands: NameMe::new(EquipmentSlot::Hands),
            weapon: NameMe::new(EquipmentSlot::Weapon),
            shield: NameMe::new(EquipmentSlot::Shield),
            ring: vec![NameMe::new(EquipmentSlot::Ring); 2],
            amulet: NameMe::new(EquipmentSlot::Amulet),
            wondrous_item: vec![NameMe::new(EquipmentSlot::WondrousItem); 3],
        }
    }
}

impl Display for Equipment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        if let Some(helmet) = &self.helmet.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Helmet: ");
            string.push_str(&helmet.name);
        }

        if let Some(armor) = &self.armor.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Armor: ");
            string.push_str(&armor.name);
        }

        if let Some(legs) = &self.legs.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Legs: ");
            string.push_str(&legs.name);
        }

        if let Some(feet) = &self.feet.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Feet: ");
            string.push_str(&feet.name);
        }

        if let Some(hands) = &self.hands.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Hands: ");
            string.push_str(&hands.name);
        }

        if let Some(weapon) = &self.weapon.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Weapon: ");
            string.push_str(&weapon.name);
        }

        if let Some(shield) = &self.shield.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Shield: ");
            string.push_str(&shield.name);
        }

        if self.ring.iter().any(|ring| ring.item().is_some()) {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Rings: ");
        }
        for x in self.ring.iter() {
            if x.item().is_some() {
                string.push_str("\n\t\t\t💍\t ");
                string.push_str(&x.item().unwrap().name);
            }
        }

        if let Some(amulet) = &self.amulet.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Amulet: ");
            string.push_str(&amulet.name);
        }

        if self.wondrous_item.iter().any(|item| item.item().is_some()) {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Wondrous Items: ");
        }
        for x in self.wondrous_item.iter() {
            if x.item().is_some() {
                string.push_str("\n\t\t\t\t");
                string.push_str(&x.item().unwrap().name);
            }
        }

        write!(f, "{}", string)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct IndividualItem {
    pub name: String,
    pub description: String,
    pub slot: EquipmentSlot,
    pub armor: i32,
    pub dodge: i32,
    pub resistance: HashMap<ResistCategories, i32>,
    pub damage: HashMap<DamageType, i32>,
    pub attribute_bonus: Attributes,
    pub rarity: Rarity,
    pub action: i32,
    pub points: u64,
}

impl AddAssign for IndividualItem {
    fn add_assign(&mut self, rhs: Self) {
        self.armor += rhs.armor;
        self.dodge += rhs.dodge;
        self.action += rhs.action;
        self.points += rhs.points;
        self.attribute_bonus += rhs.attribute_bonus;
        self.damage
            .iter_mut()
            .for_each(|(k, v)| *v += rhs.damage.get(k).unwrap_or(&0));
        self.resistance
            .iter_mut()
            .for_each(|(k, v)| *v += rhs.resistance.get(k).unwrap_or(&0));
    }
}

impl Add for IndividualItem {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl IndividualItem {
    pub fn to_file(&self, path: String) {
        fs::write(path, serde_yaml::to_string(self).unwrap()).unwrap();
    }

    pub fn update_name(&mut self) {
        let plus = IndividualItem::item_boosted(&self.name);
        let rarity = self.rarity;
        let mut stat = String::new();
        if let Some(attribute) = self.attribute_bonus.max_stat() {
            stat += format!("of {attribute} ").as_str();
        }
        let action_string = if self.action > 0 { "Unrelenting " } else { "" };

        let mut name = String::new();
        name += action_string;
        name += format!("{rarity:?} ").as_str();
        name += random_word::gen(random_word::Lang::En);
        name += format!(" {:?} ", self.slot).as_str();
        name += stat.as_str();
        name += format!(" [+{plus}]").as_str();
        self.name = name;
    }

    fn item_boosted(item_name: &str) -> u32 {
        let re = regex::Regex::new(r"(\[\+\d+\])$").unwrap();

        if let Some(caps) = re.captures(item_name) {
            // If the name contains a "[+N]" suffix, increment N and replace it
            let current_value = caps.get(1).unwrap().as_str();
            current_value
                .trim_start_matches("[+")
                .trim_end_matches(']')
                .parse::<u32>()
                .unwrap()
                + 1
        } else {
            1
        }
    }

    pub fn boost(&mut self, sacrifice: IndividualItem, character: &Character) {
        if self.slot != sacrifice.slot {
            info!("Cannot boost item of different slot");
            return;
        }
        if sacrifice.rarity <= Rarity::Artifact {
            trace!("Cannot boost item of rarity Artifact or higher");
            return;
        }
        let max_points = self.rarity.item_points();
        let mut scale = 0.1;

        if self.points >= max_points {
            trace!("Item points exceeded. Cannot add item scaling points down a lot. {self:?}");
            scale *= 0.1;
        }

        let sacrifice_points = (sacrifice.points as f64 * scale) as u64;
        let attribute_scaling = character.current_skill.skill().attribute_scaling().expect(
            "Cannot boost item without attribute scaling. This is a bug. Please report it.",
        );
        let element_scaling =
            character.current_skill.skill().element().expect(
                "Cannot boost item without element scaling. This is a bug. Please report it.",
            );

        let new_sacrifice: IndividualItem =
            (element_scaling, attribute_scaling, sacrifice_points).into();
        *self += new_sacrifice;
        self.update_name();
    }
}

impl Hash for IndividualItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the relevant fields of the struct
        self.name.hash(state);
        self.description.hash(state);
        self.slot.hash(state);
        self.armor.hash(state);
        self.dodge.hash(state);

        // Hash the resistance and damage HashMaps
        for (category, value) in &self.resistance {
            category.hash(state);
            value.hash(state);
        }
        for (damage_type, value) in &self.damage {
            damage_type.hash(state);
            value.hash(state);
        }

        // Hash the attribute_bonus, rarity, and action fields
        self.attribute_bonus.hash(state);
        self.rarity.hash(state);
        self.action.hash(state);
    }
}

#[cfg(test)]
mod test {

    use crate::{
        character::Character,
        damage::{DamageType, ResistCategories},
        item::{EquipmentSlot, IndividualItem, Rarity},
        unit::Attributes,
    };

    #[test]
    fn adding_items_works() {
        let mut item1 = IndividualItem {
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            slot: EquipmentSlot::Helmet,
            armor: 1,
            dodge: 1,
            resistance: ResistCategories::resist_category_hash_map(),
            damage: DamageType::damage_type_hash_map(),
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
            action: 1,
            points: 1,
        };

        item1.damage.insert(DamageType::Fire, 3);
        let mut item2 = IndividualItem {
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            slot: EquipmentSlot::Helmet,
            armor: 1,
            dodge: 1,
            resistance: ResistCategories::resist_category_hash_map(),
            damage: DamageType::damage_type_hash_map(),
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
            action: 1,
            points: 1,
        };
        item2.damage.insert(DamageType::Fire, 3);
        let mut item3 = IndividualItem {
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            slot: EquipmentSlot::Helmet,
            armor: 1,
            dodge: 1,
            resistance: ResistCategories::resist_category_hash_map(),
            damage: DamageType::damage_type_hash_map(),
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
            action: 1,
            points: 1,
        };
        item3.damage.insert(DamageType::Fire, 3);
        item3 += item1;
        item3 += item2;
        assert_eq!(item3.armor, 3);
        assert_eq!(item3.dodge, 3);
        assert_eq!(item3.action, 3);
        assert_eq!(item3.points, 3);
        assert_eq!(item3.damage.get(&DamageType::Fire).unwrap(), &9);
    }

    #[test]
    fn item_boost_mutates_the_item() {
        let mut item = IndividualItem {
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            slot: EquipmentSlot::Helmet,
            armor: 1,
            dodge: 1,
            resistance: ResistCategories::resist_category_hash_map(),
            damage: DamageType::damage_type_hash_map(),
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
            action: 1,
            points: 1,
        };
        let sacrifice = IndividualItem {
            name: "Test Item".to_string(),
            description: "Test Description".to_string(),
            slot: EquipmentSlot::Helmet,
            armor: 1,
            dodge: 1,
            resistance: ResistCategories::resist_category_hash_map(),
            damage: DamageType::damage_type_hash_map(),
            attribute_bonus: Attributes::zero(),
            rarity: Rarity::Common,
            action: 1,
            points: 200000,
        };
        let character: Character = Default::default();

        let old_item = item.clone();
        item.boost(sacrifice, &character);
        item.name = old_item.name.clone();
        println!("Boosted {:?}", item);
        println!("\n");
        println!("Original {:?}", old_item);
        println!("\n");
        assert_ne!(item, old_item)
    }
}
