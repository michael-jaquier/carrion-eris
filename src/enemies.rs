use crate::player::Character;
use crate::units::Attribute;
use crate::units::Attributes;
use rand::distributions::Standard;
use rand::prelude::{Distribution, SliceRandom};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnemyState {
    Dead,
    Alive,
}
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enemy {
    pub(crate) kind: Mob,
    pub(crate) level: u32,
    pub(crate) experience: u32,
    pub(crate) health: i32,
    pub(crate) defense: u32,
    pub(crate) resistance: u32,
    pub(crate) gold: u32,
    pub(crate) attributes: Attributes,
    pub(crate) items: Vec<Item>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
    pub fn attribute_difference(&self, attribute: Attribute) -> i32 {
        match attribute {
            Attribute::Strength(_) => self.attributes.strength.absolute_difference(&attribute),
            Attribute::Intelligence(_) => self.attributes.strength.absolute_difference(&attribute),
            Attribute::Dexterity(_) => self.attributes.dexterity.absolute_difference(&attribute),
            Attribute::Constitution(_) => {
                self.attributes.constitution.absolute_difference(&attribute)
            }
            Attribute::Wisdom(_) => self.attributes.wisdom.absolute_difference(&attribute),
            Attribute::Charisma(_) => self.attributes.charisma.absolute_difference(&attribute),
        }
    }
    fn ranges(level: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..(level.pow(3)) + 1;
        rng.gen_range(ranges)
    }

    fn slow_scaling_ranges(level: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..(level + 1);
        rng.gen_range(ranges)
    }

    fn linear_scaling(level: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..(level * 10);
        rng.gen_range(ranges)
    }
    pub fn weak(mob: Mob, level: u32) -> Enemy {
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level),
            health: Enemy::ranges(level) as i32,
            defense: Enemy::ranges(level),
            resistance: Enemy::ranges(level),
            gold: Enemy::ranges(level),
            attributes: (&mob).into(),
            items: vec![],
            state: EnemyState::Alive,
            actions: vec![MobAction::Bite, MobAction::Claw, MobAction::Stab],
        }
    }

    pub fn alive(&self) -> bool {
        match self.state {
            EnemyState::Alive => true,
            EnemyState::Dead => false,
        }
    }

    pub fn action(&self) -> AttackType {
        let mut rng = thread_rng();
        self.actions.choose(&mut rng).unwrap().act(&self)
    }

    pub fn set_actions(mut self, actions: Vec<MobAction>) -> Enemy {
        self.actions = actions;
        self
    }

    pub fn set_experience_multiple(mut self, multiple: u32) -> Enemy {
        self.experience *= multiple;
        self
    }
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            kind: Mob::Orc,
            level: 1,
            experience: 0,
            health: 0,
            defense: 0,
            resistance: 0,
            gold: 0,
            attributes: Attributes::default(),
            items: vec![],
            state: EnemyState::Alive,
            actions: vec![],
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Mob {
    Orc,
    Elf,
}

impl Mob {
    pub fn generate(&self, character: &Character) -> Enemy {
        let level_range = thread_rng().gen_range(character.level..character.level + 5);
        match self {
            Mob::Orc => Enemy::weak(Mob::Orc, level_range).set_actions(self.actions()),

            Mob::Elf => Enemy::weak(Mob::Elf, level_range)
                .set_actions(self.actions())
                .set_experience_multiple(4),
        }
    }

    pub fn actions(&self) -> Vec<MobAction> {
        match self {
            Mob::Orc => vec![MobAction::Bite, MobAction::Claw, MobAction::Stab],
            Mob::Elf => vec![MobAction::Stab, MobAction::FireBall],
        }
    }

    pub fn alignment(&self) -> Alignment {
        match self {
            Mob::Orc => Alignment::ChaoticEvil,
            Mob::Elf => Alignment::LawfulGood,
        }
    }

    pub fn vulnerability(&self) -> DamageType {
        match self {
            Mob::Orc => DamageType::Fire,
            Mob::Elf => DamageType::Iron,
        }
    }
}

impl Display for Mob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mob::Orc => write!(f, "Orc 🧌"),
            Mob::Elf => write!(f, "Elf 🧝"),
        }
    }
}

impl Distribution<Mob> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mob {
        match rng.gen_range(0..=20) {
            // rand 0.8
            0 => Mob::Elf,
            _ => Mob::Orc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MobAction {
    Bite,
    Claw,
    Stab,
    FireBall,
}

impl MobAction {
    pub fn act(&self, enemy: &Enemy) -> AttackType {
        match self {
            MobAction::Bite => AttackType::Physical(
                thread_rng().gen_range(1..2 * enemy.level) + *enemy.attributes.strength,
            ),
            MobAction::Claw => AttackType::Physical(
                thread_rng().gen_range(1..2 * enemy.level) + *enemy.attributes.strength,
            ),
            MobAction::Stab => AttackType::Physical(
                thread_rng().gen_range(1..2 * enemy.level) + *enemy.attributes.dexterity,
            ),
            MobAction::FireBall => AttackType::Magical(
                thread_rng().gen_range(1..15 * enemy.level) + *enemy.attributes.intelligence,
            ),
        }
    }
}

impl Distribution<MobAction> for MobAction {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MobAction {
        match rng.gen_range(0..=3) {
            0 => MobAction::Bite,
            1 => MobAction::Claw,
            2 => MobAction::Stab,
            3 => MobAction::FireBall,
            _ => MobAction::Bite,
        }
    }
}
