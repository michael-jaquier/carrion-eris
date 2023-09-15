use tracing::info;
use tracing_subscriber::fmt::format;
use crate::{Character, Classes};
use crate::database::surreal::consumer::SurrealConsumer;
use crate::enemies::{Enemy, Mobs};


pub fn battle(character: Character) -> String {
    info!("Battle!");
    info!("Character: {:?}", character);
    let mut enemy = Mobs::generate(&Mobs::Orc);
    let action = character.class.action();
    while enemy.alive() {
        enemy.act(&action);
    };
    format!("{} has defeated the {}", character.name, enemy.name())

}

pub async fn all_battle() -> Vec<String> {
    info!("All Battle!");
    let characters = SurrealConsumer::get_all_characters().await;
    let mut results = vec![];
    match characters {
        Ok(characters) => {
            for character in characters {
                let result = battle(character);
                results.push(result);
            }
        }
        Err(_) => {
            info!("Failed to get characters");
        }
    }
    results
}