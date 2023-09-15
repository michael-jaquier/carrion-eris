use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use crate::Actions;


pub enum EnemyState {
    Dead,
    Alive,
}
pub enum EquipmentSlot {
    Head,
    Chest,
    Legs,
    Feet,
    Hands,
    Weapon,
    Shield,
    Ring,
    Amulet,
    Consumable,
    Misc,
}
pub struct Item {
    name: String,
    description: String,
    value: u32,
    rarity: u32,
    damage: u32,
    defense: u32,
    resistance: u32,
    slot: EquipmentSlot,
}

pub struct Enemy {
    name: String,
    level: u32,
    experience: u32,
    health: i32,
    attack: u32,
    defense: u32,
    speed: u32,
    magic: u32,
    resistance: u32,
    strength: u32,
    intelligence: u32,
    dexterity: u32,
    constitution: u32,
    wisdom: u32,
    charisma: u32,
    gold: u32,
    items: Vec<Item>,
    state: EnemyState,
}
impl Enemy {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn weak(mob: Mobs) -> Enemy {
        let mut rng = rand::thread_rng();
        Enemy {
            name: format!("{:?}", mob),
            level: 1,
            experience: rng.gen_range(1..10),
            health: rng.gen_range(1..10),
            attack: rng.gen_range(1..10),
            defense: rng.gen_range(1..10),
            speed: rng.gen_range(1..10),
            magic: rng.gen_range(1..10),
            resistance: rng.gen_range(1..10),
            strength: rng.gen_range(1..10),
            intelligence: rng.gen_range(1..10),
            dexterity: rng.gen_range(1..10),
            constitution: rng.gen_range(1..10),
            wisdom: rng.gen_range(1..10),
            charisma: rng.gen_range(1..10),
            gold: rng.gen_range(1..10),
            items: vec![],
            state: EnemyState::Alive,
        }
    }

    pub fn alive(&self) -> bool {
        match self.state {
            EnemyState::Alive => true,
            EnemyState::Dead => false,
        }
    }

    pub fn act(&mut self, action: &Actions)  {
        match action {
            Actions::Slash => {
                let damage = thread_rng().gen_range(1..3);
                println!("Slashed the {} for {} damage!", self.name, damage);
                self.health -= damage;
                if self.health <= 0 {
                    self.state = EnemyState::Dead;
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mobs {
    Orc,
}

impl Mobs {
    pub fn generate(&self) -> Enemy {
        match self {
            Mobs::Orc => Enemy::weak(Mobs::Orc),
        }
    }
}
