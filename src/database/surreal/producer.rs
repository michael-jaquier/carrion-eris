use crate::database::surreal::{CHARACTER_TABLE, DB, ENEMY_TABLE, ITEM_TABLE, MOB_TABLE};
use crate::enemies::{Enemy, Mob};
use crate::player::{Character, SkillSet};
use crate::{CarrionResult, MobQueue, Record};

use surrealdb::opt::PatchOp;
use surrealdb::sql::Thing;

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

    pub async fn patch_character(content: Character) -> CarrionResult<Option<Record>> {
        let patch_hp = PatchOp::replace("/hp", content.hp);
        let patch_level = PatchOp::replace("/level", content.level);
        let patch_experience = PatchOp::replace("/experience", content.experience);
        let patch_equipment = PatchOp::replace("/equipment", content.equipment);
        let patch_traits = PatchOp::replace("/traits", content.traits);
        let patch_available_traits =
            PatchOp::replace("/available_traits", content.available_traits);
        Ok(DB
            .update((CHARACTER_TABLE, content.user_id))
            .patch(patch_hp)
            .patch(patch_level)
            .patch(patch_experience)
            .patch(patch_equipment)
            .patch(patch_traits)
            .patch(patch_available_traits)
            .await?)
    }

    pub async fn store_active_enemy(
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

    pub async fn store_mob_queue(
        character: &Character,
        enemies: Vec<Mob>,
    ) -> CarrionResult<Option<Record>> {
        info!("Storing Enemies: {:?}", enemies);

        let record = DB
            .update((MOB_TABLE, character.user_id))
            .content(MobQueue { mobs: enemies })
            .await?;
        Ok(record)
    }

    pub async fn patch_mob_queue(
        character: &Character,
        enemies: Vec<Mob>,
    ) -> CarrionResult<Option<Record>> {
        info!("Patching Enemies: {:?}", enemies);

        let patch = PatchOp::add("/mobs", enemies);
        let record = DB
            .update((MOB_TABLE, character.user_id))
            .patch(patch)
            .await?;
        Ok(record)
    }

    pub async fn delete_mob_queue(user_id: u64) -> CarrionResult<Option<Record>> {
        let record = DB.delete((MOB_TABLE, user_id)).await?;
        debug!("Deleted Mob Queue for: {:?}", record);
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

    pub async fn delete_user_items(user_id: u64) -> CarrionResult<Option<Record>> {
        let record = DB.delete((ITEM_TABLE, user_id)).await?;
        debug!("Deleted Items for: {:?}", record);
        Ok(record)
    }

    pub async fn create_user_items(user_id: Thing) -> CarrionResult<Option<Record>> {
        let items = crate::items::Items::default();
        Ok(DB.update((ITEM_TABLE, user_id.id)).content(items).await?)
    }

    pub async fn patch_user_gold(
        gold: u64,
        user_id: u64,
        negative: bool,
    ) -> CarrionResult<Option<Record>> {
        debug!("Patching Gold: {:?}", gold);
        let now = tokio::time::Instant::now();
        if gold == 0 {
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
            old_gold.saturating_sub(gold)
        } else {
            old_gold + gold
        };
        let patch = PatchOp::replace("/gold", gold);
        let record = DB.update((ITEM_TABLE, user_id)).patch(patch).await?;
        debug!("Patched Gold: {:?}", record);
        debug!("Patched Gold: {:?}", now.elapsed());
        Ok(record)
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;

    use super::*;
    use crate::database::surreal::consumer::SurrealConsumer;
    use crate::database::surreal::SurrealDB;

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
}
