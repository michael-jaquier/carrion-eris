use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE, ITEM_TABLE, MOB_TABLE};
use crate::database::Consumer;
use crate::enemies::{Enemy, Mob};
use crate::items::Items;
use crate::player::{Character, SkillSet};
use crate::{CarrionResult, MobQueue};

use serenity::async_trait;
use tracing::debug;

pub struct SurrealConsumer {}

#[async_trait]
impl Consumer for SurrealConsumer {
    async fn get_character(&self, user_id: u64) -> CarrionResult<Option<Character>> {
        let record: Option<Character> = DB.select((CHARACTER_TABLE, user_id)).await?;
        Ok(record)
    }
    async fn get_all_characters(&self) -> CarrionResult<Vec<Character>> {
        let records: Vec<Character> = DB.select(CHARACTER_TABLE).await?;
        Ok(records)
    }

    async fn get_enemy(&self, character: &Character) -> CarrionResult<Option<Enemy>> {
        let record: Option<Enemy> = DB.select((ENEMY_TABLE, character.user_id)).await?;
        Ok(record)
    }

    async fn get_related_mobs(&self, character: &Character) -> CarrionResult<Vec<Mob>> {
        let mobs: Option<MobQueue> = DB.select((MOB_TABLE, character.user_id)).await?;
        Ok(mobs.unwrap_or_default().mobs)
    }

    async fn get_skill(
        &self,
        character: &Character,
        skill_id: u64,
    ) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", character.user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_skill: {:?}", skill);
        Ok(skill)
    }

    async fn get_current_skill(&self, user_id: u64) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", user_id), 999);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_current_skill: {:?}", skill);
        Ok(skill)
    }

    async fn get_skill_id(&self, user_id: u64, skill_id: u64) -> CarrionResult<Option<SkillSet>> {
        debug!("get_skill_id: {:?}, {:?}", user_id, skill_id);
        let key = (format!("{}", user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_skill_id_gotten: {:?}", skill);
        Ok(skill)
    }

    async fn get_items(&self, user_id: u64) -> CarrionResult<Option<Items>> {
        let key = (ITEM_TABLE, user_id);
        let items: Option<Items> = DB.select(key).await?;
        debug!("get_items: {:?}", items);
        Ok(items)
    }
}
