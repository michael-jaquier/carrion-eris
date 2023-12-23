use crate::{
    character::Character,
    enemy::{Enemy, Mob},
    BattleInfo,
};
use std::{cell::RefCell, fmt::Debug};

use rand::random;
use tracing::debug;

use carrion_patterns::fsm::{FsmEnum, GameError, PublicGameError, Response, Stateful};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum GameState {
    Waiting,
    Battle,
    Null,
    Utility,
}

pub struct Waiting {}
pub struct Initiate {}
pub struct Battle {}
pub struct Utility {}
pub struct Null {}

impl Utility {
    pub fn new() -> Self {
        Self {}
    }
}

impl Null {
    pub fn new() -> Self {
        Self {}
    }
}
impl Waiting {
    pub fn new() -> Self {
        Self {}
    }
}

impl Initiate {
    pub fn new() -> Self {
        Self {}
    }
}

impl Battle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Stateful<GameState, Context, Event> for Utility {
    fn on_enter(&mut self, _context: &mut Context) -> Response<GameState> {
        debug!("Entering Utility state");
        Response::Ignore
    }

    fn on_event(&mut self, event: &Event, context: &mut Context) -> Response<GameState> {
        debug!("Utility state received event: {:?}", event);
        match event {
            Event::Heal => {
                let character_mut = &mut context.character_mut();
                let character = character_mut.as_mut().unwrap();
                character.hp = character.max_hp as i32;
                Response::Transition(GameState::Waiting)
            }
            _ => Response::Ignore,
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        debug!("Exiting Utility state");
    }
}

impl Stateful<GameState, Context, Event> for Null {
    fn on_enter(&mut self, _context: &mut Context) -> Response<GameState> {
        debug!("Entering Null state");
        Response::Ignore
    }

    fn on_event(&mut self, event: &Event, context: &mut Context) -> Response<GameState> {
        debug!("Null state received event: {:?}", event);
        match event {
            Event::Create(c) => {
                context.set_character(c.clone());
                Response::Transition(GameState::Waiting)
            }
            _ => Response::Ignore,
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        debug!("Exiting Null state");
    }
}

impl Stateful<GameState, Context, Event> for Waiting {
    fn on_enter(&mut self, _context: &mut Context) -> Response<GameState> {
        debug!("Entering Waiting state");
        Response::Ignore
    }

    fn on_event(&mut self, event: &Event, context: &mut Context) -> Response<GameState> {
        debug!("Waiting state received event: {:?}", event);
        match event {
            Event::GenereateEnemy => {
                if context.get_enemy().is_ok() {
                    return Response::Ignore;
                }

                let mob: Mob = random();
                let enemy = mob.generate(context.get_character().unwrap().level);
                context.set_enemy(enemy);
                debug!("Generating enemy");
                Response::Transition(GameState::Battle)
            }
            _ => Response::Ignore,
        }
    }

    fn on_exit(&mut self, _context: &mut Context) {
        debug!("Exiting Waiting state");
    }
}

impl Stateful<GameState, Context, Event> for Battle {
    fn on_enter(&mut self, context: &mut Context) -> Response<GameState> {
        if !context.character_is_set() {
            return Response::Transition(GameState::Null);
        }
        if context.character_ref().as_ref().unwrap().hp <= 0 {
            return Response::Transition(GameState::Utility);
        }
        if !context.enemy_is_set() {
            return Response::Transition(GameState::Waiting);
        }

        context.battle_init();

        debug!("Entering Battle state");
        Response::Ignore
    }

    fn on_event(&mut self, event: &Event, context: &mut Context) -> Response<GameState> {
        debug!("Battle state received event: {:?}", event);
        match event {
            Event::Attack => {
                context.character_ref().as_ref().unwrap().player_attack(
                    &context.enemy_ref().as_ref().unwrap(),
                    &mut context.battle_mut(),
                );

                Response::Transition(GameState::Battle)
            }
            Event::Defend => {
                context.character_ref().as_ref().unwrap().enemy_attack(
                    &context.enemy_ref().as_ref().unwrap(),
                    &mut context.battle_mut(),
                );

                Response::Transition(GameState::Battle)
            }
            _ => Response::Ignore,
        }
    }

    fn on_exit(&mut self, context: &mut Context) {
        context
            .character_mut()
            .as_mut()
            .unwrap()
            .apply_battle_info(&context.get_battle());
        context
            .enemy_mut()
            .as_mut()
            .unwrap()
            .apply_battle_info(&context.get_battle());
        context.take_dead_enemy();
        debug!("Exiting Battle state");
    }
}

#[derive(Debug)]
pub enum Event {
    Create(Character),
    GenereateEnemy,
    Join,
    Start,
    Attack,
    Defend,
    End,
    Heal,
}
#[derive(Debug)]
pub struct Context {
    pub character: RefCell<Option<Character>>,
    pub enemy: RefCell<Option<Enemy>>,
    pub battle: RefCell<BattleInfo>,
}

impl Context {
    pub fn null() -> Self {
        Self {
            character: RefCell::new(None),
            enemy: RefCell::new(None),
            battle: RefCell::new(Default::default()),
        }
    }
    pub fn set_character(&self, character: Character) {
        self.character.swap(&RefCell::new(Some(character)));
    }
    pub fn set_enemy(&self, enemy: Enemy) {
        self.enemy.swap(&RefCell::new(Some(enemy)));
    }
    pub fn set_battle(&self, battle: BattleInfo) {
        self.battle.swap(&RefCell::new(battle));
    }

    pub fn battle_init(&self) {
        let battle_info = BattleInfo::begin(
            self.character_ref().as_ref().unwrap(),
            self.enemy_ref().as_ref().unwrap(),
        );
        self.set_battle(battle_info);
    }
    pub fn get_battle(&self) -> std::cell::Ref<'_, BattleInfo> {
        self.battle.borrow()
    }

    pub fn battle_mut(&self) -> std::cell::RefMut<'_, BattleInfo> {
        self.battle.borrow_mut()
    }
    pub fn battle_take(&self) -> BattleInfo {
        self.battle.take()
    }
    pub fn enemy_ref(&self) -> std::cell::Ref<'_, Option<Enemy>> {
        self.enemy.borrow()
    }
    pub fn character_ref(&self) -> std::cell::Ref<'_, Option<Character>> {
        self.character.borrow()
    }
    pub fn character_mut(&self) -> std::cell::RefMut<'_, Option<Character>> {
        self.character.borrow_mut()
    }
    pub fn enemy_mut(&self) -> std::cell::RefMut<'_, Option<Enemy>> {
        self.enemy.borrow_mut()
    }
    pub fn get_enemy(&self) -> Result<Enemy, PublicGameError> {
        let enemy = self.enemy_ref();
        enemy
            .as_ref()
            .ok_or(PublicGameError::new(GameError::NoEnemy.into()))
            .map(|e| e.clone())
    }

    pub fn get_character(&self) -> Result<Character, PublicGameError> {
        let character = self.character_ref();
        character
            .as_ref()
            .ok_or(PublicGameError::new(GameError::NoCharacter.into()))
            .map(|c| c.clone())
    }

    pub fn character_is_set(&self) -> bool {
        self.character_ref().is_some()
    }
    pub fn enemy_is_set(&self) -> bool {
        self.enemy_ref().is_some()
    }
    pub fn take_dead_enemy(&self) -> Option<Enemy> {
        let enemy_alive = self
            .enemy
            .borrow()
            .as_ref()
            .and_then(|e| Some(e.alive()))
            .unwrap_or(false);
        if enemy_alive {
            None
        } else {
            self.enemy.take()
        }
    }
}

impl FsmEnum<GameState, Context, Event> for GameState {
    fn create(value: &GameState) -> Box<dyn Stateful<GameState, Context, Event> + Send> {
        match value {
            GameState::Waiting => Box::new(Waiting::new()),
            GameState::Battle => Box::new(Battle::new()),
            GameState::Null => Box::new(Null::new()),
            GameState::Utility => Box::new(Utility::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use carrion_patterns::fsm::StateMachine;

    use super::*;
    use crate::{character::Character, class::Classes};
    #[test]
    fn initiate_state_machine() {
        let character = Character::new("Test".to_string(), 1, Classes::Paladin);
        let mut state_machine: StateMachine<GameState, Context, Event> =
            StateMachine::new(Context::null());

        let _ = state_machine.init(GameState::Null);
        assert!(state_machine.current_context().get_character().is_err());
        let events = vec![
            Event::Create(character),
            Event::GenereateEnemy,
            Event::Attack,
            Event::Defend,
            Event::Defend,
            Event::Defend,
            Event::Defend,
            Event::Defend,
            Event::Defend,
            Event::Defend,
            Event::Defend,
        ];
        for e in events.into_iter() {
            let ee = state_machine.event(&e);
            assert!(ee.is_ok());
            assert!(state_machine.current_context().get_character().is_ok());
            println!("Current state machine {:?}", state_machine.current_state());
            println!(
                "{:?}",
                state_machine
                    .current_context()
                    .get_character()
                    .as_ref()
                    .unwrap()
                    .hp
            );
        }
    }
}
