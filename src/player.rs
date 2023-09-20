use crate::enemies::{Enemy, EnemyState};
use crate::{AttributeScaling, BattleInfo, ElementalScaling};

use serde::{Deserialize, Serialize};

use crate::classes::Classes;
use crate::mutators::{AttackModifiers, DefenseModifiers};
use crate::traits::{CharacterTraits, TraitMutations};
use crate::units::{AttackType, Attribute, Attributes};
use std::collections::HashSet;

use eris_macro::{ErisTitleCase, AttributeScaling, ElementalScaling};
use std::fmt::Display;
use heck::ToTitleCase;
use serenity::model::guild::automod::ActionType;

use crate::dice::{AdvantageState, Dice, Die};
use crate::units::DamageType;
use tracing::log::debug;
use tracing::{Instrument, warn};

pub type PhysicalMagical = ((u32, bool), (u32, bool));
pub type MagicalDice = Option<Dice>;
pub type PhysicalDice = Option<Dice>;



pub struct ActionDice {
    pub physical: Option<Dice>,
    pub magical: Option<Dice>,
}


impl ActionDice {
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
    }}
impl Default for ActionDice {
    fn default() -> Self {
        Self {
            physical: None,
            magical: None,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, AttributeScaling, ElementalScaling)]
pub enum PlayerAction {
    #[stat("intelligence")]
    #[element("physical")]
    Slash,
    #[element("arcane")]
    #[stat("intelligence")]
    MagicMissile,
    #[element("fire")]
    #[stat("intelligence")]
    FireBall,
    #[element("water")]
    #[stat("intelligence")]
    WaterBall,
}

impl Display for PlayerAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!("{:?}", self);
        write!(f, "{}", string.to_title_case())
    }
}



impl PlayerAction {
    pub fn damage(&self, modifiers: AttackModifiers) -> u32 {
        modifiers.generate_damage_values()
    }

    pub fn act(&self, player: &Character, enemy: &Enemy) -> u32 {
        self.damage(AttackModifiers::builder(player, enemy))
    }

    pub fn action_base_damage(&self) -> ActionDice {
        let mut base_die = ActionDice::default();
        if let Some(attribute) = AttributeScaling::scaling(self) {
             match attribute {
                Attribute::Strength(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D20.into(); 1]));
                }
                Attribute::Intelligence(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 4]));

                }
                Attribute::Dexterity(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D12.into(); 2]));

                }
                Attribute::Constitution(_) => {
                    base_die.physical = Some(Dice::new(vec![Die::D4.into(); 2]));
                    base_die.magical = Some(Dice::new(vec![Die::D4.into(); 4]));

                }
                Attribute::Wisdom(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D6.into(); 4]));
                }
                Attribute::Charisma(_) => {
                    base_die.magical = Some(Dice::new(vec![Die::D8.into(); 3]));
                }
            };
        };
        if let Some(elemental) = ElementalScaling::scaling(self) {
            match elemental {
                DamageType::Fire => {
                    base_die.set_critical_state(AdvantageState::Advantage);

                }
                DamageType::Water => {
                }
                DamageType::Earth => {}
                DamageType::Air => {}
                DamageType::Light => {}
                DamageType::Dark => {}
                DamageType::Iron => {}
                DamageType::Arcane => {}
                DamageType::Holy => {
                    base_die.set_critical_state(AdvantageState::Advantage);
                    base_die.magical = Some(Dice::new(vec![Die::D20.into(); 2]));
                }
                DamageType::NonElemental => {}
                DamageType::Physical => {}
            }
        }

      base_die
    }

    pub fn action_attribute_modifiers(&self, player: &Character) -> u32 {
        let default_scale = |n: u32| ((n as f64).ln().powf(1.1)).floor() as u32;
        let attribute =  AttributeScaling::scaling(self);
        if let Some(attribute) = attribute {
            let attribute_value = player.attributes.get(&attribute);
            default_scale(attribute_value)
        } else {
            1
        }
    }

    pub fn action_level_scaling(&self, n: u32) -> u32 {
        let default_scale = ((n as f64).ln().powf(1.1)).floor() as u32;
        match self {
            _ => default_scale,
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
        }
    }
}

impl Character {
    pub fn mutations(&self) -> TraitMutations {
        CharacterTraits::apply_traits(&self.traits)
    }
    pub fn experience_to_next_level(&self) -> u32 {
        (self.level as f64 * (self.level as f64).ln()) as u32 + self.level * 100
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
        }
    }

    pub fn hp_gain(&self, level: u32) -> u32 {
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

    pub fn player_attack(&mut self, enemy: &mut Enemy) -> BattleInfo {
        let action = self.class.action();
        let mut damage = action.act(&self, enemy);

        if enemy.defense.success() {
            let suppress = (enemy.defense.roll()).min(90);
            let suppress_quantity = damage as f64 * suppress as f64 / 100.0;
            damage -= suppress_quantity as u32;
        }

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
        if defense.dodge() {
            return;
        }

        match action {
            AttackType::Physical(damage) => {
                let mitigated_damage =
                    damage - (damage as f64 * defense.physical_mitigation()) as u32;
                if mitigated_damage < 0 {
                    warn!("{} has negative Physical damage!", self.name);
                }
                self.hp -= mitigated_damage as i32;
            }

            AttackType::Magical(damage) => {
                let mitigated_damage = damage - (damage as f64 * defense.magical_suppress()) as u32;
                if mitigated_damage < 0 {
                    warn!("{} has negative Magical damage!", self.name);
                }
                self.hp -= mitigated_damage as i32;
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
    use crate::AttributeScaling;
    use crate::ElementalScaling;
    use crate::mutators::AttackModifiers;
    use crate::units::Attribute::Intelligence;
    use crate::units::DamageType::Arcane;


    #[test]
    fn attack_modifiers() {
        let attack_modifiers = AttackModifiers::default();
        let damage = attack_modifiers.generate_damage_values();
        assert!(damage > 0);
    }

    #[test]
    fn player_action_print() {
        use crate::player::PlayerAction;
        let action = PlayerAction::MagicMissile;
        assert_eq!(action.to_string(), "Magic Missile");
    }

    #[test]
    fn player_action_attributes() {
        use crate::player::PlayerAction;
        use crate::classes::Classes;
        let action = PlayerAction::MagicMissile;
        let element = ElementalScaling::scaling(&action);
        let attribute =  AttributeScaling::scaling(&action);
       assert_eq!(element, Some(Arcane));
        assert_eq!(attribute, Some(Intelligence(0)));
    }

}
