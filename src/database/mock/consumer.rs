

use crate::{
    character::Character,
    database::{mock::MockDatabase, Consumer},
    enemy::{Enemy, Mob},
    item::Items,
    skill::SkillSet, CarrionResult,
};
use async_trait::async_trait;


use tracing::{instrument, Level};

pub struct MockConsumer {}

#[async_trait]
impl Consumer for MockConsumer {
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_character(&self, user_id: u64) -> CarrionResult<Option<Character>> {
        match MockDatabase::get().characters.get(&user_id) {
            Some(c) => Ok(Some(c.clone())),
            None => Ok(None),
        }
    }
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_all_characters(&self) -> CarrionResult<Vec<Character>> {
        Ok(MockDatabase::get()
            .characters
            .iter()
            .map(|c| c.clone())
            .collect())
    }
    #[instrument(skip(self,character), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_enemy(&self, character: &Character) -> CarrionResult<Option<Enemy>> {
        Ok(MockDatabase::get()
            .enemy
            .get(&character.user_id)
            .map(|e| e.clone()))
    }
    #[instrument(skip(self,character), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_related_mobs(&self, character: &Character) -> CarrionResult<Vec<Mob>> {
        match MockDatabase::get().mobs.get(&character.user_id) {
            Some(mobs) => Ok(mobs.clone()),
            None => Ok(vec![]),
        }
    }
    #[instrument(skip(self,character), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_skill(
        &self,
        character: &Character,
        skill_id: u64,
    ) -> CarrionResult<Option<SkillSet>> {
        let skill = MockDatabase::get()
            .skills
            .get(&(character.user_id, skill_id))
            .map(|s| s.clone());
        Ok(skill)
    }
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_current_skill(&self, user_id: u64) -> CarrionResult<Option<SkillSet>> {
        Ok(MockDatabase::get()
            .current_skill
            .get(&user_id)
            .map(|s| s.clone()))
    }
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_skill_id(&self, user_id: u64, skill_id: u64) -> CarrionResult<Option<SkillSet>> {
        Ok(MockDatabase::get()
            .skills
            .get(&(user_id, skill_id))
            .map(|s| s.clone()))
    }
    #[instrument(skip(self), target = "database_consumer", ret(level = Level::TRACE))]
    async fn get_items(&self, user_id: u64) -> CarrionResult<Option<Items>> {
        Ok(MockDatabase::get().items.get(&user_id).map(|i| i.clone()))
    }
}
