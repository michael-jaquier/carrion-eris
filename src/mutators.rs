use crate::enemies::Enemy;
use crate::player::{ActionDice, Character, SkillSet};
use crate::traits::{TraitMutation, TraitMutations};
use std::ops::Div;

use crate::dice::{AdvantageState, Dice, Die, DieObject};
use crate::skills::Skill;
use crate::{ElementalScaling, EnemyEvents};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AttackModifiers {
    dice_set: ActionDice,
    player: Character,
    skill: SkillSet,
    enemy: Enemy,
}

impl AttackModifiers {
    pub fn new(dice_set: ActionDice, player: &Character, enemy: &Enemy, skill: &SkillSet) -> Self {
        Self {
            dice_set,
            player: player.clone(),
            enemy: enemy.clone(),
            skill: skill.clone(),
        }
    }
    pub fn builder(player: &Character, enemy: &Enemy, skill: &SkillSet) -> AttackModifiers {
        AttackModifiers::new(Default::default(), player, enemy, skill)
            .apply_skill_base(&player)
            .apply_vulnerability(enemy, &player.class.action())
            .clone()
    }

    fn physical_mut(&mut self) -> Option<&mut Dice> {
        self.dice_set.physical_mut()
    }

    fn magical_mut(&mut self) -> Option<&mut Dice> {
        self.dice_set.magical_mut()
    }

    fn physical(&self) -> Option<&Dice> {
        self.dice_set.physical()
    }

    fn magical(&self) -> Option<&Dice> {
        self.dice_set.magical()
    }

    fn add_existing_die(&mut self, die: Vec<DieObject>) {
        if self.physical_mut().is_some() && self.magical_mut().is_some() {
            for d in die {
                let choice = thread_rng().gen_bool(0.5);
                if choice {
                    self.physical_mut().as_mut().unwrap().add_die(vec![d]);
                } else {
                    self.magical_mut().as_mut().unwrap().add_die(vec![d]);
                }
            }
        } else {
            if let Some(d) = self.physical_mut().as_mut() {
                d.add_die(die.clone());
            }
            if let Some(d) = self.magical_mut().as_mut() {
                d.add_die(die.clone());
            }
        }
    }

    fn apply_skill_base(&mut self, player: &Character) -> &mut AttackModifiers {
        self.dice_set = self.skill.action_base_damage(player);
        self
    }

    fn apply_vulnerability(&mut self, enemy: &Enemy, action: &Skill) -> &mut AttackModifiers {
        if let Some(vulnerability) = enemy.kind.vulnerability() {
            if let Some(action_element) = crate::ElementalScaling::scaling(action) {
                if action_element == vulnerability {
                    self.add_existing_die(vec![Die::D10.into(); 2])
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
        if let Some(physical) = &self.physical() {
            let mut dmg = self.apply_traits(
                &self.enemy,
                &physical,
                &self.player.mutations().get_physical_attack(),
            );
            if let Some(dice) = self.skill.action_base_damage(&self.player).physical {
                dmg += dmg * dice.roll() as f64 / 5.0;
            }
            if let Some(element) = self.skill.skill.scaling() {
                self.player.equipment.damage().get(&element).map(|x| {
                    dmg += x.roll() as f64;
                });
            }
            return thread_rng().gen_range(dmg..dmg * 2.0) as u32;
        }
        0
    }

    fn magical_range(&self) -> u32 {
        if let Some(magical) = &self.magical() {
            let mut dmg = self.apply_traits(
                &self.enemy,
                &magical,
                &self.player.mutations().get_physical_attack(),
            );
            if let Some(dice) = self.skill.action_base_damage(&self.player).physical {
                dmg += dmg * dice.roll() as f64 / 5.0;
            }
            if let Some(element) = self.skill.skill.scaling() {
                self.player.equipment.damage().get(&element).map(|x| {
                    dmg += x.roll() as f64;
                });
            }

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
            dice_set: Default::default(),
            player: Default::default(),
            skill: Default::default(),
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

        let base = (self.character.class.armor_scaling()
            * ((self.character.attributes.constitution.inner()
                + self.character.attributes.strength.inner()) as f64
                / 2_f64))
            .floor() as usize;

        let mut rolls = map_rolls(
            Dice::new(vec![Die::D4.into(); base.max(1)])
                .set_advantage(advantage.into())
                .roll(),
        );
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
    use crate::player::{Character, SkillSet};

    #[test]
    fn magic_missile_dps() {
        let player = Character::new("test".to_string(), 1, Classes::Wizard);

        let enemy = crate::enemies::Enemy::weak(crate::enemies::Mob::Orc, 1);
        let attack = AttackModifiers::builder(&player, &enemy, &SkillSet::default());

        let ten_thousand_rolls = (0..10000)
            .map(|_| attack.generate_damage_values())
            .collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;

        assert!(average > 0f64);
        assert!(average < 100f64);
    }
}
