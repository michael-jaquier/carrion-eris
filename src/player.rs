use crate::enemies::{Enemy, EnemyState};
use crate::{
    ln_power_power_power_scale, log_power_power_scale, log_power_scale, AttributeScaling,
    BattleInfo, CarrionResult, ElementalScaling,
};

use serde::{Deserialize, Serialize};

use crate::classes::Classes;
use crate::mutators::{AttackModifiers, DefenseModifiers};
use crate::traits::{CharacterTraits, TraitMutations};
use crate::units::{AttackType, Attributes};
use std::collections::HashSet;

use rand::{thread_rng, Rng};

use std::fmt::Display;

use crate::dice::{AdvantageState, Dice, Die, DieObject};
use crate::items::Equipment;
use crate::skills::Skill;
use tracing::log::debug;
use tracing::warn;

pub type PhysicalMagical = ((u32, bool), (u32, bool));
pub type MagicalDice = Option<Dice>;
pub type PhysicalDice = Option<Dice>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionDice {
    pub physical: Option<Dice>,
    pub magical: Option<Dice>,
}

impl Default for ActionDice {
    fn default() -> Self {
        Self {
            physical: None,
            magical: None,
        }
    }
}

impl ActionDice {
    pub fn physical_mut(&mut self) -> Option<&mut Dice> {
        self.physical.as_mut()
    }
    pub fn magical_mut(&mut self) -> Option<&mut Dice> {
        self.magical.as_mut()
    }

    pub fn physical(&self) -> Option<&Dice> {
        self.physical.as_ref()
    }
    pub fn magical(&self) -> Option<&Dice> {
        self.magical.as_ref()
    }
    pub fn add_existing_die(&mut self, die: Vec<DieObject>) {
        if self.physical.is_some() && self.magical.is_some() {
            for d in die {
                let choice = thread_rng().gen_bool(0.5);
                if choice {
                    self.physical.as_mut().unwrap().add_die(vec![d]);
                } else {
                    self.magical.as_mut().unwrap().add_die(vec![d]);
                }
            }
        } else {
            if let Some(d) = self.physical.as_mut() {
                d.add_die(die.clone());
            }
            if let Some(d) = self.magical.as_mut() {
                d.add_die(die.clone());
            }
        }
    }
    pub fn set_critical_state(&mut self, state: AdvantageState) {
        if let Some(dice) = &mut self.physical {
            dice.set_critical_advantage(state)
        }
        if let Some(dice) = &mut self.magical {
            dice.set_critical_advantage(state)
        }
    }

    pub fn set_critical_target(&mut self, target: i32) {
        if let Some(dice) = &mut self.physical {
            dice.set_critical_target(target)
        }
        if let Some(dice) = &mut self.magical {
            dice.set_critical_target(target)
        }
    }
}

impl From<Skill> for SkillSet {
    fn from(value: Skill) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillSet {
    pub(crate) skill: Skill,
    pub(crate) experience: u64,
    pub(crate) active: bool,
    pub(crate) level: u32,
}
impl Default for SkillSet {
    fn default() -> Self {
        Self {
            skill: Skill::Slash,
            experience: 0,
            active: false,
            level: 1,
        }
    }
}

impl Display for SkillSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("```");
        string.push_str("\n");
        string.push_str(&format!("Skill: {}\n", self.skill));
        string.push_str(&format!("Level: {}\n", self.level));
        string.push_str(&format!("Experience: {}\n", self.experience));
        string.push_str("\n");
        string.push_str("```");
        write!(f, "{}", string)
    }
}

impl SkillSet {
    pub fn new(skill: Skill) -> Self {
        Self {
            skill: skill.clone(),
            experience: 0,
            active: true,
            level: 1,
        }
    }

    pub fn skill(&self) -> Skill {
        self.skill.clone()
    }
    pub fn damage(&self, modifiers: AttackModifiers) -> u32 {
        modifiers.generate_damage_values()
    }

    pub fn act(&self, player: &Character, enemy: &Enemy) -> u32 {
        self.damage(AttackModifiers::builder(player, enemy, self))
    }
    pub fn experience_to_next_level(&self) -> u64 {
        ln_power_power_power_scale(self.level) as u64
    }

    pub fn action_base_damage(&self, player: &Character) -> ActionDice {
        let mut base_die = ActionDice::default();
        self.skill.attribute(&mut base_die, &player.attributes);
        self.skill.elemental(&mut base_die);
        self.action_level_scaling(&mut base_die, player);
        base_die
    }

    pub fn action_level_scaling(&self, base_die: &mut ActionDice, player: &Character) {
        let scaling = log_power_scale(player.level, Some(1.1)) as usize;
        let additional_die = vec![Die::D4.into(); scaling];
        base_die.add_existing_die(additional_die);
    }

    pub fn action_experience_scaling(&self, base_die: &mut ActionDice) {
        let scaling = log_power_scale(self.level, Some(1.1)) as usize;
        let additional_die = vec![Die::D4.into(); scaling];
        base_die.add_existing_die(additional_die);
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
    pub fn mutations(&self) -> TraitMutations {
        CharacterTraits::apply_traits(&self.traits)
    }
    pub fn experience_to_next_level(&self) -> u32 {
        ln_power_power_power_scale(self.level)
    }
    pub fn new(name: String, user_id: u64, class: Classes) -> Self {
        let max_hp = match class {
            Classes::Warrior => 20,
            Classes::Wizard => 10,
            Classes::Sorcerer => 10,
        };
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
            current_skill: SkillSet::default(),
            equipment: Default::default(),
        }
    }

    pub fn hp_gain(&self, _level: u32) -> u32 {
        let constitution = self.attributes.constitution.inner();
        let hp_gain = match self.class {
            Classes::Warrior => (constitution * 10) + 10,
            Classes::Wizard => (constitution * 3) + 5,
            Classes::Sorcerer => (constitution * 3) + 5,
        };
        hp_gain + self.max_hp
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_hp = self.hp_gain(self.level);
        self.hp = self.max_hp as i32;
        self.experience = self
            .experience
            .checked_sub(self.experience_to_next_level())
            .unwrap_or(0);
    }

    pub fn rest(&mut self) {
        self.hp = self.max_hp as i32;
    }

    pub fn player_attack(&mut self, enemy: &mut Enemy, battle_info: &mut BattleInfo) {
        let mut damage = self.current_skill.act(self, enemy);
        if enemy.defense.success() {
            let suppress = (enemy.defense.roll()).min(90);
            let suppress_quantity = damage as f64 * suppress as f64 / 100.0;
            damage -= suppress_quantity as u32;
        }

        enemy.health -= damage as i32;
        battle_info.damage_dealt += damage as i32;
        battle_info.monster_hp += enemy.health;
        debug!(
            "{} attacked {} for {} damage! {} has {} hp",
            self.name, enemy.kind, damage, enemy.kind, enemy.health
        );

        let mut level = false;
        if enemy.health <= 0 {
            battle_info.kill = true;
            battle_info.gold_gained += enemy.gold;
            enemy.state = EnemyState::Dead;
            self.experience += enemy.experience;
            while self.experience >= self.experience_to_next_level() {
                self.level_up();
                level = true;
                battle_info.leveled_up = true;
                if self.level % 10 == 0 {
                    self.available_traits += 1;
                    battle_info.traits_available += 1;
                }
            }
            self.current_skill.experience += enemy.experience as u64;
            battle_info.skill_experience_gained += enemy.experience;
            while self.current_skill.experience
                >= self.current_skill.experience_to_next_level() as u64
            {
                self.current_skill.level += 1;
                self.current_skill.experience = self
                    .current_skill
                    .experience
                    .checked_sub(self.current_skill.experience_to_next_level())
                    .unwrap_or(0);
            }
        }
    }

    pub fn enemy_attack(&mut self, enemy: &Enemy, battle_info: &mut BattleInfo) {
        let (action, mob_action) = enemy.action();
        let defense: DefenseModifiers = self.into();
        if defense.dodge() {
            return;
        }

        if action.physical().is_some() {
            let damage = action.physical().unwrap().roll();
            let mitigated_damage = damage - (damage as f64 * defense.physical_mitigation()) as u32;
            if mitigated_damage < 0 {
                warn!("{} has negative Physical damage!", self.name);
            }
            battle_info.damage_taken += mitigated_damage as i32;
            self.hp -= mitigated_damage as i32;
        }
        if action.magical().is_some() {
            let damage = action.magical().unwrap().roll();
            let mitigated_damage = damage - (damage as f64 * defense.magical_suppress()) as u32;
            if mitigated_damage < 0 {
                warn!("{} has negative Magical damage!", self.name);
            }
            battle_info.damage_taken += mitigated_damage as i32;
            self.hp -= mitigated_damage as i32;
        }

        if self.hp > self.max_hp as i32 {
            warn!("{} has more hp than max hp!", self.name);
            self.hp = self.max_hp as i32;
        }
        self.hp = self.hp.max(0);
        battle_info.enemy_action = mob_action.to_string();
        battle_info.monster_name = enemy.kind.to_string();
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str("```");
        string.push_str("\n");
        string.push_str(&format!("Name: {}\n", self.name));
        string.push_str(&format!("Level: {}\n", self.level));
        string.push_str(&format!("Class: {}\n", self.class));
        string.push_str(&format!("HP: {}/{}\n", self.hp, self.max_hp));
        string.push_str(&format!("Experience: {}\n", self.experience));
        string.push_str(&format!("Attributes: {}\n", self.attributes));
        string.push_str(&format!("Traits:\n",));
        for tr in &self.traits {
            string.push_str(&format!("\t{}\n", tr));
        }
        string.push_str("\n");
        string.push_str("```");
        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test {
    use crate::mutators::AttackModifiers;
    use crate::units::Attribute::Intelligence;
    use crate::units::DamageType::Arcane;
    use crate::AttributeScaling;
    use crate::ElementalScaling;

    #[test]
    fn attack_modifiers() {
        let attack_modifiers = AttackModifiers::default();
        let damage = attack_modifiers.generate_damage_values();
        assert!(damage > 0);
    }

    #[test]
    fn player_action_print() {
        use crate::skills::Skill;
        let action = Skill::MagicMissile;
        assert_eq!(action.to_string(), "Magic Missile");
    }

    #[test]
    fn player_action_attributes() {
        use crate::classes::Classes;
        use crate::skills::Skill;
        let action = Skill::MagicMissile;
        let element = ElementalScaling::scaling(&action);
        let attribute = AttributeScaling::scaling(&action);
        assert_eq!(element, Some(Arcane));
        assert_eq!(attribute, Some(Intelligence(0)));
    }
}
