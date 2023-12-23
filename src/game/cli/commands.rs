use crossterm::style::Color;

use crate::{
    character::Character,
    class::Classes,
    item::EquipmentSlot,
    r#trait::CharacterTraits,
    skill::{Skill, SkillSet},
    ui::cli::{Messages, TICK_RATE},
    ValidEnum,
};

use super::{game_loop::GameState, locations::Locations, states::State};

type CommandFn = fn(&mut GameState, &[&str]) -> Messages;
pub struct Command {
    pub command: &'static str,
    pub help: &'static str,
    program: CommandFn,
}
impl Command {
    pub fn run(&self, state: &mut GameState, args: &[&str]) -> Messages {
        (self.program)(state, args)
    }
}

pub(crate) fn create(state: &mut GameState, args: &[&str]) -> Messages {
    if args.is_empty() {
        let mut msg = Messages::new();
        msg.push(
            format!("Invalid class {args:?} choose one of the following"),
            Color::Red,
        );
        msg.extend(Classes::valid_flat(), Color::Magenta);
        return msg;
    }

    match Classes::try_from(args.first().unwrap().to_string()) {
        Ok(class) => {
            let mut msg = vec![];
            if let Some(x) = args.get(1) {
                state.name = Some(x.to_string());
            }

            if state.character.is_some() {
                state.state = State::confirm(state.clone());
                msg.push((
                    "This action will delete your current character please confirm ( yes / no )"
                        .to_string(),
                    Color::Red,
                ));
            }

            state.class = Some(class);
            if state.character.is_none() {
                msg.push((format!("You are a level 1 {class}"), Color::Green));
            }
            if state.name.is_none() {
                state.state = State::name();
                msg.push((
                    "-- What is your hero's name? --".to_string(),
                    Color::DarkGreen,
                ))
            } else {
                let character = Some(Character::new(
                    state.name.as_mut().unwrap().clone(),
                    1,
                    class,
                ));
                state.character = character;
                if state.state == State::Null {
                    msg.push((
                        format!("Welcome to Carrion-Eris {}", state.name.as_ref().unwrap()),
                        Color::Green,
                    ))
                }
            }
            msg.into()
        }
        Err(_) => {
            let mut msg = Messages::new();
            let _valid_classes = Classes::valid_flat();
            msg.push(
                format!("Invalid class {args:?} choose one of the following"),
                Color::Red,
            );
            msg.extend(Classes::valid_flat(), Color::Magenta);
            msg
        }
    }
}

pub(crate) fn name(state: &mut GameState, args: &[&str]) -> Messages {
    let name = args
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
        .to_string();
    state.name = Some(name.clone());
    vec![(format!("Welcome to Carrion-Eris {name}"), Color::Green)].into()
}

pub(crate) fn travel(state: &mut GameState, args: &[&str]) -> Messages {
    Locations::travel(state, args).into()
}

pub(crate) fn equipment_display(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msgs = Messages::new();

    if state.character.is_none() {
        msgs.push("You have no equipment".to_string(), Color::Red);
        return msgs;
    }
    if !args.is_empty() {
        let slot = args.first().unwrap();
        let slot = match EquipmentSlot::try_from(slot.to_string()) {
            Ok(slot) => slot,
            Err(_) => {
                return vec![(
                    format!(
                        "Invalid slot {args:?} choose one of the following {:?}",
                        EquipmentSlot::valid_flat()
                    ),
                    Color::Red,
                )]
                .into()
            }
        };

        let vec: Vec<Vec<String>> = state
            .get_character_ref_unchecked()
            .equipment
            .get(slot)
            .iter()
            .map(|iter| iter.cli_display())
            .collect();

        if vec.is_empty() {
            msgs.push("You have no equipment".to_string(), Color::Red);
            return msgs;
        }

        for item in vec {
            msgs.extend(item, Color::Magenta);
        }

        let sum = state.get_character_ref_unchecked().equipment.sum();
        msgs.push(format!("Total: {sum}"), Color::Magenta);
        return msgs;
    }
    msgs.extend(
        state
            .get_character_ref_unchecked()
            .equipment
            .display_for_cli(),
        Color::Magenta,
    );

    let sum = state
        .get_character_ref_unchecked()
        .equipment
        .sum()
        .cli_display()
        .into_iter()
        .skip(2)
        .collect();
    msgs.extend(sum, Color::Magenta);
    msgs
}

pub(crate) fn camp(state: &mut GameState, _args: &[&str]) -> Messages {
    let mut msg = Messages::new();

    match state.state {
        State::Camping(x) => {
            let total_ticks = (6000 / TICK_RATE) as f64;

            if x >= total_ticks as u64 {
                let character = state.get_character_mut_unchecked();
                character.hp = character.max_hp as i32;
                state.state = State::Null;
                msg.push("You break camp refreshed".to_string(), Color::Grey);
                return msg;
            }

            state.state = State::Camping(x + 1);
            fn terminal_function(u: u64) -> (String, Color) {
                let total_ticks = (6000 / TICK_RATE) as f64;
                let current = (u as f64 / total_ticks * 100.0) as u64;
                (
                    format!("You have rested {current}% of the night", current = current),
                    Color::DarkGreen,
                )
            }
            msg.update(terminal_function, x);
        }

        State::Null => {
            state.state = State::Camping(0);
            msg.push("You setup camp for the night".to_string(), Color::Grey)
        }
        _ => {}
    }

    msg
}

pub(crate) fn select_trait(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msg = Messages::new();
    if state.character.is_none() {
        return msg;
    }

    if state.get_character_ref_unchecked().available_traits == 0 {
        msg.push("No trait points avaliable".to_string(), Color::Red);
        return msg;
    }

    if args.is_empty() {
        msg.extend(CharacterTraits::valid_flat(), Color::Blue);
        return msg;
    }
    match CharacterTraits::try_from(args.first().unwrap().to_string()) {
        Ok(tr) => match state.get_character_mut_unchecked().insert_trait(tr) {
            true => msg.push(format!("{} Trait added", tr), Color::Green),
            false => msg.push(format!("{} Trait already chosen", tr), Color::Red),
        },
        Err(err) => msg.push(err, Color::Red),
    }
    msg
}

pub(crate) fn change_skill(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msg = Messages::new();
    if args.is_empty() {
        let skill_display = state
            .get_character_ref_unchecked()
            .current_skill
            .display_for_cli();
        msg.extend(skill_display, Color::White);
    } else {
        match choose_skill(state.character.as_ref().unwrap(), args) {
            Ok((ok_msg, skill)) => {
                msg.send(ok_msg);
                state.character.as_mut().unwrap().current_skill = SkillSet::new(skill);
            }
            Err(e) => {
                msg.send(e);
            }
        }
    }
    msg
}

fn choose_skill(character: &Character, args: &[&str]) -> Result<(Messages, Skill), Messages> {
    let valid_skills = character.skill_list();
    let skill = match Skill::try_from(args.first().unwrap().to_string()) {
        Ok(skill) => skill,
        Err(_) => {
            return Err(vec![
                (
                    format!("Invalid skill {args:?} choose one of the following"),
                    Color::Red,
                ),
                (format!("{skills:?}", skills = valid_skills), Color::Red),
            ]
            .into())
        }
    };

    if !valid_skills.contains(&skill) {
        return Err(vec![
            (
                format!("Invalid skill {args:?} choose one of the following"),
                Color::Red,
            ),
            (format!("{skills:?}", skills = valid_skills), Color::Red),
        ]
        .into());
    }
    Ok((
        vec![(format!("Equipped {skill}"), Color::Green)].into(),
        skill,
    ))
}

pub(crate) fn auto(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msgs = Vec::new();

    if !args.is_empty() {
        return vec![(
            "Auto Battle can not be used with a skill change".to_string(),
            Color::Red,
        )]
        .into();
    }

    while state.state != State::Null && state.character.is_some() {
        msgs.extend(fight(state, args).iter().cloned());
        msgs.push((" ".to_string(), Color::White))
    }
    msgs.into()
}

pub(crate) fn auto_fight(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msgs = Vec::new();

    if !args.is_empty() {
        return vec![(
            "Auto Battle can not be used with a skill change".to_string(),
            Color::Red,
        )]
        .into();
    }

    if state.location.enemy().is_none() {
        return vec![("There is nothing to fight here".to_string(), Color::Red)].into();
    }

    msgs.extend(fight(state, args).iter().cloned());

    while state.state != State::Null && state.character.is_some() {
        msgs.extend(fight(state, args).iter().cloned());
        msgs.push((" ".to_string(), Color::White))
    }
    msgs.into()
}

pub(crate) fn fight(state: &mut GameState, args: &[&str]) -> Messages {
    let mut msg = Vec::new();
    match state.state.clone() {
        State::Fighting => {
            if args.len() == 1 {
                match choose_skill(state.character.as_ref().unwrap(), args) {
                    Ok((rmsg, skill)) => {
                        msg.extend(rmsg.iter().cloned());
                        state.character.as_mut().unwrap().current_skill = SkillSet::new(skill);
                    }
                    Err(msg) => return msg,
                }
            }
            if state.get_character_ref_unchecked().hp
                <= (state.get_character_ref_unchecked().max_hp as f64 * 0.35) as i32
            {
                msg.push(("You attempt to flee".to_string(), Color::Red));
                msg.extend(Locations::flee(state));
                state.state = State::Null;
                return msg.into();
            }
            let enemy = state.location.get_mut_enemy();

            let damage_done = state.character.as_mut().unwrap().cli_player(enemy);

            if !enemy.alive() {
                let mut messages = Messages::new();
                state.state = State::Null;

                let character_display = state.character.as_ref().unwrap().display_for_cli();

                messages.push(format!("You have killed the {}", &enemy.kind), Color::Green);
                messages.push(
                    format!("You have gained {} experience", enemy.experience),
                    Color::Cyan,
                );
                if !enemy.items.is_empty() {
                    messages.push(
                        format!("You have gained {} gold", enemy.gold),
                        Color::Yellow,
                    );
                    enemy.items.iter().for_each(|item| {
                        item.cli_display().into_iter().for_each(|x| {
                            messages.push(x, Color::DarkYellow);
                        });
                    });
                }

                state.location.enemy_killed();

                messages.push(
                    format!(
                        "You did {damage_done} damage with {}",
                        state.get_character_ref_unchecked().current_skill.skill()
                    ),
                    Color::Magenta,
                );
                messages.extend(character_display, Color::White);
                return messages;
            }
            let damage_taken = state.character.as_mut().unwrap().cli_enemy(enemy);

            if state.character.as_ref().unwrap().hp <= 0 {
                let mut msg = Vec::new();
                msg.push((
                    format!(
                        "You did {damage_done} damage with {}",
                        state.character.as_ref().unwrap().current_skill.skill()
                    ),
                    Color::Magenta,
                ));
                msg.push((format!("Enemy did {damage_taken} damage",), Color::Blue));
                msg.extend(vec![(
                    "You have been killed... better luck next time".to_string(),
                    Color::DarkYellow,
                )]);
                msg.push(("".to_string(), Color::White));
                msg.push((
                    "Create a new character to continue your adventures".to_string(),
                    Color::Blue,
                ));
                state.character = None;
                state.location.go_to_origin();
                state.state = State::Null;

                return msg.into();
            }

            state.state = State::Fighting;

            msg.push((
                format!(
                    "You did {damage_done} damage with {}",
                    state.character.as_ref().unwrap().current_skill.skill()
                ),
                Color::Magenta,
            ));
            msg.push((format!("Enemy did {damage_taken} damage",), Color::Blue));
            msg.push((
                format!("Enemy {} has {} health remaining", enemy.kind, enemy.health,),
                Color::DarkBlue,
            ));
            msg.push((
                format!(
                    "You have {} health remaining",
                    state.character.as_ref().unwrap().hp
                ),
                Color::DarkMagenta,
            ));

            msg.into()
        }
        State::Null => {
            if let Some(mob) = state.location.enemy() {
                state.state = State::Fighting;
                vec![(
                    format!("You are fighting a {mob}", mob = mob.kind),
                    Color::Red,
                )]
                .into()
            } else {
                vec![("There is nothing to fight here".to_string(), Color::Red)].into()
            }
        }
        _ => vec![("You cannot fight right now".to_string(), Color::Red)].into(),
    }
}

pub(crate) fn help(_state: &mut GameState, _args: &[&str]) -> Messages {
    COMMANDS
        .iter()
        .map(|command| (command.help.to_string(), Color::DarkYellow))
        .collect::<Vec<_>>()
        .into()
}

pub(crate) fn battle_help(_state: &mut GameState, _args: &[&str]) -> Messages {
    BATTLE_COMMANDS
        .iter()
        .map(|command| (command.help.to_string(), Color::DarkYellow))
        .collect::<Vec<_>>()
        .into()
}

pub(crate) static BATTLE_COMMANDS: [Command; 5] = [
    Command {
        command: "attack",
        help: "Attack the enemy - usage ( attack [ skill ] )",
        program: fight,
    },
    Command {
        command: "cast",
        help: "Cast a spell - usage ( cast [ spell ] )",
        program: fight,
    },
    Command {
        command: "use",
        help: "Use an item - usage ( item [ item ] )",
        program: fight,
    },
    Command {
        command: "help",
        help: "Prints the help menu - usage ( battle )",
        program: battle_help,
    },
    Command {
        command: "auto",
        help: "Auto battles",
        program: auto,
    },
];

pub(crate) static COMMANDS: [Command; 11] = [
    Command {
        command: "trait",
        help: "Select a trait - usage ( trait [ name ] ) ",
        program: select_trait,
    },
    Command {
        command: "eq",
        help: "Display your equipment - usage ( eq )",
        program: equipment_display,
    },
    Command {
        command: "camp",
        help: "Spend some time camping to rest up - usage ( camp [ optional ( break ) ] ) ",
        program: camp
    },
    Command {
        command: "fight",
        help: "Fight the enemy - usage ( fight )",
        program: fight,
    },
    Command {
        command: "af", 
        help: "Auto Fight - usage ( af )", 
        program: auto_fight
    },
    Command {
        command: "skill",
        help: "Change your skill or display your current skill - usage ( skill [ optional ( skill ) ] )",
        program: change_skill,
    },
    Command {
        command: "g",
        help: "Travel to a new location - usage ( g [ direction ] )",
        program: travel,
    },
    Command {
        command: "create",
        help: "Create a new character - usage ( create [ class ] )",
        program: create,
    },
    Command {
        command: "name",
        help: "Rename your character - usage ( name [ name ] )",
        program: name,
    },
    Command {
        command: "travel",
        help: "Travel to a new location - usage ( travel [ direction ] )",
        program: travel,
    },
    Command {
        command: "help",
        help: "Prints the help menu - usage ( help )",
        program: help,
    },
];
