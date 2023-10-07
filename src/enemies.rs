use crate::dice::{Dice, Die};
use std::f64::consts::E;

use crate::player::{ActionDice, Character};
use crate::skills::MobAction;
use crate::units::Attributes;
use crate::units::DamageType;
use eris_macro::{ErisDisplayEmoji, ErisMob, ErisValidEnum};

use crate::constructed::ItemsWeHave;
use crate::sub_linear_scaling;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

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
    pub(crate) gold: u64,
    pub(crate) attributes: Attributes,
    pub(crate) items: Vec<ItemsWeHave>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
    fn linear_scaling(level: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = level..(level * 50);
        let exp = rng.gen_range(ranges);
        exp
    }

    fn hp_gain(attributes: &Attributes, level: u32) -> u32 {
        let constitution = attributes.constitution.inner() as f64;
        let level = level as f64;
        let exp = E.powf(level.ln());
        ((level * constitution + exp) * E.powf(constitution.ln())) as u32
    }

    pub fn weak(mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: Dice::new(vec![Die::D8.into(); 3].into()),
            resistance: Dice::new(vec![Die::D8.into(); 3].into()),
            gold: sub_linear_scaling(level) as u64,
            attributes,
            items: ItemsWeHave::drop_chance(level as u64, mob.grade()),
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn normal(mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 2),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: Dice::new(vec![Die::D8.into(); 5].into()),
            resistance: Dice::new(vec![Die::D8.into(); 5].into()),
            gold: sub_linear_scaling(level * 2) as u64,
            attributes,
            items: ItemsWeHave::drop_chance(level as u64, mob.grade()),
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn strong(mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 5),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: Dice::new(vec![Die::D8.into(); 8].into()),
            resistance: Dice::new(vec![Die::D8.into(); 8].into()),
            gold: sub_linear_scaling(level * 5) as u64,
            attributes: (&mob).into(),
            items: ItemsWeHave::drop_chance(level as u64, mob.grade()),
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn boss(mob: Mob, level: u32) -> Enemy {
        let attributes = <&Mob as Into<Attributes>>::into(&mob)
            .log_scaling(level)
            .clone();
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level * 10),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: Dice::new(vec![Die::D8.into(); 17].into()),
            resistance: Dice::new(vec![Die::D8.into(); 17].into()),
            gold: sub_linear_scaling(level * 100) as u64,
            attributes,
            items: ItemsWeHave::drop_chance(level as u64, mob.grade()),
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

    pub fn action(&self) -> (ActionDice, MobAction) {
        let mut rng = thread_rng();
        let action = self.actions.choose(&mut rng).expect("No Skill found");
        (action.act(&self), action.clone())
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
    Weak = 10,
    Normal = 5,
    Strong = 3,
    Boss = 1,
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

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Copy, ErisDisplayEmoji, ErisMob, ErisValidEnum,
)]
pub enum Mob {
    #[emoji("🧌")]
    #[grade(MobGrade::Weak)]
    #[actions(vec![MobAction::Bite, MobAction::Stab])]
    #[alignment(Alignment::TrueNeutral)]
    #[vulnerability(DamageType::Fire)]
    Orc,
    #[emoji("🧝")]
    #[grade(MobGrade::Normal)]
    #[actions(vec![MobAction::FireBall])]
    #[alignment(Alignment::TrueNeutral)]
    #[vulnerability(DamageType::Iron)]
    Elf,
    #[emoji("🧟")]
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::Glare, MobAction::FireBall])]
    #[alignment(Alignment::TrueNeutral)]
    #[vulnerability(DamageType::Holy)]
    Drow,
    #[emoji("👑")]
    #[grade(MobGrade::Boss)]
    #[actions(vec![MobAction::Crush, MobAction::SlimeAbsorb])]
    #[alignment(Alignment::TrueNeutral)]
    #[vulnerability(DamageType::Despair)]
    KingSlime,
    #[emoji("👹")]
    #[grade(MobGrade::Weak)]
    #[actions(vec![MobAction::Bite])]
    #[alignment(Alignment::ChaoticEvil)]
    #[vulnerability(DamageType::Fire)]
    Goblin,
    #[emoji("🤯")]
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::MindBreak, MobAction::Glare])]
    #[alignment(Alignment::ChaoticNeutral)]
    #[vulnerability(DamageType::Existential)]
    NeuronThief,
    #[emoji("💣")]
    #[grade(MobGrade::Boss)]
    #[actions(vec![MobAction::Explode])]
    #[alignment(Alignment::ChaoticEvil)]
    #[vulnerability(DamageType::Holy)]
    Bomb,
    #[emoji("💀")]
    #[grade(MobGrade::Boss)]
    #[actions(vec![MobAction::NecroticBlast, MobAction::SummonUndead])]
    #[alignment(Alignment::LawfulEvil)]
    #[vulnerability(DamageType::Holy)]
    Lich,
    #[emoji("🧟")]
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::Smash, MobAction::Regenerate])]
    #[alignment(Alignment::ChaoticEvil)]
    #[vulnerability(DamageType::Fire)]
    Troll,
}

impl Mob {
    pub fn generate(&self, character: &Character) -> Enemy {
        let enemy: Enemy = self.grade().to_enemy(self.clone(), character.level);
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

    #[test]
    fn bomb_is_a_bomb() {
        let bomb = Mob::Bomb;
        let t = bomb.alignment();
        assert_eq!(t, Alignment::ChaoticEvil);
        let t = bomb.grade();
        assert_eq!(t, MobGrade::Boss);
        let t = bomb.actions();
        assert_eq!(t, vec![MobAction::Explode]);
        let bomb_string = bomb.actions().first().unwrap().to_string();
        assert_eq!(bomb_string, "💥 Explode 💥");
    }

    #[test]
    fn no_drop() {
        let _enemy = Enemy::weak(Mob::Orc, 1);
        let item = ItemsWeHave::drop_chance(1, MobGrade::Weak);
        assert!(item.is_empty());
    }
    #[test]
    fn drop_almost_sure() {
        let item = ItemsWeHave::drop_chance(1000, MobGrade::Boss);
        assert!(!item.is_empty());
    }
}
