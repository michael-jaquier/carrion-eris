use crate::dice::Dice;

use crate::units::{Attributes, DamageType};
use eris_macro::{ErisDisplayEmoji, ErisValidEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::constructed::ItemsWeHave;

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Default)]
pub struct Items {
    items: Vec<ItemsWeHave>,
    pub(crate) gold: u64,
}

impl Items {
    pub fn push(&mut self, item: ItemsWeHave) {
        self.items.push(item);
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ItemsWeHave> {
        self.items.iter()
    }

    pub fn slot(&self, slot: EquipmentSlot) -> Self {
        let items = self
            .items
            .iter()
            .filter(|item| item.generate().slot == slot)
            .cloned()
            .collect();
        Self {
            items,
            gold: self.gold,
        }
    }

    pub fn contains(&self, have_item: String) -> Option<ItemsWeHave> {
        for item in &self.items {
            if item.generate().name == have_item {
                return Some(*item);
            }
        }
        None
    }

    pub fn take(&mut self, have_item: String) -> Option<ItemsWeHave> {
        for (index, item) in self.items.iter().enumerate() {
            if item.generate().name == have_item {
                return Some(self.items.remove(index));
            }
        }
        None
    }

    pub fn sell(&mut self, slot: Option<EquipmentSlot>) -> &mut Items {
        if let Some(slot) = slot {
            for item in &self.items {
                if item.generate().slot == slot {
                    self.gold += item.generate().value;
                }
            }
            self.items.retain(|item| item.generate().slot != slot);
        } else {
            for item in &self.items {
                self.gold += item.generate().value;
            }
            self.items.clear();
        }
        self
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
        let mut wonderous_items = EquipmentSlot::WondrousItem.to_string();

        for item in &self.items {
            let slot = item.generate().slot;
            match slot {
                EquipmentSlot::Helmet => {
                    helms.push_str("\n\t\t⮁\t");
                    helms.push_str(&item.generate().name);
                }
                EquipmentSlot::Armor => {
                    armors.push_str("\n\t\t⮁\t");
                    armors.push_str(&item.generate().name);
                }
                EquipmentSlot::Legs => {
                    legs.push_str("\n\t\t⮁\t");
                    legs.push_str(&item.generate().name);
                }
                EquipmentSlot::Feet => {
                    feet.push_str("\n\t\t⮁\t");
                    feet.push_str(&item.generate().name);
                }
                EquipmentSlot::Hands => {
                    hands.push_str("\n\t\t⮁\t");
                    hands.push_str(&item.generate().name);
                }
                EquipmentSlot::Weapon => {
                    weapons.push_str("\n\t\t⮁\t");
                    weapons.push_str(&item.generate().name);
                }
                EquipmentSlot::Shield => {
                    shields.push_str("\n\t\t⮁\t");
                    shields.push_str(&item.generate().name);
                }
                EquipmentSlot::Ring => {
                    rings.push_str("\n\t\t⮁\t");
                    rings.push_str(&item.generate().name);
                }
                EquipmentSlot::Amulet => {
                    amulets.push_str("\n\t\t⮁\t");
                    amulets.push_str(&item.generate().name);
                }
                EquipmentSlot::WondrousItem => {
                    wonderous_items.push_str("\n\t\t⮁\t");
                    wonderous_items.push_str(&item.generate().name);
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
        if wonderous_items.len() > EquipmentSlot::WondrousItem.to_string().len() {
            string.push('\n');
            string.push_str(&wonderous_items);
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
    item: Option<ItemsWeHave>,
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

    pub fn item(&self) -> Option<&ItemsWeHave> {
        self.item.as_ref()
    }

    pub fn auto_equip(&mut self, new_item: ItemsWeHave) -> Option<ItemsWeHave> {
        if self.player_equipped {
            return Some(new_item);
        }
        if let Some(item) = self.item {
            if item.generate().rarity <= new_item.generate().rarity {
                self.item = Some(new_item);
                Some(item)
            } else {
                Some(new_item)
            }
        } else {
            self.item = Some(new_item);
            None
        }
    }

    pub fn equip(&mut self, new_item: ItemsWeHave) -> Option<ItemsWeHave> {
        self.player_equipped = true;
        if let Some(item) = self.item {
            self.item = Some(new_item);
            Some(item)
        } else {
            self.item = Some(new_item);
            None
        }
    }

    pub fn damage(&self) -> HashMap<DamageType, Dice> {
        if let Some(item) = &self.item {
            item.generate().damage
        } else {
            DamageType::damage_type_hash_map()
        }
    }

    pub fn armor(&self) -> Dice {
        if let Some(item) = &self.item {
            item.generate().armor
        } else {
            Dice::zero()
        }
    }

    pub fn action(&self) -> u32 {
        if let Some(item) = &self.item {
            item.generate().action
        } else {
            0
        }
    }

    pub fn attribute(&self) -> Attributes {
        if let Some(item) = &self.item {
            item.generate().attribute_bonus
        } else {
            Attributes::zero()
        }
    }

    pub fn resistance(&self) -> HashMap<DamageType, Dice> {
        if let Some(item) = &self.item {
            item.generate().resistance
        } else {
            DamageType::damage_type_hash_map()
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
    pub fn equip(&mut self, new_item: ItemsWeHave) -> Option<ItemsWeHave> {
        let item = new_item.generate();
        match item.slot {
            EquipmentSlot::Helmet => self.helmet.equip(new_item),
            EquipmentSlot::Armor => self.armor.equip(new_item),
            EquipmentSlot::Legs => self.legs.equip(new_item),
            EquipmentSlot::Feet => self.feet.equip(new_item),
            EquipmentSlot::Hands => self.hands.equip(new_item),
            EquipmentSlot::Weapon => self.weapon.equip(new_item),
            EquipmentSlot::Shield => self.shield.equip(new_item),
            EquipmentSlot::Ring => {
                if let Some((index, _)) = self
                    .ring
                    .iter()
                    .enumerate()
                    .find(|(_index, ring)| !ring.player_equipped)
                {
                    self.ring[index].equip(new_item)
                } else {
                    let old_ring = self.ring[0].equip(new_item);
                    let one = self.ring[0].clone();
                    let two = self.ring[1].clone();
                    // Cannot mem::swap
                    self.ring[0] = two;
                    self.ring[1] = one;
                    old_ring
                }
            }
            EquipmentSlot::Amulet => self.amulet.equip(new_item),
            EquipmentSlot::WondrousItem => {
                if let Some((index, _)) = self
                    .wondrous_item
                    .iter()
                    .enumerate()
                    .find(|(_index, wonder)| !wonder.player_equipped)
                {
                    self.wondrous_item[index].equip(new_item)
                } else {
                    let old_wonder = self.wondrous_item[0].equip(new_item);
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

    pub(crate) fn auto_equip(&mut self, new_item: ItemsWeHave) -> Option<ItemsWeHave> {
        let item = new_item.generate();
        match item.slot {
            EquipmentSlot::Helmet => self.helmet.auto_equip(new_item),
            EquipmentSlot::Armor => self.armor.auto_equip(new_item),
            EquipmentSlot::Legs => self.legs.auto_equip(new_item),
            EquipmentSlot::Feet => self.feet.auto_equip(new_item),
            EquipmentSlot::Hands => self.hands.auto_equip(new_item),
            EquipmentSlot::Weapon => self.weapon.auto_equip(new_item),
            EquipmentSlot::Shield => self.shield.auto_equip(new_item),
            EquipmentSlot::Ring => {
                let lowest_rarity_index = self
                    .ring
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, item)| item.item().map(|x| x.generate().rarity))
                    .unwrap()
                    .0;
                self.ring[lowest_rarity_index].auto_equip(new_item)
            }
            EquipmentSlot::Amulet => self.amulet.auto_equip(new_item),
            EquipmentSlot::WondrousItem => {
                let lowest_rarity_index = self
                    .wondrous_item
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, item)| item.item().map(|x| x.generate().rarity))
                    .unwrap()
                    .0;
                self.wondrous_item[lowest_rarity_index].auto_equip(new_item)
            }
        }
    }

    pub fn dodge() -> Option<Dice> {
        None
    }
    pub fn armor(&self) -> Dice {
        let mut base = Dice::zero();

        base += self.armor.armor();
        base += self.helmet.armor();
        base += self.legs.armor();
        base += self.feet.armor();
        base += self.hands.armor();
        base += self.shield.armor();
        base += self.amulet.armor();
        base += self.weapon.armor();
        base += self.ring.iter().fold(Dice::zero(), |mut acc, ring| {
            acc += ring.armor();
            acc
        });
        base += self
            .wondrous_item
            .iter()
            .fold(Dice::zero(), |mut acc, item| {
                acc += item.armor();
                acc
            });
        base
    }
    pub fn resistance(&self) -> HashMap<DamageType, Dice> {
        let mut resistance = DamageType::damage_type_hash_map();
        self.armor
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.helmet
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.legs
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.feet
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.hands
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.shield
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.amulet
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        self.weapon
            .resistance()
            .iter()
            .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        for ring in &self.ring {
            ring.resistance()
                .iter()
                .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
        }
        for wonder in &self.wondrous_item {
            wonder
                .resistance()
                .iter()
                .for_each(|(k, v)| resistance.get_mut(k).unwrap().add_dice(v.clone()));
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

    pub fn action_points(&self) -> u32 {
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
    pub fn damage(&self) -> HashMap<DamageType, Dice> {
        let mut base = DamageType::damage_type_hash_map();
        self.weapon
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.helmet
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.legs
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.feet
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.hands
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.shield
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        self.amulet
            .damage()
            .iter()
            .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        for ring in &self.ring {
            ring.damage()
                .iter()
                .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
        }
        for wonder in &self.wondrous_item {
            wonder
                .damage()
                .iter()
                .for_each(|(k, v)| base.get_mut(k).unwrap().add_dice(v.clone()));
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
            string.push_str(&helmet.generate().name);
        }

        if let Some(armor) = &self.armor.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Armor: ");
            string.push_str(&armor.generate().name);
        }

        if let Some(legs) = &self.legs.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Legs: ");
            string.push_str(&legs.generate().name);
        }

        if let Some(feet) = &self.feet.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Feet: ");
            string.push_str(&feet.generate().name);
        }

        if let Some(hands) = &self.hands.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Hands: ");
            string.push_str(&hands.generate().name);
        }

        if let Some(weapon) = &self.weapon.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Weapon: ");
            string.push_str(&weapon.generate().name);
        }

        if let Some(shield) = &self.shield.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Shield: ");
            string.push_str(&shield.generate().name);
        }

        if self.ring.iter().any(|ring| ring.item().is_some()) {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Rings: ");
        }
        for x in self.ring.iter() {
            if x.item().is_some() {
                string.push_str("\n\t\t\t💍\t ");
                string.push_str(&x.item().unwrap().generate().name);
            }
        }

        if let Some(amulet) = &self.amulet.item() {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Amulet: ");
            string.push_str(&amulet.generate().name);
        }

        if self.wondrous_item.iter().any(|item| item.item().is_some()) {
            string.push_str("\n\t");
            string.push_str("🎲\t");
            string.push_str("Wondrous Items: ");
        }
        for x in self.wondrous_item.iter() {
            if x.item().is_some() {
                string.push_str("\n\t\t\t\t");
                string.push_str(&x.item().unwrap().generate().name);
            }
        }

        write!(f, "{}", string)
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn rarity_order() {
        use crate::items::Rarity;
        assert!(Rarity::Common > Rarity::Uncommon);
        assert!(Rarity::Uncommon > Rarity::Rare);
        assert!(Rarity::Rare > Rarity::VeryRare);
        assert!(Rarity::VeryRare > Rarity::Epic);
        assert!(Rarity::Epic > Rarity::Legendary);
        assert!(Rarity::Legendary > Rarity::Artifact);
        assert!(Rarity::Artifact > Rarity::Wondrous);
        assert!(Rarity::Wondrous > Rarity::Unique);
    }

    #[test]
    fn wondrous_item_equip() {
        let wonder = crate::constructed::ItemsWeHave::generate_random_item_slot(
            crate::items::EquipmentSlot::WondrousItem,
        );
        let mut equipment = crate::items::Equipment::default();
        equipment.equip(wonder);
        assert_eq!(equipment.wondrous_item[0].item(), Some(&wonder));
    }
}
