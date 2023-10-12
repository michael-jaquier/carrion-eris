use crate::game::mutations::Mutations;
use crate::game_loop::{BUFFER, GAME};

#[derive(Debug, Clone, Default)]
pub struct Buffer {
    pub mutations: Vec<Mutations>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { mutations: vec![] }
    }

    pub fn add(&mut self, mutation: Mutations) {
        self.mutations.push(mutation);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Mutations> {
        self.mutations.iter()
    }

    pub fn clear(&mut self) {
        self.mutations.clear();
    }
    pub async fn mutations() {
        {
            if !BUFFER.read().await.mutations.is_empty() {
                let mut write_game = GAME.write().await;
                write_game.apply_mutations().await;
                BUFFER.write().await.clear();
            }
        }
    }
}
