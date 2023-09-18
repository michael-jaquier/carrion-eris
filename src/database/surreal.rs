pub mod consumer;
pub mod producer;

use once_cell::sync::Lazy;

use surrealdb::engine::any::Any;

use crate::CarrionResult;
use surrealdb::opt::auth::Root;

use surrealdb::Surreal;

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

pub static CHARACTER_TABLE: &str = "characters";
pub static ENEMY_TABLE: &str = "enemies";

pub struct SurrealDB {}
impl SurrealDB {
    pub async fn connect(address: &str) -> CarrionResult<()> {
        println!("Connecting to: {}", address);
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
