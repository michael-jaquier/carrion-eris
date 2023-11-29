pub mod mock;
pub mod surreal;

use serenity::async_trait;

use crate::character::Character;
use crate::enemy::{Enemy, Mob};
use crate::item::Items;
use crate::skill::SkillSet;
use crate::CarrionResult;

#[async_trait]
pub trait Producer {
    async fn create_character(&self, content: Character) -> CarrionResult<()>;
    async fn create_or_update_character(&self, content: Character) -> CarrionResult<()>;
    async fn delete_character(&self, user_id: u64) -> CarrionResult<()>;

    async fn create_or_update_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()>;
    async fn set_current_skill(&self, content: SkillSet, user_id: u64) -> CarrionResult<()>;
    async fn delete_character_skills(&self, user_id: u64) -> CarrionResult<()>;

    async fn store_mob_queue(&self, character: &Character, enemies: Vec<Mob>) -> CarrionResult<()>;
    async fn delete_mob_queue(&self, user_id: u64) -> CarrionResult<()>;

    async fn store_user_items(&self, content: Items, user_id: u64) -> CarrionResult<()>;
    async fn delete_user_items(&self, user_id: u64) -> CarrionResult<()>;
    async fn create_user_items(&self, user_id: u64) -> CarrionResult<()>;
}

#[async_trait]
pub trait Consumer {
    async fn get_character(&self, user_id: u64) -> CarrionResult<Option<Character>>;
    async fn get_all_characters(&self) -> CarrionResult<Vec<Character>>;
    async fn get_enemy(&self, character: &Character) -> CarrionResult<Option<Enemy>>;
    async fn get_related_mobs(&self, character: &Character) -> CarrionResult<Vec<Mob>>;
    async fn get_skill(
        &self,
        character: &Character,
        skill_id: u64,
    ) -> CarrionResult<Option<SkillSet>>;
    async fn get_current_skill(&self, user_id: u64) -> CarrionResult<Option<SkillSet>>;
    async fn get_skill_id(&self, user_id: u64, skill_id: u64) -> CarrionResult<Option<SkillSet>>;
    async fn get_items(&self, user_id: u64) -> CarrionResult<Option<Items>>;
}

#[derive(Debug, Clone, Copy)]
pub enum Database {
    Surreal,
    Mock,
}

impl Database {
    pub fn get_producer(&self) -> Box<dyn Producer + Sync + Send> {
        match self {
            Database::Surreal => Box::new(surreal::producer::SurrealProducer {}),
            Database::Mock => Box::new(mock::producer::MockProducer {}),
        }
    }

    pub fn get_consumer(&self) -> Box<dyn Consumer + Sync + Send> {
        match self {
            Database::Surreal => Box::new(surreal::consumer::SurrealConsumer {}),
            Database::Mock => Box::new(mock::consumer::MockConsumer {}),
        }
    }
}
