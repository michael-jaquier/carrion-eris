use crate::enemy::Enemy;
use crate::{level_up_scaling, BattleInfo};

use serde::{Deserialize, Serialize};
use tracing::{info, trace};

use crate::class::Classes;
use crate::damage::{DamageType, Defense};
use crate::r#trait::{CharacterTraits, TraitMutations};
use crate::unit::Attributes;
use std::collections::HashSet;

use std::fmt::Display;

use crate::item::Equipment;

use crate::skill::SkillSet;
use tracing::log::debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub(crate) level: u32,
    pub(crate) name: String,
    pub(crate) user_id: u64,
    pub(crate) class: Classes,
    pub(crate) max_hp: u32,
    pub(crate) hp: i32,
    pub(crate) experience: u64,
    pub(crate) attributes: Attributes,
    traits: HashSet<CharacterTraits>,
    pub(crate) available_traits: u32,
    pub(crate) current_skill: SkillSet,
    pub(crate) equipment: Equipment,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            level: 1,
            name: String::new(),
            user_id: 0,
            class: Classes::Warrior,
            max_hp: 20,
            hp: 20,
            experience: 0,
            attributes: Attributes::default(),
            traits: HashSet::new(),
            available_traits: 0,
            current_skill: SkillSet::default(),
            equipment: Default::default(),
        }
    }
}

impl Character {
    pub(crate) fn get_traits(&self) -> &HashSet<CharacterTraits> {
        &self.traits
    }

    pub(crate) fn insert_trait(&mut self, trait_: CharacterTraits) -> bool {
        info!("Inserting trait: {:?}", trait_);
        trait_.attribute_mutator(&mut self.attributes);
        info!("Attributes after mutation {:?}", self.attributes);
        self.traits.insert(trait_)
    }

    pub fn mutations(&self) -> TraitMutations {
        CharacterTraits::apply_traits(&self.traits)
    }
    pub fn experience_to_next_level(&self) -> u64 {
        level_up_scaling(self.level, Some(1.0 + (self.level as f64).ln())) + self.level as u64 * 100
    }

    pub fn action_points(&self) -> i32 {
        let mut base_action_points: i32 = 1;
        base_action_points += self.level as i32 / 20i32;
        base_action_points += self.mutations().action_points();
        base_action_points += self.equipment.action_points();
        base_action_points
    }
    pub fn new(name: String, user_id: u64, class: Classes) -> Self {
        let max_hp = match class {
            Classes::Warrior => 20,
            Classes::Wizard => 10,
            Classes::Sorcerer => 10,
            Classes::Paladin => 15,
        };

        let base_skill = SkillSet::new(class.action());

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
            current_skill: base_skill,
            equipment: Default::default(),
        }
    }

    pub fn hp_gain(&self, _level: u32) -> u32 {
        let constitution = self.attributes.constitution;
        let hp_gain = match self.class {
            Classes::Warrior => (constitution * 10) + 10,
            Classes::Wizard => (constitution * 3) + 5,
            Classes::Sorcerer => (constitution * 3) + 5,
            Classes::Paladin => (constitution * 7) + 10,
        } as u32;
        hp_gain + self.max_hp
    }

    pub fn try_trait_gain(&mut self) -> bool {
        if self.level % 10 == 0
            && (self.available_traits + self.traits.len() as u32) < self.level / 10
        {
            self.available_traits += 1;
            return true;
        }

        false
    }

    pub fn try_level_up(&mut self) -> bool {
        if self.experience < self.experience_to_next_level() {
            return false;
        }
        while self.experience >= self.experience_to_next_level() {
            self.experience -= self.experience_to_next_level();
            self.level += 1;
            self.max_hp = self.hp_gain(self.level);
            self.hp = self.max_hp as i32;
        }

        true
    }

    pub fn rest(&mut self) {
        self.hp = self.max_hp as i32;
    }

    pub fn player_attack(&self, enemy: &Enemy, battle_info: &mut BattleInfo) {
        for _ in 0..self.action_points() {
            battle_info.number_of_player_attacks += 1;
            let damage = self.current_skill.act(self, enemy);
            let defense: Defense = enemy.into();

            if defense.dodge() {
                continue;
            }

            let mitigation = defense.defense(damage.dtype().resist_category());
            let damage_taken_pre = damage.damage();
            let damage_taken =
                damage_taken_pre - (damage_taken_pre as f64 * mitigation / 100.0) as i32;

            trace!(
                "Mitigation: {} Damage Taken Pre: {} Damage Taken {} for damage type: {:?}",
                mitigation,
                damage_taken_pre,
                damage_taken,
                damage.dtype()
            );
            let damage = damage.damage();

            battle_info.damage_dealt += damage;
            battle_info.monster_hp = enemy.health;
            debug!(
                "{} attacked {} for {} damage! {} has {} hp",
                self.name, enemy.kind, damage, enemy.kind, enemy.health
            );
            battle_info.skill_experience_gained += (enemy.experience / 10).max(1);
            if battle_info.damage_dealt > enemy.health {
                break;
            }
        }

        if battle_info.damage_dealt > enemy.health {
            battle_info.item_gained.extend(enemy.items.clone());
            battle_info.enemy_killed = true;
            battle_info.gold_gained += enemy.gold;
            battle_info.experience_gained = enemy.experience;
            battle_info.traits_available = self.available_traits;
            trace!(
                "Experience Gained {} Next Level {} Curent Experience {}",
                battle_info.experience_gained,
                self.experience_to_next_level(),
                self.experience
            );
            battle_info.next_level = self
                .experience_to_next_level()
                .saturating_sub(self.experience);
        }
    }

    pub fn enemy_attack(&self, enemy: &Enemy, battle_info: &mut BattleInfo) {
        let (damage, action) = enemy.action();
        let defense: Defense = self.into();
        if damage.dtype() == DamageType::Healing {
            let heal = damage.damage();
            battle_info.player_healing += heal;
            battle_info.enemy_action = action.to_string();
            battle_info.monster_name = enemy.kind.to_string();
            return;
        }

        if defense.dodge() {
            return;
        }

        let mitigation = defense.defense(damage.dtype().resist_category());
        let damage_taken_pre = damage.damage();
        let damage_taken = damage_taken_pre - (damage_taken_pre as f64 * mitigation / 100.0) as i32;
        trace!(
            "Mitigation: {} Damage Taken Pre: {} Damage Taken {} for damage type: {:?}",
            mitigation,
            damage_taken_pre,
            damage_taken,
            damage.dtype()
        );
        battle_info.damage_taken += damage_taken;
        battle_info.enemy_action = action.to_string();
        battle_info.monster_name = enemy.kind.to_string();
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("```");
        string.push('\n');
        string.push_str(&format!("Name: {}\n", self.name));
        string.push_str(&format!("Level: {}\n", self.level));
        string.push_str(&format!("Class: {}\n", self.class));
        string.push_str(&format!("HP: {}/{}\n", self.hp, self.max_hp));
        string.push_str(&format!("Experience: {}\n", self.experience));
        string.push_str(&format!("Attributes: {:?}\n", self.attributes));
        string.push_str("Traits:\n");

        for tr in &self.traits {
            string.push_str(&format!("\t{}\n", tr));
        }
        string.push_str(&format!("Equipment:\n{}", self.equipment));
        string.push_str("```");
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test {}
