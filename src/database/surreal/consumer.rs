use crate::{CarrionError, CarrionResult, Character};
use crate::database::surreal::{CHARACTER_TABLE, DB};

pub struct SurrealConsumer {}

impl SurrealConsumer {
    pub async fn get_character<F>(id: F) -> CarrionResult<Option<Character>>
    where
        F: Into<surrealdb::sql::Id>,
    {

        let record: Option<Character> = DB.select((CHARACTER_TABLE, id)).await?;
        Ok(record)
    }
    pub async fn get_all_characters() -> CarrionResult<Vec<Character>>{
        let records: Vec<Character> = DB.select(CHARACTER_TABLE).await?;
        Ok(records)
    }
}
