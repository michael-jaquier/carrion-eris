use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE};
use crate::enemies::Enemy;
use crate::player::{Character, SkillSet};
use crate::CarrionResult;
use tracing::debug;

pub struct SurrealConsumer {}

impl SurrealConsumer {
    pub async fn get_character<F>(id: F) -> CarrionResult<Option<Character>>
    where
        F: Into<surrealdb::sql::Id>,
    {
        let record: Option<Character> = DB.select((CHARACTER_TABLE, id)).await?;
        Ok(record)
    }
    pub async fn get_all_characters() -> CarrionResult<Vec<Character>> {
        let records: Vec<Character> = DB.select(CHARACTER_TABLE).await?;
        Ok(records)
    }

    pub async fn get_enemy(character: &Character) -> CarrionResult<Option<Enemy>> {
        let record: Option<Enemy> = DB.select((ENEMY_TABLE, character.user_id)).await?;
        Ok(record)
    }

    pub async fn get_skill(
        character: &Character,
        skill_id: u64,
    ) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", character.user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_skill: {:?}", skill);
        Ok(skill)
    }

    pub async fn get_current_skill(user_id: u64) -> CarrionResult<Option<SkillSet>> {
        let key = (format!("{}", user_id), 999);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_current_skill: {:?}", skill);
        Ok(skill)
    }

    pub async fn get_skill_id(user_id: u64, skill_id: u64) -> CarrionResult<Option<SkillSet>> {
        debug!("get_skill_id: {:?}, {:?}", user_id, skill_id);
        let key = (format!("{}", user_id), skill_id);
        let skill: Option<SkillSet> = DB.select(key).await?;
        debug!("get_skill_id_gotten: {:?}", skill);
        Ok(skill)
    }
}
