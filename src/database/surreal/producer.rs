use tracing::{error, info};
use tracing_subscriber::fmt::init;
use crate::{CarrionError, CarrionResult, Character, Record};
use crate::database::surreal::{CHARACTER_TABLE, DB};


pub struct SurrealProducer {}

impl SurrealProducer {
    pub async fn create_character(content: Character) -> CarrionResult<Option<Record>> {
        info!("Producer: {:?}", content);
        let record = DB.create((CHARACTER_TABLE, content.user_id)).content(content).await?;
        Ok(record)
    }
    pub async fn create_or_update_character(content: Character) -> CarrionResult<(Option<Record>)> {
        let record = DB.update((CHARACTER_TABLE, content.user_id)).content(content).await?;
        Ok(record)
    }

}
