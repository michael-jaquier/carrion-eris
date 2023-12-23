use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid class: {0}")]
    InvalidClass(String),
    #[error("State not found: {0}")]
    StateNotFound(String),
    #[error("State machine not initialized")]
    StateMachineNotInitialized,
    #[error("State machine already initialized")]
    StateMachineAlreadyInitialized,
    #[error("No character found")]
    NoCharacter,
    #[error("No Enemy found")]
    NoEnemy,
    #[error("No BattleInfo found")]
    NoBattleInfo,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PublicGameError(#[from] GameError);

impl PublicGameError {
    pub fn new(e: GameError) -> Self {
        Self(e)
    }
}
pub enum Response<S> {
    Transition(S),
    Ignore,
}
pub trait Stateful<S, CTX, E>
where
    S: Hash + PartialEq + Eq + Clone,
    E: Debug,
{
    fn on_enter(&mut self, context: &mut CTX) -> Response<S>;
    fn on_event(&mut self, event: &E, context: &mut CTX) -> Response<S>;
    fn on_exit(&mut self, context: &mut CTX);
}

pub trait FsmEnum<S, CTX, E> {
    fn create(value: &S) -> Box<dyn Stateful<S, CTX, E> + Send>;
}

pub trait EventHandler<S, CTX, E>
where
    S: Hash + PartialEq + Eq + Clone,
    E: Debug,
{
    fn on_event(&self, event: &E, ctx: &mut CTX) -> Response<S>;
}

pub struct StateMachine<S, CTX, E>
where
    S: Hash + PartialEq + Eq + Clone + FsmEnum<S, CTX, E>,
    E: Debug,
{
    current_state: Option<S>,
    states: HashMap<S, Box<dyn Stateful<S, CTX, E> + Send>>,
    context: CTX,
    handler: Option<Box<dyn EventHandler<S, CTX, E> + Send>>,
}

impl<S, CTX, E> StateMachine<S, CTX, E>
where
    S: Hash + PartialEq + Eq + Clone + FsmEnum<S, CTX, E>,
    E: Debug,
    CTX: Debug,
{
    pub fn new(ctx: CTX) -> Self {
        Self {
            current_state: None,
            states: HashMap::new(),
            context: ctx,
            handler: None,
        }
    }

    pub fn current_state(&self) -> Option<&S> {
        self.current_state.as_ref()
    }

    pub fn get_state(&self) -> S {
        self.current_state.as_ref().unwrap().clone()
    }

    pub fn current_context(&mut self) -> &CTX {
        &self.context
    }

    pub fn add_state(&mut self, state: S) {
        self.states
            .entry(state.clone())
            .or_insert_with(|| S::create(&state));
    }
    pub fn set_handler<H>(&mut self, handler: Option<Box<dyn EventHandler<S, CTX, E> + Send>>) {
        self.handler = handler;
    }

    pub fn init(&mut self, state: S) -> Result<(), PublicGameError> {
        if self.current_state.is_none() {
            self.current_state = Some(state.clone());
            loop {
                let current_state = self.current_state.as_ref().ok_or_else(|| {
                    Into::<PublicGameError>::into(GameError::StateMachineNotInitialized)
                })?;
                let state = if let Some(existing) = self.states.get_mut(current_state) {
                    existing
                } else {
                    let new_state = S::create(current_state);
                    self.states
                        .entry(current_state.clone())
                        .or_insert(new_state)
                };

                match state.on_enter(&mut self.context) {
                    Response::Transition(s) => self.current_state = Some(s),
                    Response::Ignore => break,
                }
            }
        }

        Ok(())
    }

    pub fn event(&mut self, event: &E) -> Result<(), PublicGameError> {
        let current_state = self
            .current_state
            .as_ref()
            .ok_or_else(|| Into::<PublicGameError>::into(GameError::StateMachineNotInitialized))?;

        if let Some(global) = &mut self.handler {
            match global.on_event(event, &mut self.context) {
                Response::Transition(s) => {
                    if s != *current_state {
                        return self.transition(s);
                    }
                }
                Response::Ignore => {}
            }
        }

        let state = if let Some(existing) = self.states.get_mut(current_state) {
            existing
        } else {
            let new_state = S::create(current_state);
            self.states
                .entry(current_state.clone())
                .or_insert(new_state)
        };

        match state.on_event(event, &mut self.context) {
            Response::Transition(s) => {
                return self.transition(s);
            }
            Response::Ignore => {}
        };

        Ok(())
    }

    pub fn transition(&mut self, new_state: S) -> Result<(), PublicGameError> {
        let current_state = self
            .current_state
            .as_ref()
            .ok_or_else(|| Into::<PublicGameError>::into(GameError::StateMachineNotInitialized))?;
        let state = if let Some(existing) = self.states.get_mut(current_state) {
            existing
        } else {
            let new_state = S::create(current_state);
            self.states
                .entry(current_state.clone())
                .or_insert(new_state)
        };
        state.on_exit(&mut self.context);
        self.current_state = Some(new_state.clone());
        loop {
            let current_state = self.current_state.as_ref().ok_or_else(|| {
                Into::<PublicGameError>::into(GameError::StateMachineNotInitialized)
            })?;
            let state = if let Some(existing) = self.states.get_mut(current_state) {
                existing
            } else {
                let new_state = S::create(current_state);
                self.states
                    .entry(current_state.clone())
                    .or_insert(new_state)
            };

            match state.on_enter(&mut self.context) {
                Response::Transition(s) => {
                    if s == *self.current_state.as_ref().unwrap() {
                        break;
                    } else {
                        self.current_state = Some(s);
                    }
                }
                Response::Ignore => break,
            }
        }

        Ok(())
    }
}
