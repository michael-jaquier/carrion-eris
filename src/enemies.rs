use crate::dice::{Dice, Die};
use crate::items::Item;
use crate::player::{ActionDice, Character};
use crate::skills::MobAction;
use crate::units::Attributes;
use crate::units::{AttackType, DamageType};
use eris_macro::{ErisDisplayEmoji, ErisMob};
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
            _ => constitution * 10 + level * 10,
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
            actions: mob.actions(),
        }
    }

    pub fn normal(mob: Mob, level: u32) -> Enemy {
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 2),
            health: Enemy::super_logarithm_scaling(level * 2) as i32,
            defense: Dice::new(vec![Die::D20.into(); 10].into()),
            resistance: Dice::new(vec![Die::D20.into(); 10].into()),
            gold: Enemy::super_logarithm_scaling(level * 2),
            attributes: (&mob).into(),
            items: vec![],
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn strong(mob: Mob, level: u32) -> Enemy {
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 5),
            health: Enemy::super_logarithm_scaling(level * 5) as i32,
            defense: Dice::new(vec![Die::D20.into(); 15].into()),
            resistance: Dice::new(vec![Die::D20.into(); 15].into()),
            gold: Enemy::super_logarithm_scaling(level * 5),
            attributes: (&mob).into(),
            items: vec![],
            state: EnemyState::Alive,
            actions: mob.actions(),
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
            attributes: <&Mob as Into<Attributes>>::into(&mob)
                .log_scaling(level)
                .clone(),
            items: vec![],
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn alive(&self) -> bool {
        match self.state {
            EnemyState::Alive => true,
            EnemyState::Dead => false,
        }
    }

    pub fn action(&self) -> ActionDice {
        let mut rng = thread_rng();
        self.actions
            .choose(&mut rng)
            .expect("No Skill found")
            .act(&self)
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

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq)]
pub enum MobGrade {
    Weak,
    Normal,
    Strong,
    Boss,
}

impl MobGrade {
    pub fn to_enemy(&self, mob: Mob, level: u32) -> Enemy {
        match self {
            MobGrade::Weak => Enemy::weak(mob, level),
            MobGrade::Normal => Enemy::normal(mob, level),
            MobGrade::Strong => Enemy::strong(mob, level),
            MobGrade::Boss => Enemy::boss(mob, level),
        }
    }
}

impl rand::prelude::Distribution<MobGrade> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MobGrade {
        let grade = rng.gen_range(1..=100);
        match grade {
            1..=50 => MobGrade::Weak,
            51..=80 => MobGrade::Normal,
            81..=95 => MobGrade::Strong,
            96..=100 => MobGrade::Boss,
            _ => MobGrade::Weak,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, ErisDisplayEmoji, ErisMob)]
pub enum Mob {
    #[emoji("👹")]
    #[grade(MobGrade::Boss),actions(MobAction::SlimeAbsorb),alignment(Alignment::TrueNeutral)]
    Orc,
    #[emoji("🧝")]
    #[grade(MobGrade::Boss),actions(MobAction::SlimeAbsorb),alignment(Alignment::TrueNeutral)]
    Elf,
    #[emoji("🧟")]
    #[grade(MobGrade::Boss),actions(MobAction::SlimeAbsorb),alignment(Alignment::TrueNeutral)]
    Drow,
    // Bosses
    #[emoji("👑")]
    #[grade(MobGrade::Boss),actions(MobAction::SlimeAbsorb),alignment(Alignment::TrueNeutral)]
    KingSlime,
}

impl Distribution<Mob> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mob {
        let mob = rng.gen_range(1..=100);
        match mob {
            1..=50 => Mob::Orc,
            51..=80 => Mob::Elf,
            81..=95 => Mob::Drow,
            96..=100 => Mob::KingSlime,
            _ => Mob::Orc,
        }
    }
}

impl Mob {
    pub fn generate(&self, character: &Character) -> Enemy {
        let level_range = thread_rng().gen_range(character.level..character.level + 5);
        let enemy: Enemy = self.grade().to_enemy(self.clone(), level_range);
        enemy
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn ttt() {
        let mob = crate::enemies::Mob::Elf;
        let t = mob.alignment();
        assert_eq!(t, Alignment::LawfulGood);
        let t = mob.grade();
        assert_eq!(t, MobGrade::Normal);
        let t = mob.actions();
        assert_eq!(t, vec![MobAction::FireBall]);
    }
}
