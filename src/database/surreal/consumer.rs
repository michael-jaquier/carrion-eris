use crate::database::surreal::{CHARACTER_TABLE, COMBAT_TABLE, DB, ENEMY_TABLE, ITEM_TABLE};
use crate::enemies::Enemy;
use crate::items::Items;
use crate::player::{Character, SkillSet};
use crate::{CarrionResult, Record};
use surrealdb::sql::Thing;
use surrealdb::Response;
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

    pub async fn get_related_enemies(
        character: &Character,
    ) -> CarrionResult<Option<(Enemy, Thing)>> {
        let sql = format!(
            "select id from (select id, count(->{}->{}) as fight, array::pop(->{}->{}.user_id) as user_id from {}) where user_id={} limit 1",
            COMBAT_TABLE, CHARACTER_TABLE, COMBAT_TABLE,CHARACTER_TABLE,ENEMY_TABLE, character.user_id
        );

        let mut record: Response = DB.query(sql).await?;
        let enemy_records: Option<Record> = record.take(0)?;
        if let Some(record) = enemy_records {
            let enemy: Option<Enemy> = DB.select(record.id.clone()).await?;
            let true_enemy_records = (enemy.expect("No matching record"), record.id.clone());
            Ok(Option::from(true_enemy_records))
        } else {
            Ok(None)
        }
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

    pub async fn get_items(user_id: u64) -> CarrionResult<Option<Items>> {
        let key = (ITEM_TABLE, user_id);
        let items: Option<Items> = DB.select(key).await?;
        debug!("get_items: {:?}", items);
        Ok(items)
    }
}
