use crate::enemies::{AttackType, DamageType, Enemy, EnemyState};
use crate::{BattleInfo, Dice, Die};

use serde::{Deserialize, Serialize};

use crate::classes::Classes;
use crate::mutators::{AttackModifiers, DefenseModifiers};
use crate::traits::CharacterTraits;
use crate::units::{Attributes};
use std::collections::HashSet;

use std::fmt::{Display, Formatter};

use tracing::log::debug;
use tracing::warn;

pub type PhysicalMagical = ((u32, bool), (u32, bool));
pub type MagicalDice = Option<Dice>;
pub type PhysicalDice = Option<Dice>;
pub type ActionDice = (PhysicalDice, MagicalDice);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerAction {
    Slash,
    MagicMissile,
    FireBall,
}

impl Display for PlayerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerAction::Slash => write!(f, "Slash"),
            PlayerAction::MagicMissile => write!(f, "Magic Missile"),
            PlayerAction::FireBall => write!(f, "Fire Ball"),
        }
    }
}

impl PlayerAction {
    pub fn damage(&self, modifiers: AttackModifiers) -> u32 {
        modifiers.generate_damage_values()
    }

    pub fn act(&self, player: &Character, enemy: &Enemy) -> u32 {
        self.damage(AttackModifiers::builder(player, enemy))
    }

    pub fn action_element(&self) -> Vec<DamageType> {
        match self {
            PlayerAction::Slash => {
                vec![DamageType::Physical]
            }
            PlayerAction::MagicMissile => {
                vec![DamageType::Arcane]
            }
            PlayerAction::FireBall => {
                vec![DamageType::Fire]
            }
        }
    }

    pub fn action_base_damage(&self) -> (Option<Dice>, Option<Dice>) {
        let attack_dice = |d, n| Dice::new(vec![d; n]);
        match self {
            PlayerAction::Slash => (Some(attack_dice(Die::D20.into(), 1)), None),
            PlayerAction::MagicMissile => (None, Some(attack_dice(Die::D4.into(), 5))),
            PlayerAction::FireBall => {
                (
                    Some(attack_dice(Die::D4.into(), 2)),
                    Some(attack_dice(Die::D12.into(), 1)),
                )
            }
        }
    }

    pub fn action_attribute_modifiers(&self, player: &Character) -> u32 {
        let default_scale = |n: u32| ((n as f64).ln().powf(2.0)).floor() as u32;
        match self {
            PlayerAction::Slash => default_scale(player.attributes.strength.inner()),
            PlayerAction::MagicMissile => default_scale(player.attributes.charisma.inner()),
            PlayerAction::FireBall => default_scale(player.attributes.intelligence.inner()),
        }
    }

    pub fn action_level_scaling(&self, n: u32) -> u32 {
        let default_scale = ((n as f64).ln().powf(2.0)).floor() as u32;
        match self {
            PlayerAction::Slash => default_scale,
            PlayerAction::MagicMissile => default_scale,
            PlayerAction::FireBall => default_scale,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub(crate) level: u32,
    pub(crate) name: String,
    pub(crate) user_id: u64,
    pub(crate) class: Classes,
    pub(crate) max_hp: u32,
    pub(crate) hp: i32,
    pub(crate) experience: u32,
    pub(crate) attributes: Attributes,
    pub(crate) traits: HashSet<CharacterTraits>,
    pub(crate) available_traits: u32,
}

impl Character {
    pub fn experience_to_next_level(&self) -> u32 {
        (self.level as f64 * (self.level as f64).ln()) as u32 + self.level * 100
    }
    pub fn new(name: String, user_id: u64, class: Classes) -> Self {
        let max_hp = class.hp_gain(1);
        Self {
            level: 1,
            name,
            user_id,
            class,
            max_hp,
            hp: max_hp as i32,
            experience: 0,
            attributes: (&class).into(),
            traits: HashSet::new(),
            available_traits: 0,
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_hp = self.class.hp_gain(self.level);
        self.hp = self.max_hp as i32;
        self.experience = self
            .experience
            .checked_sub(self.experience_to_next_level())
            .unwrap_or(0);
    }

    pub fn rest(&mut self) {
        self.hp = self.max_hp as i32;
    }

    pub fn player_attack(&mut self, enemy: &mut Enemy) -> BattleInfo {
        let action = self.class.action();
        let damage = action.act(&self, enemy);
        enemy.health -= damage as i32;
        debug!(
            "{} attacked {} for {} damage! {} has {} hp",
            self.name, enemy.kind, damage, enemy.kind, enemy.health
        );

        let mut level = false;
        if enemy.health <= 0 {
            enemy.state = EnemyState::Dead;
            self.experience += enemy.experience;
            while self.experience >= self.experience_to_next_level() {
                self.level_up();
                level = true;
                if self.level % 10 == 0 {
                    self.available_traits += 1;
                }
            }
        }

        BattleInfo {
            action,
            damage: damage as i32,
            player_name: self.name.clone(),
            monster_name: enemy.kind.to_string(),
            kill: enemy.health <= 0,
            critical: false,
            leveled_up: level,
            monster_hp: enemy.health,
            traits_available: self.available_traits,
            next_level: self
                .experience_to_next_level()
                .checked_sub(self.experience)
                .unwrap_or(0),
        }
    }

    pub fn enemy_attack(&mut self, enemy: &Enemy) {
        let action = enemy.action();
        let defense: DefenseModifiers = self.into();
        match action {
            AttackType::Physical(damage) => {
                let dodge = defense.physical.success();
                if dodge {
                    debug!("{} dodged the attack!", self.name);
                } else {
                    self.hp -= damage as i32;
                    debug!(
                        "{} was attacked by {} for {} damage! {} has {} hp",
                        self.name, enemy.kind, damage, self.name, self.hp
                    )
                }
            }

            AttackType::Magical(damage) => {
                if defense.magical.success() {
                    let suppress_quantity = (defense.magical.roll_sum()).min(95) as f64 / 100.0;
                    let suppressed_damage = damage as f64 * suppress_quantity;
                    let damage = (damage as f64 - suppressed_damage) as i32;
                    if damage < 0 {
                        warn!(
                            "Magic Damage is negative! {} suppress_quantity {}",
                            damage, suppress_quantity
                        )
                    }
                    self.hp -= damage;
                } else {
                    self.hp -= damage as i32;
                }
            }
        }
        if self.hp > self.max_hp as i32 {
            warn!("{} has more hp than max hp!", self.name);
            self.hp = self.max_hp as i32;
        }
        self.hp = self.hp.max(0);
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}\n level: {}\n class {}",
            self.name, self.level, self.class
        )
    }
}

#[cfg(test)]
mod test {
    use crate::mutators::AttackModifiers;

    #[test]
    fn attack_modifiers() {
        let attack_modifiers = AttackModifiers::default();
        let damage = attack_modifiers.generate_damage_values();
        assert!(damage > 0);
    }
}
