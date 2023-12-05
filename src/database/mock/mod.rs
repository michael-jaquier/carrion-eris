use crate::{
    character::Character,
    enemy::{Enemy, Mob},
    item::Items,
    skill::SkillSet,
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;
use serde::{ser::SerializeStruct, Deserialize};
use serde::{Serialize, Serializer};
pub mod consumer;
pub mod producer;

static MOCK_DB: OnceCell<MockDatabase> = OnceCell::new();
#[derive(Debug, Default)]
pub struct MockDatabase {
    pub characters: DashMap<u64, Character>,
    pub enemy: DashMap<u64, Enemy>,
    pub mobs: DashMap<u64, Vec<Mob>>,
    pub skills: DashMap<(u64, u64), SkillSet>,
    pub items: DashMap<u64, Items>,
    pub current_skill: DashMap<u64, SkillSet>,
}

impl<'de> Deserialize<'de> for MockDatabase {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MockDatabaseHelper {
            characters: Vec<(u64, Character)>,
            enemy: Vec<(u64, Enemy)>,
            mobs: Vec<(u64, Vec<Mob>)>,
            skills: Vec<((u64, u64), SkillSet)>,
            items: Vec<(u64, Items)>,
            current_skill: Vec<(u64, SkillSet)>,
        }

        let helper = MockDatabaseHelper::deserialize(deserializer)?;

        let db = MockDatabase::default();

        for (k, v) in helper.characters {
            db.characters.insert(k, v);
        }
        for (k, v) in helper.enemy {
            db.enemy.insert(k, v);
        }
        for (k, v) in helper.mobs {
            db.mobs.insert(k, v);
        }
        for (k, v) in helper.skills {
            db.skills.insert(k, v);
        }
        for (k, v) in helper.items {
            db.items.insert(k, v);
        }
        for (k, v) in helper.current_skill {
            db.current_skill.insert(k, v);
        }

        Ok(db)
    }
}

impl Serialize for MockDatabase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MockDatabase", 6)?;

        let character_entries: Vec<(u64, Character)> = self
            .characters
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        let enemy_entries: Vec<(u64, Enemy)> = self
            .enemy
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        let mob_entries: Vec<(u64, Vec<Mob>)> = self
            .mobs
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        let skill_entries: Vec<((u64, u64), SkillSet)> = self
            .skills
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        let item_entries: Vec<(u64, Items)> = self
            .items
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        let current_skill_entries: Vec<(u64, SkillSet)> = self
            .current_skill
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();

        state.serialize_field("characters", &character_entries)?;
        state.serialize_field("enemy", &enemy_entries)?;
        state.serialize_field("mobs", &mob_entries)?;
        state.serialize_field("skills", &skill_entries)?;
        state.serialize_field("items", &item_entries)?;
        state.serialize_field("current_skill", &current_skill_entries)?;

        state.end()
    }
}

impl MockDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get() -> &'static MockDatabase {
        MOCK_DB.get_or_init(|| {
            let db = Self::load_db().unwrap_or_default();
            db
        })
    }

    pub fn store_db() -> Result<(), Box<dyn std::error::Error>> {
        let db = Self::get();
        let db_serialized = serde_json::to_string(&db)?;
        std::fs::write("mock_db.json", db_serialized)?;
        Ok(())
    }

    pub fn load_db() -> Result<Self, Box<dyn std::error::Error>> {
        let db_serialized = std::fs::read_to_string("mock_db.json")?;
        let db_deserialized: MockDatabase = serde_json::from_str(&db_serialized)?;
        Ok(db_deserialized)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_mock_database() {
        use crate::database::mock::MockDatabase;
        MockDatabase::new();
        let db = MockDatabase::get();
        assert_eq!(db.characters.len(), 0);
        assert_eq!(db.enemy.len(), 0);
        assert_eq!(db.mobs.len(), 0);
        assert_eq!(db.skills.len(), 0);
        assert_eq!(db.items.len(), 0);
        assert_eq!(db.current_skill.len(), 0);
    }
    #[test]
    fn mock_database_serialize() {
        use crate::database::mock::MockDatabase;
        MockDatabase::new();
        MockDatabase::get().characters.insert(1, Default::default());
        MockDatabase::get().enemy.insert(1, Default::default());
        MockDatabase::get()
            .mobs
            .insert(1, vec![crate::enemy::Mob::Eldragor]);
        MockDatabase::get()
            .skills
            .insert((1, 1), Default::default());
        let db = MockDatabase::get();

        let db_serialized = serde_json::to_string(&db).unwrap();
        let db_deserialized: MockDatabase = serde_json::from_str(&db_serialized).unwrap();
        let db_mob = db_deserialized.mobs.get(&1).unwrap();
        assert_eq!(db_mob.len(), 1);
        assert_eq!(db_mob[0], crate::enemy::Mob::Eldragor);
    }
}
