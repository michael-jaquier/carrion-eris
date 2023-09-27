use crate::dice::{Dice, Die};
use crate::items::Item;
use crate::player::{ActionDice, Character};
use crate::skills::MobAction;
use crate::units::Attributes;
use crate::units::DamageType;
use eris_macro::{ErisDisplayEmoji, ErisMob, ErisValidEnum};

use rand::prelude::{ SliceRandom};
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
    pub(crate) items: Vec<Item>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
    fn linear_scaling(level: u32) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..(level * 3);
        rng.gen_range(ranges)
    }

    fn super_logarithm_scaling(level: u32) -> u32 {
        let default_scale = |n: u32| ((n as f64).ln().powf(2.0)).floor() as u32 + level * 10;
        default_scale(level)
    }

    fn hp_gain(attributes: &Attributes, level: u32) -> u32 {
        let constitution = attributes.constitution.inner();
        constitution * 10 + level * 10
    }

    pub fn weak(mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        Enemy {
            kind: mob.clone(),
            level,
            experience: Enemy::linear_scaling(level),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: Dice::new(vec![Die::D20.into(); 5].into()),
            resistance: Dice::new(vec![Die::D20.into(); 5].into()),
            gold: Enemy::super_logarithm_scaling(level) as u64,
            attributes,
            items: vec![],
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
            defense: Dice::new(vec![Die::D20.into(); 10].into()),
            resistance: Dice::new(vec![Die::D20.into(); 10].into()),
            gold: Enemy::super_logarithm_scaling(level * 2) as u64,
            attributes,
            items: vec![],
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
            defense: Dice::new(vec![Die::D20.into(); 15].into()),
            resistance: Dice::new(vec![Die::D20.into(); 15].into()),
            gold: Enemy::super_logarithm_scaling(level * 5) as u64,
            attributes: (&mob).into(),
            items: vec![],
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
            defense: Dice::new(vec![Die::D20.into(); 20].into()),
            resistance: Dice::new(vec![Die::D20.into(); 15].into()),
            gold: Enemy::super_logarithm_scaling(level * 100) as u64,
            attributes,
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
    Elf,
    #[emoji("🧟")]
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::Glare, MobAction::FireBall])]
    #[alignment(Alignment::TrueNeutral)]
    Drow,
    #[emoji("👑")]
    #[grade(MobGrade::Boss)]
    #[actions(vec![MobAction::Crush, MobAction::SlimeAbsorb])]
    #[alignment(Alignment::TrueNeutral)]
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
    Bomb,
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
}
