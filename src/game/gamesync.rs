use std::collections::VecDeque;

use crate::{
    character::Character, class::Classes, enemy::Enemy, ui::cli::GameClient, BattleInfo, ValidEnum,
};

use crossterm::style::Color;
use heck::ToLowerCamelCase;

#[derive(Default, Clone, PartialEq)]
enum State {
    #[default]
    Null,
    Created,
    WaitingForName,
    ConfirmChoice(Box<GameState>),
}

// <fight>
// <create> warrior
// <name> me
// <go> direction
// <equip> longsword
// <sell> blade
// <use> item
// <sneak>
// <cast> fireball

type CommandFn = fn(&mut GameState, &[&str]) -> Vec<(String, Color)>;
pub struct Command {
    pub command: &'static str,
    pub help: &'static str,
    program: CommandFn,
}
impl Command {
    pub fn run(&self, state: &mut GameState, args: &[&str]) -> Vec<(String, Color)> {
        (self.program)(state, args)
    }
}

fn create(state: &mut GameState, args: &[&str]) -> Vec<(String, Color)> {
    if args.len() != 1 {
        let valid_classes = Classes::valid_flat();
        let mut msgs = vec![(
            format!("Invalid class {args:?} choose one of the following"),
            Color::Red,
        )];
        let mut vc = valid_classes
            .split_terminator("ø")
            .map(|clas| (clas.to_string(), Color::Red))
            .collect::<Vec<_>>();

        msgs.append(&mut vc);
        return msgs;
    }
    let a = args.first().unwrap().to_string();
    match Classes::try_from(a) {
        Ok(class) => {
            let mut msg = vec![];
            if state.character.is_some() {
                state.state = State::ConfirmChoice(Box::new(state.clone()));
                msg.push((format!("This action will delete your current character please confirm ( yes / no )"), Color::Red));
            }

            state.class = Some(class);
            if state.character.is_none() {
                msg.push((format!("You are a level 1 {class}"), Color::Green));
            }
            if state.name.is_none() {
                state.state = State::WaitingForName;
                msg.push((format!("-- What is your hero's name? --"), Color::DarkGreen))
            } else {
                let character = Some(Character::new(
                    state.name.as_mut().unwrap().clone(),
                    1,
                    class,
                ));
                state.character = character;
                if state.state == State::Null {
                    msg.push((
                        format!(
                            "Welcome back to Carrion-Eris {}",
                            state.name.as_ref().unwrap()
                        ),
                        Color::Green,
                    ))
                }
            }
            return msg;
        }
        Err(_) => {
            let valid_classes = Classes::valid_flat();
            let mut msgs = vec![(
                format!("Invalid class {args:?} choose one of the following"),
                Color::Red,
            )];
            let mut vc = valid_classes
                .split_terminator("ø")
                .map(|clas| (clas.to_string(), Color::Red))
                .collect::<Vec<_>>();
            msgs.append(&mut vc);
            msgs
        }
    }
}

fn name(state: &mut GameState, args: &[&str]) -> Vec<(String, Color)> {
    let name = args
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string();
    state.name = Some(name.clone());
    vec![(format!("Welcome to Carrion-Eris {name}"), Color::Green)]
}

fn help(state: &mut GameState, args: &[&str]) -> Vec<(String, Color)> {
    COMMANDS
        .iter()
        .map(|command| ((format!("{}", command.help.to_string()), Color::DarkYellow)))
        .collect()
}

static COMMANDS: [Command; 3] = [
    Command {
        command: "create",
        help: "Create a new character - usage ( create [ class ] ) ",
        program: create,
    },
    Command {
        command: "name",
        help: "Rename your character - usage ( name [ name ] )",
        program: name,
    },
    Command {
        command: "help",
        help: "Prints the help menu - usage ( help )",
        program: help,
    },
];

#[derive(Default)]
pub struct GameStates {
    current_state: GameState,
}

impl GameStates {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn command(&mut self, command: String, client: &mut GameClient) {
        match &self.current_state.state {
            State::ConfirmChoice(previous_date) => {
                if !command.to_lowercase().contains('y') {
                    self.current_state = *previous_date.clone();
                    let msg = vec![("Change aborted".to_string(), Color::Red)];
                    client.send(msg);
                    self.current_state.state = State::Null;
                    return;
                } else {
                    let msg = vec![(
                        format!(
                            "Change applied {}",
                            self.current_state.character.as_ref().unwrap()
                        ),
                        Color::Magenta,
                    )];
                    client.send(msg);
                    return;
                }
            }
            _ => {}
        };
        self.current_state.command(command, client);
    }
}
#[derive(Default, Clone, PartialEq)]
struct GameState {
    class: Option<Classes>,
    name: Option<String>,
    character: Option<Character>,
    enemy: Option<Enemy>,
    battle: Option<BattleInfo>,
    state: State,
}

impl GameState {
    fn command(&mut self, command: String, client: &mut GameClient) {
        let mut success = false;

        if self.state == State::WaitingForName {
            self.name = Some(command.clone());
            let character = Character::new(command.clone(), 1, self.class.unwrap());
            self.character = Some(character);
            client.send(vec![(
                format!("Welcome to Carrion-Eris {command}"),
                Color::Green,
            )]);

            self.state = State::Null;
            return;
        }

        if let Some((inner_command, args)) = command
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .split_first()
        {
            COMMANDS.iter().for_each(|m| {
                if m.command == *inner_command {
                    client.send(m.run(self, args));
                    success = true;
                }
            });
        }

        if !success {
            let mut error_msg = vec![(format!("Command not recognized {command}"), Color::Red)];
            let mut help_msg = help(self, &[]);
            error_msg.append(&mut help_msg);
            client.send(error_msg);
        }
    }
}

#[cfg(test)]
mod test {

    fn splits() {
        let yy = "create warrior".to_string();
        let xx: Vec<&str> = yy.split_ascii_whitespace().collect();
        let zz = xx.split_first();
        println!("{zz:?}")
    }
}
