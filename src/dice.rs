use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DieObject {
    die: Die,
    advantage: AdvantageState,
    success: u32,
    critical: u32,
    critical_multiplier: f64,
    critical_die: Die,
}

impl DieObject {
    pub fn new(die: Die) -> Self {
        let critical_die = Die::D20;
        Self {
            die,
            advantage: AdvantageState::None,
            success: die.sides(),
            critical: critical_die.sides(),
            critical_multiplier: 1.3,
            critical_die,
        }
    }
    fn roll(&self) -> u32 {
        self.die.roll(self.advantage)
    }
    fn crit(&self) -> bool {
        self.die.roll(self.advantage) >= self.critical
    }
    fn success(&self) -> bool {
        self.die.roll(self.advantage) >= self.success
    }

    fn set_success(&mut self, success: u32) {
        self.success = success;
    }

    fn get_critical_multiplier(&self) -> f64 {
        self.critical_multiplier
    }

    pub fn set_critical_multiplier(&mut self, multiplier: f64) {
        self.critical_multiplier = multiplier;
    }

    pub fn increase_critical_multiplier(&mut self, multiplier: f64) {
        self.critical_multiplier += multiplier;
    }
    pub fn set_critical(&mut self, critical: i32) {
        if critical < 0 {
            self.critical = self
                .critical
                .checked_sub(critical.abs() as u32)
                .unwrap_or(0);
        } else {
            self.critical = self.critical.checked_add(critical as u32).unwrap_or(0);
        }
    }

    pub fn get_sides(&self) -> u32 {
        self.die.sides()
    }

    pub fn set_critical_die(&mut self, die: Die) -> &mut DieObject {
        self.critical_die = die;
        self.critical = die.sides();
        self
    }

    pub fn set_critical_advantage(&mut self, advantage: AdvantageState) -> &mut DieObject {
        self.advantage.transition(advantage);
        self
    }
}

impl From<Die> for DieObject {
    fn from(die: Die) -> Self {
        Self::new(die)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Die {
    fn sides(&self) -> u32 {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }
    fn roll(&self, advantage: AdvantageState) -> u32 {
        let mut rng = rand::thread_rng();
        let ranges = 1..self.sides() + 1;
        if advantage == AdvantageState::Advantage {
            let roll1 = rng.gen_range(ranges.clone());
            let roll2 = rng.gen_range(ranges);
            return std::cmp::max(roll1, roll2);
        }
        if advantage == AdvantageState::Disadvantage {
            let roll1 = rng.gen_range(ranges.clone());
            let roll2 = rng.gen_range(ranges);
            return std::cmp::min(roll1, roll2);
        }
        rng.gen_range(ranges)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Copy, Debug)]
pub enum AdvantageState {
    Advantage = 0,
    Disadvantage = 1,
    None = 2,
}

const STATE_MACHINE: [[AdvantageState; 3]; 3] = [
    // From Advantage
    [
        AdvantageState::None,
        AdvantageState::None,
        AdvantageState::None,
    ],
    // From Disadvantage
    [
        AdvantageState::None,
        AdvantageState::None,
        AdvantageState::None,
    ],
    // From None
    [
        AdvantageState::Advantage,
        AdvantageState::Disadvantage,
        AdvantageState::None,
    ],
];

impl AdvantageState {
    fn as_size(&self) -> usize {
        *self as usize
    }
    pub fn transition(&mut self, new_state: AdvantageState) {
        *self = STATE_MACHINE[self.as_size()][new_state.as_size()];
    }
}

impl From<i32> for AdvantageState {
    fn from(n: i32) -> Self {
        if n > 0 {
            AdvantageState::Advantage
        } else if n < 0 {
            AdvantageState::Disadvantage
        } else {
            AdvantageState::None
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Dice {
    dice: Vec<DieObject>,
}

impl Dice {
    pub fn new(dice: Vec<DieObject>) -> Self {
        Self { dice }
    }
    pub fn empty() -> Self {
        Self { dice: vec![] }
    }
    fn success_roll_all(&self) -> Vec<bool> {
        self.dice.iter().map(|d| d.success()).collect()
    }
    pub fn success(&self) -> bool {
        self.success_roll_all().iter().any(|s| *s)
    }

    pub fn roll(&self) -> u32 {
        self.dice
            .iter()
            .map(|d| {
                let crit = d.crit();
                if crit {
                    (d.roll() as f64 * d.get_critical_multiplier()).ceil() as u32
                } else {
                    d.roll()
                }
            })
            .sum()
    }

    pub fn add_die(&mut self, die: Vec<DieObject>) {
        self.dice.extend(die);
    }

    pub fn advantage(&mut self) -> &mut Dice {
        self.dice
            .iter_mut()
            .for_each(|d| d.advantage.transition(AdvantageState::Advantage));
        self
    }
    pub fn disadvantage(&mut self) -> &mut Dice {
        self.dice
            .iter_mut()
            .for_each(|d| d.advantage.transition(AdvantageState::Disadvantage));
        self
    }

    pub fn set_advantage(&mut self, advantage: AdvantageState) -> &mut Dice {
        self.dice
            .iter_mut()
            .for_each(|d| d.advantage.transition(advantage));
        self
    }

    pub fn dice(&mut self) -> &mut Vec<DieObject> {
        self.dice.as_mut()
    }

    pub fn set_success(&mut self, success: u32) {
        self.dice.iter_mut().for_each(|d| d.set_success(success));
    }

    pub fn set_critical_target(&mut self, critical: i32) {
        self.dice.iter_mut().for_each(|d| d.set_critical(critical));
    }

    pub fn set_critical_multiplier(&mut self, multiplier: f64) {
        self.dice
            .iter_mut()
            .for_each(|d| d.set_critical_multiplier(multiplier));
    }

    pub fn set_critical_advantage(&mut self, advantage: AdvantageState) {
        self.dice.iter_mut().for_each(|d| {
            d.set_critical_advantage(advantage);
        });
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self {
            dice: vec![Die::D20.into()],
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::dice::{AdvantageState, Dice, Die};

    #[test]
    fn test_lower_crit() {
        let die = Die::D20;
        let mut dice = Dice::new(vec![die.into()]);
        let old_crit = dice.dice()[0].critical.clone();
        dice.dice().iter_mut().for_each(|d| d.set_critical(-1));
        let new_crit = dice.dice()[0].critical.clone();
        assert_eq!(new_crit, (old_crit - 1));
    }

    #[test]
    fn expected_value_d20() {
        let dice = Dice::new(vec![Die::D20.into(); 2]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 17.0 && average < 23.0, "average: {}", average);
    }
    #[test]
    fn expected_value_d4() {
        let dice = Dice::new(vec![Die::D4.into(); 2]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 3.0 && average < 6.0, "average: {}", average);
    }
    #[test]
    fn expected_value_d6() {
        let dice = Dice::new(vec![Die::D6.into(); 2]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 6.0 && average < 8.0, "average: {}", average);
    }
    #[test]
    fn expected_value_d8() {
        let dice = Dice::new(vec![Die::D8.into(); 2]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 8.0 && average < 11.0, "average: {}", average);
    }
    #[test]
    fn expected_value_d10() {
        let dice = Dice::new(vec![Die::D10.into(); 2]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 10.0 && average < 14.0, "average: {}", average);
    }

    #[test]
    fn expected_value_many_dice() {
        let dice = Dice::new(vec![Die::D4.into(); 10]);
        let ten_thousand_rolls = (0..10000).map(|_| dice.roll()).collect::<Vec<u32>>();
        let average =
            ten_thousand_rolls.iter().sum::<u32>() as f64 / ten_thousand_rolls.len() as f64;
        assert!(average > 24.0 && average < 26.0, "average: {}", average);
    }

    #[test]
    fn test_advantage_state_change() {
        let mut advantage = AdvantageState::Advantage;
        advantage.transition(AdvantageState::Disadvantage);
        assert_eq!(advantage, AdvantageState::None);
        advantage.transition(AdvantageState::Disadvantage);
        assert_eq!(advantage, AdvantageState::Disadvantage);
    }
}
