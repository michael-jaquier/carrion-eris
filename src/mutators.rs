use crate::enemies::{Alignment, Enemy};
use crate::player::{Character, PlayerAction};
use crate::traits::CharacterTraits;

use crate::{AdvantageState, Dice, Die, DieObject};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifier {
    pub(crate) base: Dice,
}

impl AttackModifier {
    pub fn new(flat: Dice, _multiplier: u32, _crit_chance: Dice, _crit_multiplier: f64) -> Self {
        Self { base: flat }
    }
}

impl Default for AttackModifier {
    fn default() -> Self {
        Self {
            base: Dice::default(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifiers {
    pub(crate) magic: Option<Dice>,
    pub(crate) physical: Option<Dice>,
}

impl AttackModifiers {

    pub fn new(magic: Option<Dice>, physical: Option<Dice>) -> Self {
        Self { magic, physical }
    }
    pub fn builder(player: &Character, enemy: &Enemy) -> AttackModifiers {
        AttackModifiers::default()
            .apply_skill_base(&player.class.action())
            .apply_level_scaling(player, &player.class.action())
            .apply_attributes(enemy, player, &player.class.action())
            .apply_traits(player, enemy)
            .apply_vulnerability(enemy, &player.class.action())
            .clone()
    }


    fn set_advantage_state(&mut self, advantage: AdvantageState) {
        match advantage {
            AdvantageState::Advantage => {
                if let Some(d) = self.physical.as_mut() {
                    d.advantage();
                }
                if let Some(d) = self.magic.as_mut() {
                    d.advantage();
                }
            }
            AdvantageState::Disadvantage => {
                if let Some(d) = self.physical.as_mut() {
                    d.disadvantage();
                }
                if let Some(d) = self.magic.as_mut() {
                    d.disadvantage();
                }
            }
            _ => {}
        }
    }

    fn add_physical_die(&mut self, die: Vec<DieObject>) {
        if let Some(d) = self.physical.as_mut() {
            d.add_die(die.clone());
        }
    }

    fn add_magical_die(&mut self, die: Vec<DieObject>) {
        if let Some(d) = self.magic.as_mut() {
            d.add_die(die.clone());
        }
    }

    fn add_existing_die(&mut self, die: Vec<DieObject>) {
        if self.physical.is_some() && self.magic.is_some() {
            for d in die {
                let choice = thread_rng().gen_bool(0.5);
                if choice {
                    self.physical.as_mut().unwrap().add_die(vec![d]);
                } else {
                    self.magic.as_mut().unwrap().add_die(vec![d]);
                }

            }
        } else {
            if let Some(d) = self.physical.as_mut() {
                d.add_die(die.clone());
            }
            if let Some(d) = self.magic.as_mut() {
                d.add_die(die.clone());
            }
        }


    }


    fn set_negative_die(&mut self, die: Vec<DieObject>) {
        if self.physical.is_some() && self.magic.is_some() {
            for _d in die {
                let _choice = thread_rng().gen_bool(0.5);

            }
        }

        if let Some(_d) = self.physical.as_mut() {

        }
        if let Some(_d) = self.magic.as_mut() {

        }
    }

    fn set_critical_targets(&mut self, _target: u32) {
        if let Some(_d) = self.physical.as_mut() {

        }
        if let Some(_d) = self.magic.as_mut() {

        }
    }


    fn apply_skill_base(&mut self, action: &PlayerAction) -> &mut AttackModifiers {
        let (a,b) = action.action_base_damage();
        self.physical = a;
        self.magic = b;
        self
    }

    fn apply_level_scaling(
        &mut self,
        player: &Character,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        let n = action.action_level_scaling(player.level);
        self.add_existing_die(vec![Die::D4.into(); n as usize]);
        self
    }

    fn apply_attributes(
        &mut self,
        _enemy: &Enemy,
        player: &Character,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        let _attribute = action.action_attribute_modifiers(player) % 10;

        self
    }

    fn apply_vulnerability(
        &mut self,
        enemy: &Enemy,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        let vulnerability = enemy.kind.vulnerability();
        let action_element = action.action_element();
        if action_element.contains(&vulnerability) {
           self.add_existing_die(vec![Die::D4.into(); 2])
        }

        self
    }

    fn apply_traits(&mut self, player: &Character, enemy: &Enemy) -> &mut AttackModifiers {
        for tr in player.traits.iter() {
            match tr {
                CharacterTraits::Robust => {}
                CharacterTraits::Nimble => {}
                CharacterTraits::Genius => self.add_magical_die(vec![Die::D4.into(); 3]),
                CharacterTraits::Lucky => self.set_advantage_state(AdvantageState::Advantage),
                CharacterTraits::FolkHero => match enemy.kind.alignment() {
                    Alignment::ChaoticEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    Alignment::ChaoticNeutral => self.add_existing_die(vec![Die::D4.into(); 2]),
                    _ => {}
                },
                CharacterTraits::Charismatic => {}
                CharacterTraits::Strong => self.add_physical_die(vec![Die::D4.into(); 3]),
                CharacterTraits::Hermit => {}
                CharacterTraits::Addict => self.set_negative_die(vec![Die::D4.into(); 2]),
                CharacterTraits::Cursed => self.set_negative_die(vec![Die::D4.into(); 2]),
                CharacterTraits::Unlucky => {
                    self.set_advantage_state(AdvantageState::Disadvantage);
                }
                CharacterTraits::Righteous => match enemy.kind.alignment() {
                    Alignment::LawfulEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    Alignment::NeutralEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    Alignment::ChaoticEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    _ => {}
                },
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    self.set_critical_targets(18);
                }
            }
        }
        self
    }



    fn physical_range(&self) -> u32 {
        if let Some(physical) = &self.physical {
            let mut rng = thread_rng();
            let phys = physical.roll_sum();
            return rng.gen_range(phys..phys * 2);
        }
        0
    }

    fn magical_range(&self) -> u32 {
        if let Some(magical) = &self.magic {
            let mut rng = thread_rng();
            let magic = magical.roll_sum();
            return rng.gen_range(magic..magic * 2);
        }
        0
    }

    pub fn generate_damage_values(&self) -> u32 {
        self.physical_range() + self.magical_range()
    }
}

impl Default for AttackModifiers {
    fn default() -> Self {
        Self {
            magic: None,
            physical: None,
        }
    }
}

pub struct DefenseModifiers {
    pub(crate) dodge: Dice,
    pub(crate) magical: Dice,
    pub(crate) physical: Dice,
}

impl DefenseModifiers {
    pub fn new(dodge: Dice, magical: Dice, physical: Dice) -> Self {
        Self {
            dodge,
            magical,
            physical,
        }
    }
}

impl From<&mut Character> for DefenseModifiers {
    fn from(character: &mut Character) -> Self {
        let dodge_bonus = [&CharacterTraits::Nimble, &CharacterTraits::Lucky];
        let _dodge_hits = dodge_bonus
            .iter()
            .filter(|&&item| character.traits.contains(&item))
            .count() as u32;

        let suppress_bonus = [&CharacterTraits::Hermit, &CharacterTraits::Cursed];
        let _suppress_hits = suppress_bonus
            .iter()
            .filter(|&&item| character.traits.contains(&item))
            .count() as u32;

        let dodge_dice = |n| Dice::new(vec![Die::D20.into(); n]);
        let dodge = match character.attributes.dexterity.inner() {
            0..=5 => dodge_dice(1),
            6..=10 => dodge_dice(2),
            11..=15 => dodge_dice(2),
            16..=20 => dodge_dice(3),
            _ => dodge_dice(3),
        };

        let physical_dice = |n| Dice::new(vec![Die::D20.into(); n]);
        let physical = match character.attributes.constitution.inner() {
            0..=5 => physical_dice(1),
            6..=10 => physical_dice(2),
            11..=15 => physical_dice(3),
            16..=20 => physical_dice(4),
            _ => physical_dice(5),
        };

        let magical_dice = |n| Dice::new(vec![Die::D20.into(); n]);
        let magical = match character.attributes.wisdom.inner() {
            0..=5 => magical_dice(1),
            6..=10 => magical_dice(2),
            11..=15 => magical_dice(3),
            16..=19 => magical_dice(4),
            20 => magical_dice(5),
            _ => magical_dice(5),
        };

        Self {
            dodge,
            magical,
            physical,
        }
    }
}
