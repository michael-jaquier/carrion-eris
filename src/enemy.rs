use std::f64::consts::E;

use crate::character::Character;
use crate::damage::{Damage, DamageType};
use crate::skill::MobAction;
use crate::unit::Attributes;
use eris_macro::{ErisDisplayEmoji, ErisMob, ErisValidEnum};

use crate::constructed::ItemsWeHave;
use crate::{level_up_scaling, sub_linear_scaling};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::unit::Alignment;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnemyState {
    Dead,
    Alive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enemy {
    pub(crate) kind: Mob,
    pub(crate) level: u32,
    pub(crate) experience: u64,
    pub(crate) health: i32,
    pub(crate) defense: i32,
    pub(crate) resistance: i32,
    pub(crate) gold: u64,
    pub(crate) attributes: Attributes,
    pub(crate) items: Vec<ItemsWeHave>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
    fn linear_scaling(level: u32) -> u64 {
        let mut rng = rand::thread_rng();
        let base = level_up_scaling(level, Some(1.5));
        rng.gen_range(base..2 * base)
    }

    fn hp_gain(attributes: &Attributes, level: u32) -> u32 {
        let constitution = attributes.constitution as f64;
        let level = level as f64;
        let exp = E.powf(level.ln());
        ((level * constitution + exp) * E.powf(constitution.ln())) as u32 + level as u32 * 110
    }

    pub fn weak(mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        Enemy {
            kind: mob,
            level,
            experience: Enemy::linear_scaling(level),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: 10,
            resistance: 10,
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
            kind: mob,
            level,
            experience: Enemy::linear_scaling(level * 2),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: 25,
            resistance: 25,
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
            kind: mob,
            level,
            experience: Enemy::linear_scaling(level * 5),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: 30,
            resistance: 30,
            gold: sub_linear_scaling(level * 5) as u64,
            attributes: (&mob).into(),
            items: ItemsWeHave::drop_chance(level as u64, mob.grade()),
            state: EnemyState::Alive,
            actions: mob.actions(),
        }
    }

    pub fn boss(mob: Mob, level: u32) -> Enemy {
        let attributes = <&Mob as Into<Attributes>>::into(&mob).clone();
        Enemy {
            kind: mob,
            level,
            experience: Enemy::linear_scaling(level * 10),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: 100,
            resistance: 100,
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

    pub fn action(&self) -> (Damage, MobAction) {
        let mut rng = thread_rng();
        let action = self.actions.choose(&mut rng).expect("No Skill found");
        (action.base_damage(self), action.clone())
    }

    pub fn set_actions(mut self, actions: Vec<MobAction>) -> Enemy {
        self.actions = actions;
        self
    }

    pub fn set_experience_multiple(mut self, multiple: u32) -> Enemy {
        self.experience *= multiple as u64;
        self
    }

    pub fn cost(&self) -> u64 {
        self.gold * 10 * self.kind.grade() as u64 * self.level as u64
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
    Weak = 1,
    Normal = 5,
    Strong = 10,
    Boss = 50,
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

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
    Copy,
    Hash,
    ErisDisplayEmoji,
    ErisMob,
    ErisValidEnum,
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
    #[alignment(Alignment::LawfulGood)]
    #[vulnerability(DamageType::Iron)]
    Elf,
    #[emoji("🧟")]
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::Glare, MobAction::FireBall])]
    #[alignment(Alignment::LawfulEvil)]
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
        let enemy: Enemy = self.grade().to_enemy(*self, character.level);
        enemy
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
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
    #[ignore]
    fn no_drop() {
        let _enemy = Enemy::weak(Mob::Orc, 1);
        let item = ItemsWeHave::drop_chance(1, MobGrade::Weak);
        assert!(item.is_empty());
    }

    #[test]
    #[ignore]
    fn drop_almost_sure() {
        let item = ItemsWeHave::drop_chance(1000, MobGrade::Boss);
        assert!(!item.is_empty());
    }
    #[test]
    fn expected_mob_probability() {
        let mobs: Vec<Mob> = (0..10000).map(|_| rand::random()).collect();
        let mut mob_counts = std::collections::HashMap::new();
        for mob in mobs.clone() {
            let count = mob_counts.entry(mob).or_insert(0);
            *count += 1;
        }
        println!("{:?}", mob_counts);
        assert!(
            mobs.iter().filter(|&m| m == &Mob::Orc).count() > 2000,
            "Mob Grades: {:?}",
            mob_counts
        );
    }
}
