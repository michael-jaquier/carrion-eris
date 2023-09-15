pub mod consumer;
pub mod producer;


use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

use std::time::{SystemTime, UNIX_EPOCH};

use once_cell::sync::Lazy;

use surrealdb::engine::any::Any;

use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::info;
use crate::{CarrionError, CarrionResult};

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

pub static CHARACTER_TABLE: &str = "characters";


pub struct SurrealDB {}
impl SurrealDB {
    pub async fn connect(address: &str) -> CarrionResult<()> {
        DB.connect(address).await?;
        DB.use_ns("carrion").await?;
        DB.use_db("eris").await?;
        Ok(())

    }

    pub async fn authenticate(username: &str, password: &str) -> CarrionResult<()> {
        DB.signin(Root { username, password }).await?;
        Ok(())

    }
}