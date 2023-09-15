pub mod database;
pub mod commands;
pub mod battle;
mod enemies;

use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};
// Custom user data passed to all command functions
pub struct State {
}
type CarrionResult<T> = Result<T, CarrionError>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

use std::fmt::Display;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarrionError {
    #[error("Surreal error: {0}")]
    SurrealDBError(#[from] surrealdb::error::Db),
    #[error("Surreal error: {0}")]
    SurrealApiError(#[from] surrealdb::error::Api),
    #[error("Surreal error: {0}")]
    SurrealError(#[from] surrealdb::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Classes {
    Warrior,
}

impl Classes {
    pub fn action(&self) -> Actions {
        match self {
            Classes::Warrior => Actions::Slash
        }
    }

}
impl TryFrom<String> for Classes {
    type Error = CarrionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "warrior" => Ok(Classes::Warrior),
            _ => Err(CarrionError::ParseError("Unable to parse class".to_string())),
        }
    }
}

impl TryFrom<&str> for Classes {
    type Error = CarrionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "warrior" => Ok(Classes::Warrior),
            _ => Err(CarrionError::ParseError("Unable to parse class".to_string())),
        }
    }
}
impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Classes::Warrior => write!(f, "Warrior"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Actions {
    Slash,
}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Character {
    level: u32,
    name: String,
    user_id: u64,
    class: Classes,
}

impl Character {
    pub fn new(name: String, user_id: u64, class: Classes) -> Self {
        Self {
            level: 1,
            name,
            user_id,
            class,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}\n level: {}\n class {}", self.name, self.level,self.class)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}