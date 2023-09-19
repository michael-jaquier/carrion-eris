use crate::enemies::{Alignment, Enemy};
use crate::player::{Character, PlayerAction};
use crate::traits::{CharacterTraits, TraitMutation};
use std::ops::Div;

use crate::dice::{AdvantageState, Dice, Die, DieObject};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifiers {
    magic: Option<Dice>,
    physical: Option<Dice>,
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

        if let Some(_d) = self.physical.as_mut() {}
        if let Some(_d) = self.magic.as_mut() {}
    }

    fn lower_critical_targets(&mut self) {
        if let Some(d) = self.physical.as_mut() {
            d.dice().iter_mut().for_each(|d| {
                d.set_critical(-1);
            });
        }
        if let Some(d) = self.magic.as_mut() {
            d.dice().iter_mut().for_each(|d| {
                d.set_critical(-1);
            });
        }
    }

    fn set_critical_die(&mut self, die: Die) {
        if let Some(d) = self.physical.as_mut() {
            d.dice().iter_mut().for_each(|d| {
                d.set_critical_die(die.clone());
            });
        }
        if let Some(d) = self.magic.as_mut() {
            d.dice().iter_mut().for_each(|d| {
                d.set_critical_die(die.clone());
            });
        }
    }

    fn apply_skill_base(&mut self, action: &PlayerAction) -> &mut AttackModifiers {
        let (a, b) = action.action_base_damage();
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
        let default_scale = |n: u32| (1.5_f64.powf((n as f64).ln())) as u32;
        let n = action.action_attribute_modifiers(player);
        self.add_existing_die(vec![Die::D6.into(); default_scale(n) as usize]);
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
                CharacterTraits::Cursed => {
                    self.set_negative_die(vec![Die::D4.into(); 2]);
                    self.set_critical_die(Die::D100);
                }
                CharacterTraits::Unlucky => {
                    self.set_advantage_state(AdvantageState::Disadvantage);
                    self.set_critical_die(Die::D100);
                }
                CharacterTraits::Righteous => match enemy.kind.alignment() {
                    Alignment::LawfulEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    Alignment::NeutralEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    Alignment::ChaoticEvil => self.add_existing_die(vec![Die::D4.into(); 2]),
                    _ => {}
                },
                CharacterTraits::Greedy => {}
                CharacterTraits::Keen => {
                    self.lower_critical_targets();
                    self.lower_critical_targets();
                }
            }
        }
        self
    }

    fn physical_range(&self) -> u32 {
        if let Some(physical) = &self.physical {
            let mut rng = thread_rng();
            let phys = physical.roll();
            return rng.gen_range(phys..phys * 2);
        }
        0
    }

    fn magical_range(&self) -> u32 {
        if let Some(magical) = &self.magic {
            let mut rng = thread_rng();
            let magic = magical.roll();
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
    character: Character,
}
impl Default for DefenseModifiers {
    fn default() -> Self {
        Self {
            character: Character::default(),
        }
    }
}
impl DefenseModifiers {
    pub fn new(character: &Character) -> Self {
        Self {
            character: character.clone(),
        }
    }

    fn multi(mutations: &Vec<TraitMutation>) -> f64 {
        let mut multi = 0.0;
        for m in mutations {
            match m {
                TraitMutation::MultiplicativeBonus(e) => {
                    multi += e;
                }
                _ => {}
            }
        }
        multi
    }

    fn advantage(mutations: &Vec<TraitMutation>) -> AdvantageState {
        let mut advantage: i32 = 0;
        let map_advantage = |x: AdvantageState| match x {
            AdvantageState::Advantage => 1,
            AdvantageState::Disadvantage => -1,
            _ => 0,
        };
        for m in mutations {
            match m {
                TraitMutation::Advantage => {
                    advantage = map_advantage(AdvantageState::Advantage);
                }
                TraitMutation::Disadvantage => {
                    advantage = map_advantage(AdvantageState::Disadvantage);
                }
                _ => {}
            }
        }
        advantage.into()
    }

    pub fn dodge(&self) -> bool {
        let mutations = self.character.mutations().get_dodge();
        let advantage = Self::advantage(&mutations);
        let multi = Self::multi(&mutations);
        let map_rolls = |x: bool| if x { 1.0 * multi } else { -1.0 };

        let mut rolls = map_rolls(Dice::default().set_advantage(advantage.into()).success());

        for m in mutations {
            match m {
                TraitMutation::FlatIncrease(e) => {
                    rolls += m(Dice::new(e.clone())
                        .set_advantage(advantage.into())
                        .success());
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= m(Dice::new(e.clone())
                        .set_advantage((advantage * -1).into())
                        .success());
                }
                _ => {}
            }
        }

        rolls > 0.0
    }

    pub fn physical_mitigation(&self) -> f64 {
        let mutations = self.character.mutations().get_physical_mitigation();
        let advantage = Self::advantage(&mutations);
        let multi = Self::multi(&mutations);
        let map_rolls = |x: u32| (x as f64 * multi);

        let mut rolls = map_rolls(Dice::default().set_advantage(advantage.into()).roll());
        for tr in mutations {
            match tr {
                TraitMutation::FlatIncrease(e) => {
                    rolls += map_rolls(Dice::new(e.clone()).set_advantage(advantage.into()).roll());
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= map_rolls(
                        Dice::new(e.clone())
                            .set_advantage((advantage * -1).into())
                            .roll(),
                    );
                }
                _ => {}
            }
        }

        // Negative mitigation is acceptable
        rolls.min(90.0).div(100.0)
    }
    pub fn magical_suppress(&self) -> f64 {
        let mutations = self.character.mutations().get_suppress();
        let advantage = Self::advantage(&mutations);
        let multi = Self::multi(&mutations);
        let map_rolls = |x: bool| if x { 1.0 * multi } else { -1.0 };

        let mut rolls = map_rolls(Dice::default().set_advantage(advantage.into()).success());

        for m in mutations {
            match m {
                TraitMutation::FlatIncrease(e) => {
                    rolls += m(Dice::new(e.clone())
                        .set_advantage(advantage.into())
                        .success());
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= m(Dice::new(e.clone())
                        .set_advantage((advantage * -1).into())
                        .success());
                }
                _ => {}
            }
        }

        if rolls > 0.0 {
            0.5
        } else {
            0.0
        }
    }
}

impl From<&mut Character> for DefenseModifiers {
    fn from(character: &mut Character) -> Self {
        Self {
            character: character.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::classes::Classes;
    use crate::mutators::AttackModifiers;
    use crate::player::Character;

    #[test]
    fn magic_missile_dps() {
        let mut player = Character::new("test".to_string(), 1, Classes::Wizard);

        let enemy = crate::enemies::Enemy::weak(crate::enemies::Mob::Orc, 1);
        let mut attack = AttackModifiers::builder(&player, &enemy);

        let ten_thousand_rolls = (0..10000)
            .map(|_| attack.generate_damage_values())
            .collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;

        assert!(average > 0f64);
        assert!(average < 100f64);
    }
}
