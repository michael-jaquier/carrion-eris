use crate::classes::Classes;
use crate::enemies::Mob;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Deref, Sub};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub(crate) strength: Attribute,
    pub(crate) intelligence: Attribute,
    pub(crate) dexterity: Attribute,
    pub(crate) constitution: Attribute,
    pub(crate) wisdom: Attribute,
    pub(crate) charisma: Attribute,
}

impl From<&Classes> for Attributes {
    fn from(class: &Classes) -> Self {
        let mut ca = Self::default();
        match class {
            Classes::Warrior => {
                ca.strength = Attribute::Strength(17);
                ca.constitution = Attribute::Constitution(15);
            }

            Classes::Wizard => {
                ca.intelligence = Attribute::Intelligence(17);
                ca.charisma = Attribute::Charisma(7);
            }
            Classes::Sorcerer => {
                ca.charisma = Attribute::Charisma(17);
                ca.intelligence = Attribute::Intelligence(15);
            }
        }
        ca
    }
}

impl From<&Mob> for Attributes {
    fn from(enemy: &Mob) -> Self {
        let mut ca = Self::default();
        match enemy {
            Mob::Orc => {
                ca.strength = Attribute::Strength(17);
                ca.intelligence = Attribute::Intelligence(1);
            }
            Mob::Elf => {
                ca.intelligence = Attribute::Intelligence(22);
                ca.dexterity = Attribute::Dexterity(19);
                ca.constitution = Attribute::Constitution(3);
            }
        }
        ca
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: Attribute::Strength(7),
            intelligence: Attribute::Intelligence(7),
            dexterity: Attribute::Dexterity(0),
            constitution: Attribute::Constitution(0),
            wisdom: Attribute::Wisdom(0),
            charisma: Attribute::Charisma(0),
        }
    }
}

impl Sub for Attributes {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            strength: self.strength - rhs.strength,
            intelligence: self.intelligence - rhs.intelligence,
            dexterity: self.dexterity - rhs.dexterity,
            constitution: self.constitution - rhs.constitution,
            wisdom: self.wisdom - rhs.wisdom,
            charisma: self.charisma - rhs.charisma,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialOrd, PartialEq)]
pub enum Attribute {
    Strength(u32),
    Intelligence(u32),
    Dexterity(u32),
    Constitution(u32),
    Wisdom(u32),
    Charisma(u32),
}

impl Attribute {
    pub fn absolute_difference(&self, other: &Self) -> i32 {
        **self as i32 - **other as i32
    }
    pub fn plus(&mut self, other: u32) {
        match self {
            Attribute::Strength(v) => *v += other,
            Attribute::Intelligence(v) => *v += other,
            Attribute::Dexterity(v) => *v += other,
            Attribute::Constitution(v) => *v += other,
            Attribute::Wisdom(v) => *v += other,
            Attribute::Charisma(v) => *v += other,
        }
    }

    pub fn minus(&mut self, other: u32) {
        match self {
            Attribute::Strength(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Intelligence(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Dexterity(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Constitution(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Wisdom(v) => (*v).checked_sub(other).unwrap_or(0),
            Attribute::Charisma(v) => (*v).checked_sub(other).unwrap_or(0),
        };
    }

    pub fn inner(&self) -> u32 {
        match self {
            Attribute::Strength(v) => *v,
            Attribute::Intelligence(v) => *v,
            Attribute::Dexterity(v) => *v,
            Attribute::Constitution(v) => *v,
            Attribute::Wisdom(v) => *v,
            Attribute::Charisma(v) => *v,
        }
    }
}

impl Deref for Attribute {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        match self {
            Attribute::Strength(v) => v,
            Attribute::Intelligence(v) => v,
            Attribute::Dexterity(v) => v,
            Attribute::Constitution(v) => v,
            Attribute::Wisdom(v) => v,
            Attribute::Charisma(v) => v,
        }
    }
}

impl Add for Attribute {
    type Output = Attribute;

    fn add(self, rhs: Self) -> Self::Output {
        let v = *self + *rhs;
        match self {
            Attribute::Strength(_) => Attribute::Strength(v),
            Attribute::Intelligence(_) => Attribute::Intelligence(v),
            Attribute::Dexterity(_) => Attribute::Dexterity(v),
            Attribute::Constitution(_) => Attribute::Constitution(v),
            Attribute::Wisdom(_) => Attribute::Wisdom(v),
            Attribute::Charisma(_) => Attribute::Charisma(v),
        }
    }
}

impl Sub for Attribute {
    type Output = Attribute;

    fn sub(self, rhs: Self) -> Self::Output {
        let v = self.checked_sub(*rhs).unwrap_or(0);
        match self {
            Attribute::Strength(_) => Attribute::Strength(v),
            Attribute::Intelligence(_) => Attribute::Intelligence(v),
            Attribute::Dexterity(_) => Attribute::Dexterity(v),
            Attribute::Constitution(_) => Attribute::Constitution(v),
            Attribute::Wisdom(_) => Attribute::Wisdom(v),
            Attribute::Charisma(_) => Attribute::Charisma(v),
        }
    }
}
