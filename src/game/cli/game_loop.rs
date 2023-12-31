use crate::{character::Character, class::Classes, ui::cli::GameClient};

use crossterm::style::Color;

use super::{
    commands::{battle_help, help, BATTLE_COMMANDS, COMMANDS},
    locations::Locations,
    states::State,
};

// <fight>
// <create> warrior
// <name> me
// <go> direction
// <equip> longsword
// <sell> blade
// <use> item
// <sneak>
// <cast> fireball

#[derive(Default)]
pub struct GameStates {
    current_state: GameState,
}

impl GameStates {
    pub fn update(&mut self, client: &mut GameClient) {
        self.current_state.update(client)
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn command(&mut self, command: String, client: &mut GameClient) {
        self.current_state.command(command, client);
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) struct GameState {
    pub(crate) class: Option<Classes>,
    pub(crate) name: Option<String>,
    pub(crate) character: Option<Character>,
    pub(crate) location: Locations,
    pub(crate) state: State,
}

impl GameState {
    pub fn update(&mut self, client: &mut GameClient) {
        match self.state {
            State::Null => {}
            State::Camping(_x) => {
                client.msg_send(super::commands::camp(self, &[]));
            }
            State::Fighting => {}
            State::Input(_) => {}
        }

        let _ = client.render(None);
        if let Some(character) = self.character.as_ref() {
            let mut string = format!(
                "Class: {} HP: {} Level: {}",
                character.class, character.hp, character.level
            );
            if character.available_traits > 0 {
                string = string.clone() + " !!! Avaliable Trait !!!";
            }
            let _ = client.status_bar_bottom(&string);
        }
    }
    pub(crate) fn get_character_ref_unchecked(&self) -> &Character {
        self.character.as_ref().unwrap()
    }
    pub(crate) fn get_character_mut_unchecked(&mut self) -> &mut Character {
        self.character.as_mut().unwrap()
    }

    fn command(&mut self, command: String, client: &mut GameClient) {
        match self.state.clone() {
            State::Null => {
                if let Some((inner_command, args)) = command
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>()
                    .split_first()
                {
                    let command_found = COMMANDS.iter().find(|m| m.command == *inner_command);

                    if let Some(m) = command_found {
                        client.msg_send(m.run(self, args));
                    } else {
                        let mut error_msg: crate::ui::cli::Messages =
                            vec![(format!("Command not recognized {command}"), Color::Red)].into();
                        let help_msg = help(self, &[]);
                        error_msg.send(help_msg);
                        client.msg_send(error_msg);
                    }
                }
            }

            State::Fighting => {
                if let Some((inner_command, args)) = command
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>()
                    .split_first()
                {
                    let command_found =
                        BATTLE_COMMANDS.iter().find(|m| m.command == *inner_command);

                    if let Some(m) = command_found {
                        client.msg_send(m.run(self, args));
                    } else {
                        let mut error_msg: crate::ui::cli::Messages =
                            vec![(format!("Command not recognized {command}"), Color::Red)].into();
                        let help_msg = battle_help(self, &[]);
                        error_msg.send(help_msg);
                        client.msg_send(error_msg);
                    }
                }
            }

            State::Input(input_state) => client.send(input_state.command(command.clone(), self)),
            State::Camping(_) => {}
        };
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::ui::cli::GameClient;

    fn setup() -> (GameClient, GameStates) {
        let mut client = GameClient::new();
        let mut state = GameStates::new();

        let command = "create warrior".to_string();
        assert!(state.current_state.class.is_none());
        state.command(command, &mut client);
        assert!(state.current_state.class.is_some());

        assert!(state.current_state.state == State::name());

        let command = "name a b c d".to_string();
        assert!(state.current_state.name.is_none());
        state.command(command, &mut client);
        assert!(state.current_state.name.is_some());

        (client, state)
    }

    #[test]
    fn character_swap() {
        let mut client = GameClient::new();
        let mut state = GameStates::new();

        let command = "create warrior".to_string();
        assert!(state.current_state.class.is_none());
        state.command(command, &mut client);
        assert!(state.current_state.class.is_some());

        assert!(state.current_state.state == State::name());

        let command = "name a b c d".to_string();
        assert!(state.current_state.name.is_none());
        state.command(command, &mut client);
        assert!(state.current_state.name.is_some());

        assert!(state.current_state.state == State::Null);

        let previous_state = state.current_state.clone();

        let command = "create wizard".to_string();
        assert!(state.current_state.class.is_some());
        state.command(command, &mut client);
        assert!(state.current_state.class.is_some());
        assert!(state.current_state.state == State::confirm(previous_state));

        let command = "yes".to_string();
        state.command(command, &mut client);
        assert!(state.current_state.class.unwrap() == Classes::Wizard);
        assert!(
            state.current_state.state == State::Null,
            "{:?}",
            state.current_state.state
        );
    }

    #[test]
    fn camping() {
        let (mut client, mut state) = setup();
        let command = "camp".to_string();
        state.command(command, &mut client);
        assert!(state.current_state.state == State::Camping(0));
        for _ in 0..10000 {
            state.update(&mut client);
        }

        assert!(
            state.current_state.state == State::Null,
            "{:?}",
            state.current_state.state
        );
    }
}
