use crossterm::style::Color;

use crate::{character::Character, enemy::Enemy, ui::cli::TerminalMessages};

use super::game_loop::GameState;

#[derive(Clone, PartialEq, Debug)]
pub(crate) enum InputStates {
    Confirm(Box<GameState>),
    Name,
}
impl InputStates {
    pub fn command(&self, command: String, state: &mut GameState) -> TerminalMessages {
        let mut msg = Vec::new();
        match self {
            InputStates::Name => {
                state.name = Some(command.clone());
                let character = Character::new(command.clone(), 1, state.class.unwrap());
                state.character = Some(character);
                msg.push((format!("Welcome to Carrion-Eris {command}"), Color::Green));
                state.state = State::Null;
            }
            InputStates::Confirm(previous_date) => {
                if !command.to_lowercase().contains('y') {
                    *state = *previous_date.clone();
                    msg.push(("Change aborted".to_string(), Color::Red));
                    state.state = State::Null;
                } else {
                    for line in state.character.as_ref().unwrap().display_for_cli() {
                        msg.push((line, Color::Magenta));
                    }
                    state.state = State::Null;
                }
            }
        }

        msg
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) enum State {
    #[default]
    Null,
    Camping(u64),
    Fighting,
    Input(InputStates),
}

impl State {
    pub(crate) fn name() -> State {
        Self::Input(InputStates::Name)
    }
    pub(crate) fn confirm(state: GameState) -> State {
        Self::Input(InputStates::Confirm(Box::new(state)))
    }
}
