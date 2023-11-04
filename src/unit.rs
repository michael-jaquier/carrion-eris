use crate::class::Classes;
use crate::enemy::Mob;
use crate::enemy::MobGrade;
use crate::log_power_scale;
use crate::AttributeScaling;
use serde::{Deserialize, Serialize};

use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq, Eq, Hash)]
pub struct Attributes {
    pub(crate) strength: i32,
    pub(crate) dexterity: i32,
    pub(crate) constitution: i32,
    pub(crate) intelligence: i32,
    pub(crate) wisdom: i32,
    pub(crate) charisma: i32,
}

impl Display for Attributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('\n');
        s.push_str(&format!("\tStrength: {}\n", self.strength));
        s.push_str(&format!("\tDexterity: {}\n", self.dexterity));
        s.push_str(&format!("\tConstitution: {}\n", self.constitution));
        s.push_str(&format!("\tIntelligence: {}\n", self.intelligence));
        s.push_str(&format!("\tWisdom: {}\n", self.wisdom));
        s.push_str(&format!("\tCharisma: {}\n", self.charisma));
        write!(f, "{}", s)
    }
}

impl Attributes {
    pub fn zero() -> Self {
        Self {
            strength: 0,
            dexterity: 0,
            constitution: 0,
            intelligence: 0,
            wisdom: 0,
            charisma: 0,
        }
    }

    fn from_mob_rarity(mob: MobGrade) -> Self {
        let mob = log_power_scale(mob as i32, None).max(1);
        Self {
            strength: 5 * mob as i32,
            dexterity: 5 * mob as i32,
            constitution: 5 * mob as i32,
            intelligence: 5 * mob as i32,
            wisdom: 5 * mob as i32,
            charisma: 5 * mob as i32,
        }
    }

    pub fn get(&self, attr: &str) -> i32 {
        match attr.to_lowercase().as_str() {
            "strength" => self.strength,
            "dexterity" => self.dexterity,
            "constitution" => self.constitution,
            "intelligence" => self.intelligence,
            "wisdom" => self.wisdom,
            "charisma" => self.charisma,
            _ => panic!("Invalid attribute"),
        }
    }

    pub fn add(&mut self, attr: &str, value: i32) {
        match attr.to_lowercase().as_str() {
            "strength" => self.strength += value,
            "dexterity" => self.dexterity += value,
            "constitution" => self.constitution += value,
            "intelligence" => self.intelligence += value,
            "wisdom" => self.wisdom += value,
            "charisma" => self.charisma += value,
            _ => panic!("Invalid attribute"),
        }
    }

    pub(crate) fn sum(&self) -> i32 {
        let mut sum = 0;
        sum += self.strength;
        sum += self.dexterity;
        sum += self.constitution;
        sum += self.intelligence;
        sum += self.wisdom;
        sum += self.charisma;
        sum
    }
    pub(crate) fn max(&self) -> i32 {
        let mut max = 0;
        max = max.max(self.strength);
        max = max.max(self.dexterity);
        max = max.max(self.constitution);
        max = max.max(self.intelligence);
        max = max.max(self.wisdom);
        max = max.max(self.charisma);
        max
    }

    pub(crate) fn max_stat(&self) -> Option<String> {
        if self.max() == 0 {
            return None;
        }
        let mut max = None;
        if self.strength == self.max() {
            max = Some("strength".to_string());
        }
        if self.dexterity == self.max() {
            max = Some("dexterity".to_string());
        }
        if self.constitution == self.max() {
            max = Some("constitution".to_string());
        }
        if self.intelligence == self.max() {
            max = Some("intelligence".to_string());
        }
        if self.wisdom == self.max() {
            max = Some("wisdom".to_string());
        }
        if self.charisma == self.max() {
            max = Some("charisma".to_string());
        }
        max
    }
}

impl Add for Attributes {
    type Output = Attributes;

    fn add(self, rhs: Self) -> Self::Output {
        Attributes {
            strength: self.strength + rhs.strength,
            dexterity: self.dexterity + rhs.dexterity,
            constitution: self.constitution + rhs.constitution,
            intelligence: self.intelligence + rhs.intelligence,
            wisdom: self.wisdom + rhs.wisdom,
            charisma: self.charisma + rhs.charisma,
        }
    }
}

impl AddAssign for Attributes {
    fn add_assign(&mut self, rhs: Self) {
        self.strength += rhs.strength;
        self.dexterity += rhs.dexterity;
        self.constitution += rhs.constitution;
        self.intelligence += rhs.intelligence;
        self.wisdom += rhs.wisdom;
        self.charisma += rhs.charisma;
    }
}

impl Sub for Attributes {
    type Output = Attributes;

    fn sub(self, rhs: Self) -> Self::Output {
        Attributes {
            strength: self.strength - rhs.strength,
            dexterity: self.dexterity - rhs.dexterity,
            constitution: self.constitution - rhs.constitution,
            intelligence: self.intelligence - rhs.intelligence,
            wisdom: self.wisdom - rhs.wisdom,
            charisma: self.charisma - rhs.charisma,
        }
    }
}

impl From<&Mob> for Attributes {
    fn from(mob: &Mob) -> Self {
        Self::from_mob_rarity(crate::EnemyEvents::grade(mob))
    }
}

impl From<&Classes> for Attributes {
    fn from(classes: &Classes) -> Self {
        let stat = AttributeScaling::scaling(classes).expect("Invalid class");
        let base = Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        };

        match stat.to_owned().as_str() {
            "strength" => {
                base + Self {
                    strength: 8,
                    dexterity: 1,
                    constitution: 5,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                }
            }
            "dexterity" => {
                base + Self {
                    strength: 1,
                    dexterity: 10,
                    constitution: 3,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 0,
                }
            }
            "constitution" => {
                base + Self {
                    strength: 5,
                    dexterity: 0,
                    constitution: 8,
                    intelligence: 0,
                    wisdom: 3,
                    charisma: 3,
                }
            }
            "intelligence" => {
                base + Self {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 8,
                    wisdom: 5,
                    charisma: 0,
                }
            }
            "wisdom" => {
                base + Self {
                    strength: 0,
                    dexterity: 0,
                    constitution: 0,
                    intelligence: 3,
                    wisdom: 10,
                    charisma: 5,
                }
            }
            "charisma" => {
                base + Self {
                    strength: 5,
                    dexterity: 0,
                    constitution: 5,
                    intelligence: 0,
                    wisdom: 0,
                    charisma: 8,
                }
            }
            _ => panic!("Invalid class"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Hash)]
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
