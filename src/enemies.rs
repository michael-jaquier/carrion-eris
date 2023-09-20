use crate::dice::{Dice, Die};
use crate::items::Item;
use crate::player::Character;
use crate::units::Attributes;
use crate::units::{AttackType, Attribute, DamageType};
use rand::distributions::Standard;
use rand::prelude::{Distribution, SliceRandom};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

use crate::units::Alignment;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnemyState {
    Dead,
    Alive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enemy {
    pub(crate) kind: Mob,
    pub(crate) level: u32,
    pub(crate) experience: u32,
    pub(crate) health: i32,
    pub(crate) defense: Dice,
    pub(crate) resistance: Dice,
    pub(crate) gold: u32,
    pub(crate) attributes: Attributes,
    pub(crate) items: Vec<Item>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
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

    fn super_logarithm_scaling(level: u32) -> u32 {
        let default_scale = |n: u32| ((n as f64).ln().powf(2.0)).floor() as u32 + level * 10;

        default_scale(level)
    }

    fn hp_gain(&self, level: u32) -> u32 {
        let constitution = self.attributes.constitution.inner();
        let hp_gain = match self.kind {
            Mob::Orc => (constitution * 20) + (level * 10),
            Mob::Elf => (constitution * 10) + (level * 10),
            Mob::KingSlime => (constitution * 200) + (level * 50),
        };
        hp_gain
    }

    fn dice_scaling_log(level: u32) -> Dice {
        Dice::new(vec![
            Die::D20.into();
            Enemy::super_logarithm_scaling(level) as usize
        ])
    }
    pub fn weak(mob: Mob, level: u32) -> Enemy {
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level),
            health: Enemy::super_logarithm_scaling(level) as i32,
            defense: Dice::new(vec![Die::D20.into(); 5].into()),
            resistance: Dice::new(vec![Die::D20.into(); 5].into()),
            gold: Enemy::super_logarithm_scaling(level),
            attributes: (&mob).into(),
            items: vec![],
            state: EnemyState::Alive,
            actions: vec![MobAction::Bite, MobAction::Claw, MobAction::Stab],
        }
    }

    pub fn boss(mob: Mob, level: u32) -> Enemy {
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 10),
            health: Enemy::super_logarithm_scaling(level * 50) as i32,
            defense: Dice::new(vec![Die::D20.into(); 20].into()),
            resistance: Dice::new(vec![Die::D20.into(); 15].into()),
            gold: Enemy::super_logarithm_scaling(level * 100),
            attributes: <&Mob as Into<Attributes>>::into((&mob))
                .log_scaling(level)
                .clone(),
            items: vec![],
            state: EnemyState::Alive,
            actions: vec![],
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
            defense: Default::default(),
            resistance: Default::default(),
            gold: 0,
            attributes: Default::default(),
            items: vec![],
            state: EnemyState::Alive,
            actions: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum Mob {
    Orc,
    Elf,
    // Bosses
    KingSlime,
}

impl Mob {
    pub fn generate(&self, character: &Character) -> Enemy {
        let level_range = thread_rng().gen_range(character.level..character.level + 5);
        match self {
            Mob::Orc => Enemy::weak(Mob::Orc, level_range).set_actions(self.actions()),
            Mob::Elf => Enemy::weak(Mob::Elf, level_range)
                .set_actions(self.actions())
                .set_experience_multiple(4),
            Mob::KingSlime => Enemy::boss(Mob::KingSlime, level_range).set_actions(self.actions()),
        }
    }

    pub fn actions(&self) -> Vec<MobAction> {
        match self {
            Mob::Orc => vec![MobAction::Bite, MobAction::Claw, MobAction::Stab],
            Mob::Elf => vec![MobAction::Stab, MobAction::FireBall],
            Mob::KingSlime => vec![
                MobAction::SlimeAbsorb,
                MobAction::SlimeBall,
                MobAction::SlimePunch,
            ],
        }
    }

    pub fn alignment(&self) -> Alignment {
        match self {
            Mob::Orc => Alignment::ChaoticEvil,
            Mob::Elf => Alignment::LawfulGood,
            Mob::KingSlime => Alignment::NeutralEvil,
        }
    }

    pub fn vulnerability(&self) -> Option<DamageType> {
        match self {
            Mob::Orc => Some(DamageType::Fire),
            Mob::Elf => Some(DamageType::Iron),
            _ => None,
        }
    }
}

impl Display for Mob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mob::Orc => write!(f, "Orc 🧌"),
            Mob::Elf => write!(f, "Elf 🧝"),
            Mob::KingSlime => write!(f, "King Slime 👑"),
        }
    }
}

impl Distribution<Mob> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mob {
        match rng.gen_range(0..=1000) {
            // rand 0.8
            0..=100 => Mob::Elf,
            101..=102 => Mob::KingSlime,
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
    SlimePunch,
    SlimeBall,
    SlimeAbsorb,
}

impl MobAction {
    pub fn act(&self, enemy: &Enemy) -> AttackType {
        let attack_dice = |d, n| Dice::new(vec![d; n]);
        let die = |attribute: Attribute, level| {
            let default_scale = |n: u32| ((n as f64).ln().powf(2.0)).floor() as u32;
            (default_scale(attribute.inner()) as usize + default_scale(level) as usize).min(1)
        };
        match self {
            MobAction::Bite => AttackType::Physical(
                attack_dice(Die::D20.into(), die(enemy.attributes.strength, enemy.level)).roll(),
            ),
            MobAction::Claw => AttackType::Physical(
                attack_dice(
                    Die::D6.into(),
                    3 * die(enemy.attributes.dexterity, enemy.level),
                )
                .roll(),
            ),
            MobAction::Stab => AttackType::Physical(
                attack_dice(
                    Die::D12.into(),
                    2 * die(enemy.attributes.dexterity, enemy.level),
                )
                .roll(),
            ),
            MobAction::FireBall => AttackType::Magical(
                attack_dice(
                    Die::D20.into(),
                    die(enemy.attributes.intelligence, enemy.level),
                )
                .roll(),
            ),
            MobAction::SlimePunch => AttackType::Physical(
                attack_dice(
                    Die::D20.into(),
                    die(enemy.attributes.dexterity, enemy.level * 10),
                )
                .roll(),
            ),
            MobAction::SlimeBall => AttackType::Magical(
                attack_dice(
                    Die::D20.into(),
                    die(enemy.attributes.intelligence, enemy.level * 10),
                )
                .roll(),
            ),
            MobAction::SlimeAbsorb => AttackType::Magical(
                attack_dice(
                    Die::D100.into(),
                    die(enemy.attributes.wisdom, enemy.level * 10),
                )
                .roll(),
            ),
        }
    }
}
