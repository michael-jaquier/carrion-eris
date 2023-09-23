use crate::classes::Classes;
use crate::enemies::Mob;

use serde::{Deserialize, Serialize};
use std::fmt::Display;
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

impl Display for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            self.strength,
            self.intelligence,
            self.dexterity,
            self.constitution,
            self.wisdom,
            self.charisma
        )
    }
}

impl Attributes {
    pub fn log_scaling(&mut self, level: u32) -> &mut Attributes {
        let default_scale = |n: u32| ((n as f64).ln().powf(1.1)).floor() as u32;
        self.constitution.plus(default_scale(level));
        self.strength.plus(default_scale(level));
        self.intelligence.plus(default_scale(level));
        self.dexterity.plus(default_scale(level));
        self.wisdom.plus(default_scale(level));
        self.charisma.plus(default_scale(level));
        self
    }

    pub fn get(&self, attr: &Attribute) -> u32 {
        match attr {
            Attribute::Strength(_) => self.strength.inner(),
            Attribute::Intelligence(_) => self.intelligence.inner(),
            Attribute::Dexterity(_) => self.dexterity.inner(),
            Attribute::Constitution(_) => self.constitution.inner(),
            Attribute::Wisdom(_) => self.wisdom.inner(),
            Attribute::Charisma(_) => self.charisma.inner(),
        }
    }
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
            Mob::KingSlime => {
                ca.intelligence = Attribute::Intelligence(22);
                ca.dexterity = Attribute::Dexterity(22);
                ca.constitution = Attribute::Constitution(22);
                ca.wisdom = Attribute::Wisdom(22);
                ca.strength = Attribute::Strength(22);
                ca.constitution = Attribute::Constitution(22);
            }
            _ => {
                ca.dexterity = Attribute::Dexterity(22);
                ca.constitution = Attribute::Constitution(15);
                ca.charisma = Attribute::Charisma(20);
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
            dexterity: Attribute::Dexterity(7),
            constitution: Attribute::Constitution(7),
            wisdom: Attribute::Wisdom(7),
            charisma: Attribute::Charisma(7),
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

impl From<&str> for Attribute {
    fn from(s: &str) -> Self {
        Attribute::from_text(s).unwrap()
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Strength(v) => write!(f, "Strength: {}", v),
            Attribute::Intelligence(v) => write!(f, "Intelligence: {}", v),
            Attribute::Dexterity(v) => write!(f, "Dexterity: {}", v),
            Attribute::Constitution(v) => write!(f, "Constitution: {}", v),
            Attribute::Wisdom(v) => write!(f, "Wisdom: {}", v),
            Attribute::Charisma(v) => write!(f, "Charisma {}", v),
        }
    }
}

impl Attribute {
    fn from_text(s: &str) -> Result<Attribute, ()> {
        match s.to_lowercase().as_str() {
            "strength" => Ok(Attribute::Strength(0)),
            "intelligence" => Ok(Attribute::Intelligence(0)),
            "dexterity" => Ok(Attribute::Dexterity(0)),
            "constitution" => Ok(Attribute::Constitution(0)),
            "wisdom" => Ok(Attribute::Wisdom(0)),
            "charisma" => Ok(Attribute::Charisma(0)),
            _ => Err(()),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash)]
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
            _ => panic!("Invalid damage type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Eq, Hash)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

impl From<&str> for Alignment {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "lawfulgood" => Alignment::LawfulGood,
            "lawfulneutral" => Alignment::LawfulNeutral,
            "lawfulevil" => Alignment::LawfulEvil,
            "neutralgood" => Alignment::NeutralGood,
            "trueneutral" => Alignment::TrueNeutral,
            "neutralevil" => Alignment::NeutralEvil,
            "chaoticgood" => Alignment::ChaoticGood,
            "chaoticneutral" => Alignment::ChaoticNeutral,
            "chaoticevil" => Alignment::ChaoticEvil,
            _ => panic!("Invalid alignment"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttackType {
    Physical(u32),
    Magical(u32),
}

impl AttackType {
    pub fn inner(&self) -> u32 {
        match self {
            AttackType::Physical(d) => *d,
            AttackType::Magical(d) => *d,
        }
    }
}
