use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE};
use crate::enemies::Enemy;
use crate::player::Character;
use crate::{CarrionError, CarrionResult, Record};
use tracing::{debug, error, info};
use tracing_subscriber::fmt::init;

pub struct SurrealProducer {}

impl SurrealProducer {
    pub async fn create_character(content: Character) -> CarrionResult<Option<Record>> {
        info!("Producer: {:?}", content);
        let record = DB
            .create((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;
        Ok(record)
    }

    pub async fn delete_character(user_id: u64) -> CarrionResult<Option<Record>> {
        let record = DB.delete((CHARACTER_TABLE, user_id)).await?;
        info!("Deleted Character for: {:?}", record);
        Ok(record)
    }
    pub async fn create_or_update_character(content: Character) -> CarrionResult<(Option<Record>)> {
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
