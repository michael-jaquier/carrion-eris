use crate::dice::{Dice, Die};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs;

use crate::units::{Attribute, Attributes, DamageType};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerialDie {
    pub(crate) die: Die,
    pub(crate) quantity: usize,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SerialAttributes {
    pub(crate) strength: i32,
    pub(crate) dexterity: i32,
    pub(crate) constitution: i32,
    pub(crate) intelligence: i32,
    pub(crate) wisdom: i32,
    pub(crate) charisma: i32,
}

impl SerialAttributes {
    pub fn zero() -> SerialAttributes {
        Self {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }
}

impl From<SerialDie> for Dice {
    fn from(dice: SerialDie) -> Self {
        Dice::new_from(dice.die, dice.quantity)
    }
}

impl From<SerialAttributes> for Attributes {
    fn from(attrs: SerialAttributes) -> Self {
        Attributes {
            strength: Attribute::Strength(attrs.strength),
            dexterity: Attribute::Dexterity(attrs.dexterity),
            constitution: Attribute::Constitution(attrs.constitution),
            intelligence: Attribute::Intelligence(attrs.intelligence),
            wisdom: Attribute::Wisdom(attrs.wisdom),
            charisma: Attribute::Charisma(attrs.charisma),
        }
    }
}
impl SerialDie {
    pub fn to_die(&self) -> Dice {
        Dice::new_from(self.die, self.quantity)
    }
    pub fn new(die: Die, quantity: usize) -> Self {
        Self { die, quantity }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IndividualItem {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) slot: crate::items::EquipmentSlot,
    pub(crate) armor: Dice,
    pub(crate) resistance: HashMap<DamageType, Dice>,
    pub(crate) damage: HashMap<DamageType, Dice>,
    pub(crate) attribute_bonus: Attributes,
    pub(crate) rarity: crate::items::Rarity,
    pub(crate) action: u32,
    pub(crate) value: u64,
}

impl IndividualItem {
    pub fn to_file(&self, path: String) {
        fs::write(path, serde_yaml::to_string(self).unwrap()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::constructed::ItemsWeHave;

    #[test]
    fn generate_dragon_armor() {
        let dragon_armor = ItemsWeHave::DragonScaleArmor;
        let item = dragon_armor.generate();
        assert_eq!(item.name, "Dragon Scale Armor");
    }

    #[test]
    fn generate_random_item() {
        let item = ItemsWeHave::generate_random();
        assert!(!item.name.is_empty());
    }

    #[test]
    fn generate_sword() {
        let item = ItemsWeHave::generate_slot(crate::items::EquipmentSlot::Weapon);
        assert_eq!(item.slot, crate::items::EquipmentSlot::Weapon);
    }

    #[test]
    pub fn create_a_dwarf_helmet() {
        let item = ItemsWeHave::HelmOfTheDwarfBetrayer.generate();
        assert_eq!(item.name, "Helm of the Dwarf Betrayer");
    }
}
