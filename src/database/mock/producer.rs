use async_trait::async_trait;

use tracing::instrument;

use crate::database::Producer;
use crate::database::{CarrionResult, Character, Items, Mob, SkillSet};

use super::MockDatabase;

pub struct MockProducer {}

#[async_trait]
impl Producer for MockProducer {
    #[instrument(skip(self, content),  target = "database_producer", fields(user_id = %content.user_id))]
    async fn create_character(&self, content: Character) -> CarrionResult<()> {
        MockDatabase::get()
            .characters
            .insert(content.user_id, content);
        Ok(())
    }

    #[instrument(skip(self, content),  target = "database_producer", fields(user_id = %content.user_id))]
    async fn create_or_update_character(&self, content: Character) -> CarrionResult<()> {
        MockDatabase::get()
            .characters
            .insert(content.user_id, content);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn delete_character(&self, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().characters.remove(&user_id);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn create_or_update_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get()
            .skills
            .insert((user_id, content.skill() as u64), content);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn set_current_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().current_skill.insert(user_id, content);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn delete_character_skills(&self, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().skills.retain(|k, _| k.0 != user_id);
        Ok(())
    }

    #[instrument(skip(self, character),  target = "database_producer", fields(user_id = %character.user_id))]
    async fn store_mob_queue(&self, character: &Character, enemies: Vec<Mob>) -> CarrionResult<()> {
        MockDatabase::get().mobs.insert(character.user_id, enemies);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn delete_mob_queue(&self, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().mobs.remove(&user_id);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn store_user_items(&self, content: Items, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().items.insert(user_id, content);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn delete_user_items(&self, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().items.remove(&user_id);
        Ok(())
    }

    #[instrument(skip(self), target = "database_producer")]
    async fn create_user_items(&self, user_id: u64) -> CarrionResult<()> {
        MockDatabase::get().items.insert(user_id, Items::default());
        Ok(())
    }
}
