use std::f64::consts::E;

use crate::damage::{Damage, DamageType};
use crate::item::IndividualItem;
use crate::skill::MobAction;
use crate::unit::Attributes;
use eris_macro::{ErisDisplayEmoji, ErisMob, ErisValidEnum};

use crate::{enemy_defense_scaling, enemy_exp_scaling, sub_linear_scaling};
use rand::prelude::SliceRandom;
use rand::thread_rng;
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
    pub(crate) items: Vec<IndividualItem>,
    pub(crate) state: EnemyState,
    actions: Vec<MobAction>,
}
impl Enemy {
    fn hp_gain(attributes: &Attributes, level: u32) -> u32 {
        let constitution = attributes.constitution as f64;
        let level = level as f64;
        let exp = E.powf(level.ln());
        ((level * constitution + exp) * E.powf(constitution.ln())) as u32 + level as u32 * 110
    }

    pub fn max_health(&self) -> u32 {
        Enemy::hp_gain(&self.attributes, self.level)
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
        self.gold * 3
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
    Champion = 25,
    Elite = 35,
    Legendary = 50,
    Boss = 100,
}

impl MobGrade {
    pub fn to_enemy(&self, mob: Mob, level: u32) -> Enemy {
        let attributes: Attributes = (&mob).into();
        let mut enemy = Enemy {
            kind: mob,
            level,
            experience: enemy_exp_scaling(level),
            health: Enemy::hp_gain(&attributes, level) as i32,
            defense: enemy_defense_scaling(level, mob.grade() as u32) as i32,
            resistance: enemy_defense_scaling(level, mob.grade() as u32) as i32,
            gold: sub_linear_scaling(level * mob.grade() as u32) as u64,
            items: vec![],
            state: EnemyState::Alive,
            actions: mob.actions(),
            attributes,
        };
        enemy.items = (&enemy).into();
        enemy
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
    #[grade(MobGrade::Strong)]
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
    #[grade(MobGrade::Strong)]
    #[actions(vec![MobAction::Explode])]
    #[alignment(Alignment::ChaoticEvil)]
    #[vulnerability(DamageType::Holy)]
    Bomb,
    #[emoji("💀")]
    #[grade(MobGrade::Strong)]
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
    #[emoji("🏆")]
    #[grade(MobGrade::Champion)]
    #[actions(vec![MobAction::Crush])]
    #[alignment(Alignment::LawfulNeutral)]
    #[vulnerability(DamageType::Arcane)]
    Gladiator,
    #[emoji("🐘")]
    #[grade(MobGrade::Elite)]
    #[actions(vec![MobAction::ShadowNova, MobAction::SolarFlare])]
    #[alignment(Alignment::ChaoticNeutral)]
    #[vulnerability(DamageType::Light)]
    Behemoth,
    #[emoji("🦖")]
    #[grade(MobGrade::Elite)]
    #[actions(vec![MobAction::BoneShatter, MobAction::FrostBreath])]
    #[alignment(Alignment::ChaoticNeutral)]
    #[vulnerability(DamageType::Air)]
    Dreadmaw,
    #[emoji("🐉")]
    #[grade(MobGrade::Legendary)]
    #[actions(vec![MobAction::DragonBreath, MobAction::TailSwipe, MobAction::FieryRoar])]
    #[alignment(Alignment::ChaoticEvil)]
    #[vulnerability(DamageType::Earth)]
    Eldragor,
}

impl Mob {
    pub fn generate(&self, level: u32) -> Enemy {
        let enemy: Enemy = self.grade().to_enemy(*self, level);
        enemy
    }
}

#[cfg(test)]
mod test {

    use crate::character::Character;

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

    #[test]
    fn sane_exp_gains() {
        let mut character = Character::new("sd".to_string(), 1, crate::class::Classes::Wizard);
        character.level = 20;
        let enemy = Mob::Orc.generate(character.level);
        let nxt = character.experience_to_next_level();
        assert!(
            enemy.experience < nxt / 5,
            "Enemy: {:?} Next: {}",
            enemy.experience,
            nxt
        );
    }
}
