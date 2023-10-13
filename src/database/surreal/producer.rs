use crate::database::surreal::{CHARACTER_TABLE, DB, ITEM_TABLE, MOB_TABLE};
use crate::database::Producer;
use crate::enemies::Mob;

use crate::player::{Character, SkillSet};
use crate::{CarrionResult, MobQueue, Record};

use serenity::async_trait;

use tracing::{debug, info};

pub struct SurrealProducer {}

#[async_trait]
impl Producer for SurrealProducer {
    async fn create_character(&self, content: Character) -> CarrionResult<()> {
        let user_id = content.user_id;
        debug!("Producer: {:?}", content);
        let _record: Option<Record> = DB
            .create((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;

        _ = self.create_user_items(user_id).await;
        Ok(())
    }

    async fn create_or_update_character(&self, content: Character) -> CarrionResult<()> {
        debug!("Updating Character for next run: {:?}", content);
        let _record: Option<Record> = DB
            .update((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;

        Ok(())
    }

    async fn delete_character(&self, user_id: u64) -> CarrionResult<()> {
        let record: Option<Record> = DB.delete((CHARACTER_TABLE, user_id)).await?;
        debug!("Deleted Character for: {:?}", record);
        Ok(())
    }

    async fn create_or_update_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        debug!("create_or_update_skill Character skill {:?}", content);
        let key = (format!("{}", user_id), content.skill() as u64);
        let _skill: Option<Record> = DB.update(key).content(content).await?;
        Ok(())
    }

    async fn set_current_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        debug!("set_current_skill Character skill {:?}", content);
        let key = (format!("{}", user_id), 999);
        let _skill: Option<Record> = DB.update(key).content(content).await?;
        Ok(())
    }
    async fn store_mob_queue(&self, character: &Character, enemies: Vec<Mob>) -> CarrionResult<()> {
        info!("Storing Enemies: {:?}", enemies);

        let _record: Option<Record> = DB
            .update((MOB_TABLE, character.user_id))
            .content(MobQueue { mobs: enemies })
            .await?;
        Ok(())
    }

    async fn delete_mob_queue(&self, user_id: u64) -> CarrionResult<()> {
        let record: Option<Record> = DB.delete((MOB_TABLE, user_id)).await?;
        debug!("Deleted Mob Queue for: {:?}", record);
        Ok(())
    }

    async fn store_user_items(
        &self,
        content: crate::items::Items,
        user_id: u64,
    ) -> CarrionResult<()> {
        debug!("Storing Items: {:?}", content);
        let _record: Option<Record> = DB.update((ITEM_TABLE, user_id)).content(content).await?;
        Ok(())
    }

    async fn delete_user_items(&self, user_id: u64) -> CarrionResult<()> {
        let record: Option<Record> = DB.delete((ITEM_TABLE, user_id)).await?;
        debug!("Deleted Items for: {:?}", record);
        Ok(())
    }

    async fn create_user_items(&self, user_id: u64) -> CarrionResult<()> {
        let items = crate::items::Items::default();
        let _: Option<Record> = DB.update((ITEM_TABLE, user_id)).content(items).await?;
        Ok(())
    }

    async fn delete_character_skills(&self, user_id: u64) -> CarrionResult<()> {
        let record: Vec<Record> = DB.delete(format!("{}", user_id)).await?;
        debug!("Deleted Skills for: {:?}", record);
        Ok(())
    }
}
