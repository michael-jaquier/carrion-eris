use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE};
use crate::enemies::Enemy;
use crate::player::{Character, SkillSet};
use crate::{CarrionResult, Record};

use tracing::debug;

pub struct SurrealProducer {}

impl SurrealProducer {
    pub async fn create_character(content: Character) -> CarrionResult<Option<Record>> {
        debug!("Producer: {:?}", content);
        let record = DB
            .create((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;
        Ok(record)
    }

    pub async fn create_or_update_skill(
        content: SkillSet,
        character: &Character,
    ) -> CarrionResult<Option<Record>> {
        debug!("create_or_update_skill Character skill {:?}", content);
        let key = (format!("{}", character.user_id), content.skill() as u64);
        let skill = DB.update(key).content(content).await?;
        Ok(skill)
    }

    pub async fn set_current_skill(
        content: SkillSet,
        character: &Character,
    ) -> CarrionResult<Option<Record>> {
        debug!("set_current_skill Character skill {:?}", content);
        let key = (format!("{}", character.user_id), 999);
        let skill = DB.update(key).content(content).await?;
        Ok(skill)
    }

    pub async fn set_current_skill_id(content: SkillSet, user_id: u64) -> CarrionResult<SkillSet> {
        debug!("set_current_skill Character skill {:?}", content);
        let key = (format!("{}", user_id), 999);
        let record: Option<Record> = DB.update(key).content(content.clone()).await?;
        debug!("set_current_skill_id: {:?}", record);
        Ok(content)
    }
    pub async fn delete_character(user_id: u64) -> CarrionResult<Option<Record>> {
        let record = DB.delete((CHARACTER_TABLE, user_id)).await?;
        debug!("Deleted Character for: {:?}", record);
        Ok(record)
    }

    pub async fn drop_character_skills(user_id: u64) -> CarrionResult<Option<Record>> {
        let record = DB.delete(format!("{}", user_id)).await?;
        debug!("Deleted Skills for: {:?}", record);
        Ok(record)
    }
    pub async fn create_or_update_character(content: Character) -> CarrionResult<Option<Record>> {
        debug!("Updating Character for next run: {:?}", content);
        let record = DB
            .update((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;

        Ok(record)
    }

    pub async fn store_enemy(
        content: Enemy,
        character: &Character,
    ) -> CarrionResult<Option<Record>> {
        debug!("Storing Enemy: {:?}", content);
        let record = DB
            .update((ENEMY_TABLE, character.user_id))
            .content(content)
            .await?;
        Ok(record)
    }

    pub async fn delete_enemy(character: &Character) -> CarrionResult<Option<Record>> {
        let record = DB.delete((ENEMY_TABLE, character.user_id)).await?;
        debug!("Deleted Enemy for: {:?}", record);
        Ok(record)
    }
}
