use serde::{Deserialize, Serialize};

use std::fmt::Display;

use crate::BattleInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BattleResult {
    pub result: Vec<BattleInfo>,
}

impl BattleResult {
    pub fn new(result: Vec<BattleInfo>) -> Self {
        Self { result }
    }

    pub fn append_result(&mut self, result: BattleInfo) {
        self.result.push(result);
    }
}

impl Display for BattleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for result in &self.result {
            string.push_str(&result.to_string());
            string.push('n')
        }
        write!(f, "{}", string)
    }
}

