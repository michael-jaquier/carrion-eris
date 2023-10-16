use crate::character::Character;
use crate::constructed::ItemsWeHave;
use crate::enemy::Mob;
use crate::item::{EquipmentSlot, Items};
use crate::r#trait::CharacterTraits;
use crate::skill::Skill;
use crate::BattleInfo;

impl Mutations {
    pub fn user_id(&self) -> &u64 {
        match self {
            Mutations::Skill(user_id, _) => user_id,
            Mutations::Equip(user_id, _) => user_id,
            Mutations::Delete(user_id) => user_id,
            Mutations::Create(character) => &character.user_id,
            Mutations::Trait(user_id, _) => user_id,
            Mutations::AddEnemy(user_id, _, _) => user_id,
            Mutations::Sell(user_id, _, _) => user_id,
            Mutations::NewItems(user_id, _) => user_id,
            Mutations::SynchronizeEnemies(user_id) => user_id,
            Mutations::SynchronizeItems(user_id) => user_id,
            Mutations::SynchronizePlayer(user_id) => user_id,
            Mutations::SynchronizeSkills(user_id) => user_id,
            Mutations::UpdateEnemies(user_id, _) => user_id,
            Mutations::UpdatePlayer(user_id, _) => user_id,
            Mutations::UpdateSkills(user_id, _) => user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Mutations {
    Skill(u64, Skill),
    Equip(u64, ItemsWeHave),
    Delete(u64),
    Create(Character),
    Trait(u64, CharacterTraits),
    AddEnemy(u64, Mob, u32),
    // user_id, optional slot to sell, and the items the user "knew" about before the sell
    Sell(u64, Option<EquipmentSlot>, Option<Items>),
    NewItems(u64, Items),

    UpdateEnemies(u64, BattleInfo),
    UpdatePlayer(u64, BattleInfo),
    UpdateSkills(u64, BattleInfo),

    // Synchros
    SynchronizeEnemies(u64),
    SynchronizeItems(u64),
    SynchronizePlayer(u64),
    SynchronizeSkills(u64),
}
