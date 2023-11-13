use crate::database::surreal::{CHARACTER_TABLE, DB, ITEM_TABLE, MOB_TABLE};
use crate::database::Producer;
use crate::enemy::Mob;

use crate::character::Character;
use crate::{CarrionResult, MobQueue, Record};

use serenity::async_trait;

use crate::skill::SkillSet;
use tracing::{info, instrument, Level};

pub struct SurrealProducer {}

pub fn info_with_span() {
    info!(target: "database_producer", "produced");
}
#[async_trait]
impl Producer for SurrealProducer {
    #[instrument(skip(self, content),  target = "database_producer", fields(user_id = %content.user_id), ret(level = Level::TRACE))]
    async fn create_character(&self, content: Character) -> CarrionResult<()> {
        let user_id = content.user_id;
        let _record: Option<Record> = DB
            .create((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;

        _ = self.create_user_items(user_id).await;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self, content),  target = "database_producer", fields(user_id = %content.user_id), ret(level = Level::TRACE))]
    async fn create_or_update_character(&self, content: Character) -> CarrionResult<()> {
        let _record: Option<Record> = DB
            .update((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn delete_character(&self, user_id: u64) -> CarrionResult<()> {
        let _record: Option<Record> = DB.delete((CHARACTER_TABLE, user_id)).await?;
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn create_or_update_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        let key = (format!("{}", user_id), content.skill() as u64);
        let _skill: Option<Record> = DB.update(key).content(content).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn set_current_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        let key = (format!("{}", user_id), 999);
        let _skill: Option<Record> = DB.update(key).content(content).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self, character), target = "database_producer", fields(user_id = %character.user_id), ret(level = Level::TRACE))]
    async fn store_mob_queue(&self, character: &Character, enemies: Vec<Mob>) -> CarrionResult<()> {
        let _record: Option<Record> = DB
            .update((MOB_TABLE, character.user_id))
            .content(MobQueue { mobs: enemies })
            .await?;

        info_with_span();

        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn delete_mob_queue(&self, user_id: u64) -> CarrionResult<()> {
        let _record: Option<Record> = DB.delete((MOB_TABLE, user_id)).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn store_user_items(
        &self,
        content: crate::item::Items,
        user_id: u64,
    ) -> CarrionResult<()> {
        let _record: Option<Record> = DB.update((ITEM_TABLE, user_id)).content(content).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn delete_user_items(&self, user_id: u64) -> CarrionResult<()> {
        let _record: Option<Record> = DB.delete((ITEM_TABLE, user_id)).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn create_user_items(&self, user_id: u64) -> CarrionResult<()> {
        let items = crate::item::Items::default();
        let _: Option<Record> = DB.update((ITEM_TABLE, user_id)).content(items).await?;
        info_with_span();
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer", ret(level = Level::TRACE))]
    async fn delete_character_skills(&self, user_id: u64) -> CarrionResult<()> {
        let _record: Vec<Record> = DB.delete(format!("{}", user_id)).await?;
        info_with_span();
        Ok(())
    }
}
