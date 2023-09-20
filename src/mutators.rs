use crate::enemies::Enemy;
use crate::player::{Character, PlayerAction};
use crate::traits::{TraitMutation, TraitMutations};
use std::ops::Div;

use crate::dice::{AdvantageState, Dice, Die, DieObject};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifiers {
    magic: Option<Dice>,
    physical: Option<Dice>,
    player: Character,
    enemy: Enemy,
}

impl AttackModifiers {
    pub fn new(
        magic: Option<Dice>,
        physical: Option<Dice>,
        player: &Character,
        enemy: &Enemy,
    ) -> Self {
        Self {
            magic,
            physical,
            player: player.clone(),
            enemy: enemy.clone(),
        }
    }
    pub fn builder(player: &Character, enemy: &Enemy) -> AttackModifiers {
        AttackModifiers::new(None, None, player, enemy)
            .apply_skill_base(&player.class.action())
            .apply_level_scaling(player, &player.class.action())
            .apply_attributes(enemy, player, &player.class.action())
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
        let action_die = action.action_base_damage();
        self.physical = action_die.physical.clone();
        self.magic = action_die.magical.clone();
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
        let n = action.action_attribute_modifiers(player);
        self.add_existing_die(vec![Die::D6.into(); n as usize]);
        self
    }

    fn apply_vulnerability(
        &mut self,
        enemy: &Enemy,
        action: &PlayerAction,
    ) -> &mut AttackModifiers {
        if let Some(vulnerability) = enemy.kind.vulnerability() {
            if let Some(action_element) =  crate::ElementalScaling::scaling(action) {
                if action_element == vulnerability {
                    self.add_existing_die(vec![Die::D4.into(); 2])
                }
            }
        }
        self
    }

    fn apply_traits(&self, enemy: &Enemy, base: &Dice, mutations: &Vec<TraitMutation>) -> f64 {
        let advantage = TraitMutations::advantage(&mutations);
        let critical_advantage = TraitMutations::critical_advantage(&mutations);
        let critical_multiplier = TraitMutations::critical_multiplier(&mutations);
        let multi = TraitMutations::multi(&mutations);
        let map_unique = |d: Dice, ad: AdvantageState| {
            let mut d = d;
            d.set_critical_advantage(critical_advantage.into());
            d.set_critical_multiplier(critical_multiplier);
            d.set_advantage(ad);
            d.roll() as f64 * multi
        };
        let mut rolls = map_unique(base.clone(), advantage.into());

        for tr in mutations {
            match tr {
                TraitMutation::FlatIncrease(e) => {
                    rolls += map_unique(Dice::new(e.clone()), advantage.into());
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= map_unique(Dice::new(e.clone()), (advantage * -1).into());
                }

                TraitMutation::AlignmentBonus(e, d) => {
                    if &enemy.kind.alignment() == e {
                        rolls += map_unique(Dice::new(d.clone()), advantage.into());
                    }
                }

                _ => {}
            }
        }
        rolls
    }

    fn physical_range(&self) -> u32 {
        if let Some(physical) = &self.physical {
            let dmg = self.apply_traits(
                &self.enemy,
                &physical,
                &self.player.mutations().get_physical_attack(),
            );
            return thread_rng().gen_range(dmg..dmg * 2.0) as u32;
        }
        0
    }

    fn magical_range(&self) -> u32 {
        if let Some(magical) = &self.magic {
            let dmg = self.apply_traits(
                &self.enemy,
                &magical,
                &self.player.mutations().get_physical_attack(),
            );
            return thread_rng().gen_range(dmg..dmg * 2.0) as u32;
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
            player: Default::default(),
            enemy: Default::default(),
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

    pub fn dodge(&self) -> bool {
        let binding = self.character.mutations();
        let mutations = binding.get_dodge();
        let advantage = TraitMutations::advantage(&mutations);
        let multi = TraitMutations::multi(&mutations);
        let map_rolls = |x: bool| if x { 1.0 * multi } else { -1.0 };

        let mut rolls = map_rolls(Dice::default().set_advantage(advantage.into()).success());

        for m in mutations {
            match m {
                TraitMutation::FlatIncrease(e) => {
                    rolls += map_rolls(
                        Dice::new(e.clone())
                            .set_advantage(advantage.into())
                            .success(),
                    );
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= map_rolls(
                        Dice::new(e.clone())
                            .set_advantage((advantage * -1).into())
                            .success(),
                    );
                }
                _ => {}
            }
        }

        rolls > 0.0
    }

    pub fn physical_mitigation(&self) -> f64 {
        let binding = self.character.mutations();
        let mutations = binding.get_armor();
        let advantage = TraitMutations::advantage(&mutations);
        let multi = TraitMutations::multi(&mutations);
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
        let binding = self.character.mutations();
        let mutations = binding.get_suppress();
        let advantage = TraitMutations::advantage(&mutations);
        let multi = TraitMutations::multi(&mutations);
        let map_rolls = |x: bool| if x { 1.0 * multi } else { -1.0 };

        let mut rolls = map_rolls(Dice::default().set_advantage(advantage.into()).success());

        for m in mutations {
            match m {
                TraitMutation::FlatIncrease(e) => {
                    rolls += map_rolls(
                        Dice::new(e.clone())
                            .set_advantage(advantage.into())
                            .success(),
                    );
                }
                TraitMutation::FlatDecrease(e) => {
                    rolls -= map_rolls(
                        Dice::new(e.clone())
                            .set_advantage((advantage * -1).into())
                            .success(),
                    );
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
