use crate::character::Character;
use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE, ITEM_TABLE, MOB_TABLE};
use crate::database::Consumer;
use crate::enemy::{Enemy, Mob};
use crate::item::Items;
use crate::{CarrionResult, MobQueue};

use crate::skill::SkillSet;
use serenity::async_trait;
use tracing::{info, instrument, Level};

pub struct SurrealConsumer {}

pub fn info_with_span() {
    info!(target: "database_consumer", "consumed");
}

#[async_trait]
impl Consumer for SurrealConsumer {
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_character(&self, user_id: u64) -> CarrionResult<Option<Character>> {
        let record: Option<Character> = DB.select((CHARACTER_TABLE, user_id)).await?;
        info_with_span();
        Ok(record)
    }
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_all_characters(&self) -> CarrionResult<Vec<Character>> {
        let records: Vec<Character> = DB.select(CHARACTER_TABLE).await?;
        info_with_span();
        Ok(records)
    }

    #[instrument(skip(self, character), target = "database_consumer", fields(user_id = %character.user_id), ret(level = Level::TRACE))]
    async fn get_enemy(&self, character: &Character) -> CarrionResult<Option<Enemy>> {
        let record: Option<Enemy> = DB.select((ENEMY_TABLE, character.user_id)).await?;
        info_with_span();
        Ok(record)
    }

    #[instrument(skip(self, character), target = "database_consumer", fields(user_id = %character.user_id), ret(level = Level::TRACE))]
    async fn get_related_mobs(&self, character: &Character) -> CarrionResult<Vec<Mob>> {
        let mobs: Option<MobQueue> = DB.select((MOB_TABLE, character.user_id)).await?;
        info_with_span();
        Ok(mobs.unwrap_or_default().mobs)
    }

    #[instrument(skip(self, character), target = "database_consumer", fields(user_id = %character.user_id), ret(level = Level::TRACE))]
    async fn get_skill(
        &self,
        character: &Character,
        skill_id: u64,
    ) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", character.user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        info_with_span();
        Ok(skill)
    }

    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_current_skill(&self, user_id: u64) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", user_id), 999);
        let skill: Option<SkillSet> = DB.select(key).await?;
        info_with_span();
        Ok(skill)
    }

    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_skill_id(&self, user_id: u64, skill_id: u64) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        info_with_span();
        Ok(skill)
    }

    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_items(&self, user_id: u64) -> CarrionResult<Option<Items>> {
        let key = (ITEM_TABLE, user_id);
        let items: Option<Items> = DB.select(key).await?;
        info_with_span();
        Ok(items)
    }
}
