use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    name: String,
    description: String,
    value: u32,
    rarity: u32,
    damage: u32,
    defense: u32,
    resistance: u32,
    slot: EquipmentSlot,
}
