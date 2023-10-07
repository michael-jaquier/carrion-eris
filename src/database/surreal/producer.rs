use crate::database::surreal::{CHARACTER_TABLE, COMBAT_TABLE, DB, ENEMY_TABLE, ITEM_TABLE};
use crate::enemies::Enemy;
use crate::player::{Character, SkillSet};
use crate::{CarrionResult, Record};

use surrealdb::opt::PatchOp;
use surrealdb::sql::Thing;
use surrealdb::Response;

use crate::database::surreal::consumer::SurrealConsumer;
use tracing::{debug, info};

pub struct SurrealProducer {}

impl SurrealProducer {
    pub async fn create_character(content: Character) -> CarrionResult<Option<Record>> {
        debug!("Producer: {:?}", content);
        let record: Option<Record> = DB
            .create((CHARACTER_TABLE, content.user_id))
            .content(content)
            .await?;
        let record_id = record.clone().expect("Expected record").id;
        _ = SurrealProducer::create_user_items(record_id).await;
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

    pub async fn drop_character_skills(user_id: u64) -> CarrionResult<Vec<Record>> {
        let record: Vec<Record> = DB.delete(format!("{}", user_id)).await?;
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

    pub async fn store_user_items(
        content: crate::items::Items,
        user_id: u64,
    ) -> CarrionResult<Option<Record>> {
        debug!("Storing Items: {:?}", content);
        let record = DB.update((ITEM_TABLE, user_id)).content(content).await?;
        Ok(record)
    }

    pub async fn create_user_items(user_id: Thing) -> CarrionResult<Option<Record>> {
        let items = crate::items::Items::default();
        Ok(DB.update((ITEM_TABLE, user_id.id)).content(items).await?)
    }

    pub async fn patch_user_gold(
        content: u64,
        user_id: u64,
        negative: bool,
    ) -> CarrionResult<Option<Record>> {
        debug!("Patching Gold: {:?}", content);
        let now = tokio::time::Instant::now();
        if content == 0 {
            return Ok(None);
        }
        let old_gold = match SurrealConsumer::get_items(user_id).await? {
            Some(items) => items.gold,
            None => {
                let items = crate::items::Items::default();
                SurrealProducer::store_user_items(items, user_id).await?;
                0
            }
        };
        let gold = if negative {
            old_gold.saturating_sub(content)
        } else {
            old_gold + content
        };
        let patch = PatchOp::replace("/gold", gold);
        let record = DB.update((ITEM_TABLE, user_id)).patch(patch).await?;
        debug!("Patched Gold: {:?}", record);
        debug!("Patched Gold: {:?}", now.elapsed());
        Ok(record)
    }

    pub async fn store_related_enemies(
        character: &Character,
        enemies: Vec<Enemy>,
    ) -> CarrionResult<Response> {
        // Create enemies
        let now = tokio::time::Instant::now();
        let mut create_enemies = "BEGIN TRANSACTION;".to_string();
        for enemy in enemies {
            let serialized = serde_json::to_string(&enemy).unwrap();
            create_enemies.push_str(&format!("CREATE {} CONTENT {}; ", ENEMY_TABLE, serialized));
        }
        create_enemies.push_str("COMMIT TRANSACTION;");
        let mut created_enemies = DB.query(create_enemies).await?;
        info!("Created Enemies: {:?}", now.elapsed());

        // Get Enemy Records
        let record_count = created_enemies.num_statements();
        let mut record_ids = vec![];
        for i in 0..record_count {
            let _take: Option<Record> = created_enemies.take(i).unwrap();
            let id = _take.unwrap().id;
            record_ids.push(id);
        }

        // Relate enemies to player
        let mut relate = "BEGIN TRANSACTION;".to_string();
        for record_id in record_ids {
            relate.push_str(&format!(
                "relate {}->{}->{}:{}; ",
                record_id, COMBAT_TABLE, CHARACTER_TABLE, character.user_id
            ));
        }
        relate.push_str(&format!(
            "delete {} where state != 'Alive' return none;",
            ENEMY_TABLE
        ));
        relate.push_str("COMMIT TRANSACTION;");
        let groups = DB.query(relate).await?;
        info!("Relate Enemies: {:?}", now.elapsed());
        Ok(groups)
    }

    pub async fn store_related_enemy(
        character: &Character,
        enemy: &Enemy,
        thing: Option<Thing>,
    ) -> CarrionResult<Response> {
        let record_id = match thing {
            Some(thing) => {
                let record: Option<Record> =
                    DB.update(thing.clone()).content(enemy.clone()).await?;
                debug_assert_eq!(record.unwrap().id, thing, "Failed to update enemy");
                thing
            }
            None => {
                let record: Vec<Record> = DB.create(ENEMY_TABLE).content(enemy.clone()).await?;
                record.first().unwrap().id.clone()
            }
        };
        let relate = format!(
            "relate {}->{}->{}:{}; delete {} where state != 'Alive' return none;",
            record_id, COMBAT_TABLE, CHARACTER_TABLE, character.user_id, ENEMY_TABLE
        );
        let groups = DB
            .query(relate)
            .await
            .expect("Failed to relate enemy to user");
        Ok(groups)
    }

    pub async fn delete_related(user_id: u64) -> CarrionResult<()> {
        let sql = format!(
            "select id from (select id, count(->{}->{}) as fight, array::pop(->{}->{}.user_id) as user_id from {}) where user_id={}",
            COMBAT_TABLE, CHARACTER_TABLE, COMBAT_TABLE,CHARACTER_TABLE,ENEMY_TABLE, user_id
        );
        let mut record: Response = DB.query(sql).await?;
        let enemy_records: Vec<Record> = record.take(0).unwrap();
        for record in enemy_records {
            let _: Option<Enemy> = DB.delete(record.id).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;

    use super::*;
    use crate::database::surreal::consumer::SurrealConsumer;
    use crate::database::surreal::SurrealDB;
    use crate::enemies::EnemyState;

    #[ignore]
    #[tokio::test]
    async fn test_gold_storage() {
        SurrealDB::connect("memory").await.unwrap();
        let gold = 100;
        let user_id = 123456789;
        let items = crate::items::Items::default();
        let _items_record = SurrealProducer::store_user_items(items, user_id)
            .await
            .unwrap();
        let _record = SurrealProducer::patch_user_gold(gold, user_id, false).await;
        let items_fetch = SurrealConsumer::get_items(user_id).await.unwrap();
        assert_eq!(items_fetch.unwrap().gold, gold);
        let more_gold = 200;
        let _record = SurrealProducer::patch_user_gold(more_gold, user_id, false).await;
        let items_fetch = SurrealConsumer::get_items(user_id).await.unwrap();
        assert_eq!(items_fetch.unwrap().gold, more_gold + gold);
        let negative_gold = 100;
        let _record = SurrealProducer::patch_user_gold(negative_gold, user_id, true).await;
        let items_fetch = SurrealConsumer::get_items(user_id).await.unwrap();
        assert_eq!(
            items_fetch.unwrap().gold,
            (more_gold + gold - negative_gold)
        );
    }

    #[ignore]
    #[tokio::test]
    async fn delete_related_enemies() {
        SurrealDB::connect("memory").await.unwrap();
        let user_id = 442792120336777217;
        let character = Character {
            user_id,
            ..Default::default()
        };
        let other_character = Character {
            user_id: 123456789,
            ..Default::default()
        };

        let mut enemy = Enemy::default();
        let mut other_enemy = Enemy::default();
        other_enemy.gold = 999;
        enemy.gold = 333;

        SurrealProducer::create_character(character.clone())
            .await
            .unwrap();

        SurrealProducer::create_character(other_character.clone())
            .await
            .unwrap();

        let _groups = SurrealProducer::store_related_enemy(&character, &enemy, None)
            .await
            .unwrap();

        let _groups = SurrealProducer::store_related_enemy(&other_character, &other_enemy, None)
            .await
            .unwrap();

        let enemy_records = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap();

        assert!(enemy_records.is_some());

        SurrealProducer::delete_related(user_id).await.unwrap();
        let enemy_records = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap();
        assert!(enemy_records.is_none());

        let other_enemy_records = SurrealConsumer::get_related_enemies(&other_character)
            .await
            .unwrap();
        assert!(other_enemy_records.is_some());
    }
    #[ignore]
    #[tokio::test]
    async fn store_enemy_relation_to_user() {
        SurrealDB::connect("memory").await.unwrap();
        let user_id = 442792120336777217;
        let character = Character {
            user_id,
            ..Default::default()
        };

        let mut enemy = Enemy::default();
        enemy.gold = 333;
        SurrealProducer::create_character(character.clone())
            .await
            .unwrap();
        let _groups = SurrealProducer::store_related_enemy(&character, &enemy, None)
            .await
            .unwrap();

        let enemy_records = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap();
        assert!(enemy_records.is_some());
        assert_eq!(enemy_records.unwrap().0.gold, 333);
        let mut new_enemy = Enemy::default();
        new_enemy.gold = 444;
        SurrealProducer::store_related_enemy(&character, &new_enemy, None)
            .await
            .unwrap();
        let enemy_records = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap();
        assert!(enemy_records.is_some());
    }
    #[ignore]
    #[tokio::test]
    async fn mutate_enemies() {
        SurrealDB::connect("memory").await.unwrap();
        let user_id = 442792120336777217;
        let character = Character {
            user_id,
            ..Default::default()
        };
        let mut enemy = Enemy::default();
        enemy.gold = 333;
        SurrealProducer::create_character(character.clone())
            .await
            .unwrap();
        let _groups = SurrealProducer::store_related_enemy(&character, &enemy, None)
            .await
            .unwrap();
        let (mut enemy, id) = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap()
            .unwrap()
            .clone();
        enemy.state = EnemyState::Dead;
        println!("enemy: {:?}", enemy);
        println!("id: {:?}", id);
        SurrealProducer::store_related_enemy(&character, &enemy, Some(id))
            .await
            .unwrap();
        let enemies = SurrealConsumer::get_related_enemies(&character)
            .await
            .unwrap();
        println!("enemies: {:?}", enemies);
        assert!(enemies.is_none());
    }
}
