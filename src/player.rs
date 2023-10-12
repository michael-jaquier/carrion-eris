use crate::enemies::Enemy;
use crate::{
    exp_scaling, ln_power_power_power_scale, log_power_scale, BattleInfo, ElementalScaling,
};

use serde::{Deserialize, Serialize};

use crate::classes::Classes;
use crate::mutators::{AttackModifiers, DefenseModifiers};
use crate::traits::{CharacterTraits, TraitMutations};
use crate::units::{Attributes, DamageType};
use std::collections::HashSet;

use rand::{thread_rng, Rng};

use std::fmt::Display;

use crate::dice::{AdvantageState, Dice, Die, DieObject};
use crate::items::Equipment;
use crate::skills::Skill;
use tracing::log::debug;

pub type PhysicalMagical = ((u32, bool), (u32, bool));
pub type MagicalDice = Option<Dice>;
pub type PhysicalDice = Option<Dice>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionDice {
    pub physical: Option<Dice>,
    pub magical: Option<Dice>,
}

impl ActionDice {
    pub fn len(&self) -> usize {
        let mut len = 0;
        if let Some(d) = &self.physical {
            len += d.len();
        }
        if let Some(d) = &self.magical {
            len += d.len();
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
        string.push('\n');
        string.push_str(&format!("Skill: {}\n", self.skill));
        string.push_str(&format!("Level: {}\n", self.level));
        string.push_str(&format!("Experience: {}\n", self.experience));
        string.push('\n');
        string.push_str("```");
        write!(f, "{}", string)
    }
}

impl SkillSet {
    pub fn new(skill: Skill) -> Self {
        Self {
            skill,
            experience: 0,
            active: true,
            level: 1,
        }
    }

    pub fn try_level_up(&mut self) {
        while self.experience >= self.experience_to_next_level() {
            self.level += 1;
            self.experience = self
                .experience
                .saturating_sub(self.experience_to_next_level());
        }
    }

    pub fn skill(&self) -> Skill {
        self.skill
    }
    pub fn damage(&self, modifiers: AttackModifiers) -> u32 {
        modifiers.generate_damage_values()
    }

    pub fn act(&self, player: &Character, enemy: &Enemy) -> u32 {
        self.damage(AttackModifiers::builder(player, enemy, self))
    }
    pub fn experience_to_next_level(&self) -> u64 {
        exp_scaling(self.level.pow(2))
    }

    pub fn action_base_damage(&self, player: &Character) -> ActionDice {
        let mut base_die = ActionDice::default();
        let mut attributes = player.attributes.clone();
        attributes += player.equipment.attribute();
        self.skill.attribute(&mut base_die, &attributes);
        self.skill.elemental(&mut base_die);
        self.action_level_scaling(&mut base_die, player);

        base_die
    }

    pub fn action_level_scaling(&self, base_die: &mut ActionDice, player: &Character) {
        let scaling = log_power_scale(player.level as i32, Some(1.1)) as usize;
        let additional_die = vec![Die::D4.into(); scaling];

        base_die.add_existing_die(additional_die);
    }

    pub fn action_experience_scaling(&self, base_die: &mut ActionDice) {
        let scaling = log_power_scale(self.level as i32, Some(1.1)) as usize;
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
        ln_power_power_power_scale(self.level) - 200
    }

    pub fn action_points(&self) -> u32 {
        let mut base_action_points = 1;
        base_action_points += self.level / 10;
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
        let constitution = self.attributes.constitution.inner();
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

        self.level += 1;
        self.max_hp = self.hp_gain(self.level);
        self.hp = self.max_hp as i32;
        self.experience = self
            .experience
            .saturating_sub(self.experience_to_next_level());
        true
    }

    pub fn rest(&mut self) {
        self.hp = self.max_hp as i32;
    }

    pub fn player_attack(&self, enemy: &Enemy, battle_info: &mut BattleInfo) {
        for _ in 0..self.action_points() {
            battle_info.number_of_player_attacks += 1;
            let mut damage = self.current_skill.act(self, enemy);
            if enemy.defense.success() {
                let suppress = (enemy.defense.roll()).min(99);
                let suppress_quantity = damage as f64 * suppress as f64 / 100.0;

                damage -= suppress_quantity as u32;
            }

            battle_info.damage_dealt += damage as i32;
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
            battle_info.player_killed = true;
            battle_info.gold_gained += enemy.gold;
            battle_info.experience_gained = enemy.experience;
            battle_info.experience_gained += enemy.experience;
            battle_info.traits_available = self.available_traits;
        }
    }

    pub fn enemy_attack(&self, enemy: &Enemy, battle_info: &mut BattleInfo) {
        let (action, mob_action) = enemy.action();
        let defense: DefenseModifiers = self.into();
        if let Some(regen) = ElementalScaling::scaling(&mob_action) {
            if regen == DamageType::Healing {
                let heal = action.magical().unwrap().roll();
                battle_info.enemy_healing += heal as i32;
                battle_info.enemy_action = mob_action.to_string();
                battle_info.monster_name = enemy.kind.to_string();
                return;
            }
        }
        if defense.dodge() {
            return;
        }

        if action.physical().is_some() {
            let damage = action.physical().unwrap().roll();
            let mitigated_damage = damage - (damage as f64 * defense.physical_mitigation()) as u32;
            battle_info.damage_taken += mitigated_damage as i32;
        }
        if action.magical().is_some() {
            let damage = action.magical().unwrap().roll();

            let mitigated_damage = damage - (damage as f64 * defense.magical_suppress()) as u32;
            battle_info.damage_taken += mitigated_damage as i32;
        }

        battle_info.enemy_action = mob_action.to_string();
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
        string.push_str(&format!("Attributes: {}\n", self.attributes));
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
mod test {
    use crate::mutators::DefenseModifiers;
    use crate::player::Character;
    use crate::units::Attribute::Intelligence;
    use crate::units::DamageType::Arcane;
    use crate::AttributeScaling;
    use crate::ElementalScaling;

    #[test]
    fn player_action_attributes() {
        use crate::skills::Skill;
        let action = Skill::MagicMissile;
        let element = ElementalScaling::scaling(&action);
        let attribute = AttributeScaling::scaling(&action);
        assert_eq!(element, Some(Arcane));
        assert_eq!(attribute, Some(Intelligence(0)));
    }

    #[test]
    fn wizard_armor_is_low() {
        let class = crate::classes::Classes::Wizard;
        let mut character = Character::new("Test".to_string(), 0, class);
        let defense: DefenseModifiers = (&mut character).into();
        let mean_mitigation = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;
        assert!(
            mean_mitigation < 0.1,
            "Mean mitigation was {}",
            mean_mitigation
        );
    }

    #[test]
    fn sorcerer_mitigation_is_low() {
        let class = crate::classes::Classes::Sorcerer;
        let mut character = Character::new("Test".to_string(), 0, class);
        let defense: DefenseModifiers = (&mut character).into();
        let mean_mitigation = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;
        assert!(
            mean_mitigation < 0.05,
            "Mean mitigation was {}",
            mean_mitigation
        );
    }

    #[test]
    fn warrior_mitigation_is_high() {
        let class = crate::classes::Classes::Warrior;
        let mut character = Character::new("Test".to_string(), 0, class);
        let defense: DefenseModifiers = (&mut character).into();
        let mean_mitigation = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;
        assert!(
            mean_mitigation > 0.25,
            "Mean mitigation was {}",
            mean_mitigation
        );
        assert!(
            mean_mitigation < 0.35,
            "Mean mitigation was {}",
            mean_mitigation
        );
    }

    #[test]
    fn robust_traits_works_on_wizard() {
        let class = crate::classes::Classes::Wizard;
        let mut character = Character::new("Test".to_string(), 0, class);
        character
            .traits
            .insert(crate::traits::CharacterTraits::Robust);
        let defense: DefenseModifiers = (&mut character).into();
        let mean_mitigation = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;

        assert!(
            mean_mitigation > 0.15,
            "Mean mitigation was {}",
            mean_mitigation
        );
        assert!(
            mean_mitigation < 0.25,
            "Mean mitigation was {}",
            mean_mitigation
        );
    }

    #[test]
    fn robust_trait_boosts_mitigation() {
        let class = crate::classes::Classes::Warrior;
        let mut character = Character::new("Test".to_string(), 0, class);
        let defense: DefenseModifiers = (&mut character).into();
        let previous_mean = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;
        character
            .traits
            .insert(crate::traits::CharacterTraits::Robust);
        let defense: DefenseModifiers = (&mut character).into();
        let mean_mitigation = (0..10000)
            .map(|_| defense.physical_mitigation())
            .sum::<f64>()
            / 10000.0;

        assert!(
            (mean_mitigation - 0.15) > previous_mean,
            "Expected Robust mean {} to be higher than non-Robust mean {}",
            mean_mitigation,
            previous_mean
        );

        assert!(
            mean_mitigation > 0.45,
            "Mean mitigation was {}",
            mean_mitigation
        );
        assert!(
            mean_mitigation < 0.55,
            "Mean mitigation was {}",
            mean_mitigation
        );
    }
}
