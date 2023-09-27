use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum EquipmentSlot {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    Weapon,
    Shield,
    Ring,
    Amulet,
    Consumable,
    Misc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Item {
    name: String,
    description: String,
    value: u64,
    rarity: u64,
    damage: u64,
    defense: u64,
    resistance: u64,
    quantity: u64,
    slot: EquipmentSlot,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Items {
    items: Vec<Item>,
    pub(crate) gold: u64,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            gold: 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Equipment {
    head: Option<Item>,
    chest: Option<Item>,
    legs: Option<Item>,
    feet: Option<Item>,
    hands: Option<Item>,
    weapon: Option<Item>,
    shield: Option<Item>,
    ring: Option<Item>,
    amulet: Option<Item>,
    consumable: Option<Item>,
    misc: Option<Item>,
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            head: None,
            chest: None,
            legs: None,
            feet: None,
            hands: None,
            weapon: None,
            shield: None,
            ring: None,
            amulet: None,
            consumable: None,
            misc: None,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    #[test]
    fn item_hashing() {
        let item = Item {
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
            value: 100,
            rarity: 1,
            damage: 0,
            defense: 0,
            resistance: 0,
            quantity: 1,
            slot: EquipmentSlot::Misc,
        };
        let item2 = Item {
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
            value: 100,
            rarity: 1,
            damage: 0,
            defense: 0,
            resistance: 0,
            quantity: 1,
            slot: EquipmentSlot::Misc,
        };
        assert_eq!(item, item2);
    }

    #[test]
    fn hashing_items() {
        let item = Item {
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
            value: 100,
            rarity: 1,
            damage: 0,
            defense: 0,
            resistance: 0,
            quantity: 1,
            slot: EquipmentSlot::Misc,
        };
        let item2 = Item {
            name: "Test Item".to_string(),
            description: "A test item".to_string(),
            value: 100,
            rarity: 1,
            damage: 0,
            defense: 0,
            resistance: 0,
            quantity: 1,
            slot: EquipmentSlot::Misc,
        };
        let mut map = HashMap::new();
        map.insert(item, 1);
        map.insert(item2, 2);
        assert_eq!(map.len(), 1);
    }
}
