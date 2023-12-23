use crate::character::Character;
use crate::enemy::Enemy;
use crate::BattleInfo;

pub struct GameLoop {
    character: Character,
    enemy: Option<Enemy>,
    battle: Option<BattleInfo>,
}

