use crate::classes::Classes;
use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::player::Character;
use crate::skills::Skill;
use crate::traits::CharacterTraits;
use crate::ValidEnum;
use crate::{Context, Error};
use tracing::field::debug;
use tracing::{debug, info, warn};

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom:
                "Carrion-Eris is an RPG autobattler. To begin /create a character.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
/// Select your character traits
#[poise::command(prefix_command, slash_command)]
pub async fn character_trait(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Select a trait from the list of valid traits"]
    command: Option<String>,
) -> Result<(), Error> {
    info!("character_trait input {:?}", command);
    let id = ctx.author().id.0;
    let mut character: Character = SurrealConsumer::get_character(id)
        .await?
        .expect("Failed to create character");
    if character.available_traits == 0 {
        ctx.reply("You have no traits to spend").await?;
        return Ok(());
    }
    if let Some(ctrait) = command {
        let ctrait = CharacterTraits::try_from(ctrait);
        match ctrait {
            Ok(ctrait) => {
                if !character.traits.insert(ctrait) {
                    ctx.reply("You already have that trait").await?;
                    return Ok(());
                }

                ctrait.attribute_mutator(&mut character.attributes);

                character.available_traits -= 1;
                let record = SurrealProducer::create_or_update_character(character.clone()).await?;
                match record {
                    Some(_record) => {
                        let created_character = SurrealConsumer::get_character(id)
                            .await?
                            .expect("Failed to create character");
                        ctx.send(|b| {
                            b.content(format!("Updated character: {}", created_character))
                                .ephemeral(true)
                        })
                        .await?;
                    }
                    None => {
                        ctx.reply("Failed to update character").await?;
                    }
                }
            }
            Err(e) => {
                ctx.reply(format!(
                    "Invalid trait: {}\n Valid Traits:\n {}",
                    e,
                    CharacterTraits::valid_traits()
                ))
                .await?;
            }
        }
    } else {
        ctx.reply(format!(
            "No trait provided\n Valid Traits:\n {}",
            CharacterTraits::valid_traits()
        ))
        .await?;
    }
    Ok(())
}
/// Delete your character and start over
#[poise::command(prefix_command, slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>,
) -> Result<(), Error> {
    info!("delete_character");
    info!("Command: {:?}", command);
    let e = SurrealProducer::delete_character(ctx.author().id.0).await?;
    let x = SurrealProducer::drop_character_skills(ctx.author().id.0).await?;
    let y = SurrealProducer::delete_enemy_uid(ctx.author().id.0).await?;
    info!(?e, ?x, ?y);
    match e {
        None => {
            ctx.reply(format!("No character to delete")).await?;
            Ok(())
        }
        Some(_e) => {
            ctx.reply(format!("Deleted character")).await?;
            Ok(())
        }
    }
}

/// Create your character using a class
#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Create a character form the list of valid classes"]
    command: Option<String>,
) -> Result<(), Error> {
    info!("create_character");
    info!("Command: {:?}", command);
    if let Some(class) = command {
        let class = Classes::try_from(class);
        match class {
            Ok(class) => {
                let name = ctx.author().name.clone();
                let id = ctx.author().id.0;
                let new_character = Character::new(name, id, class);
                let record = SurrealProducer::create_character(new_character).await?;
                match record {
                    Some(_record) => {
                        let created_character = SurrealConsumer::get_character(id)
                            .await?
                            .expect("Failed to create character");
                        ctx.send(|b| {
                            b.content(format!("Created character: {}", created_character))
                                .ephemeral(true)
                        })
                        .await?;
                    }
                    None => {
                        ctx.send(|b| {
                            b.content(format!("Failed to create character"))
                                .ephemeral(true)
                        })
                        .await?;
                    }
                }
            }
            Err(_) => {
                let valid_classes = Classes::valid();
                ctx.send(|b| {
                    b.content(format!(
                        "Invalid class: {:?}\n Valid Classes:\n {}",
                        class, valid_classes
                    ))
                    .ephemeral(true)
                })
                .await?;
            }
        }
    } else {
        let valid_classes = Classes::valid();
        ctx.send(|b| {
            b.content(format!(
                "No class provided\n Valid Classes:\n {}",
                valid_classes
            ))
            .ephemeral(true)
        })
        .await?;
    }

    Ok(())
}

/// Information about your character
#[poise::command(prefix_command, slash_command)]
pub async fn me(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Create a character form the list of valid classes"]
    _command: Option<String>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.0;
    let character = SurrealConsumer::get_character(user_id).await?;
    match character {
        Some(character) => {
            ctx.send(|b| {
                b.content(format!("Character: {}", character))
                    .ephemeral(true)
            })
            .await?;
        }
        None => {
            ctx.send(|b| b.content(format!("No character found")).ephemeral(true))
                .await?;
        }
    }
    Ok(())
}

/// Change your skill
#[poise::command(prefix_command, slash_command)]
pub async fn skill(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Create a character form the list of valid classes"]
    command: Option<String>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.0;
    match command {
        Some(command) => {
            let skill = Skill::try_from(command);
            match skill {
                Ok(skill) => {
                    let has_skill = SurrealConsumer::get_skill_id(user_id, skill.clone() as u64)
                        .await
                        .expect("Failed to get skill");
                    match has_skill {
                        Some(skill) => {
                            let set_skill = SurrealProducer::set_current_skill_id(skill, user_id)
                                .await
                                .expect("Failed to set skill");

                            ctx.send(|b| {
                                b.content(format!("Skill set: {:}", set_skill))
                                    .ephemeral(true)
                            })
                            .await?;
                        }
                        None => {
                            let set_skill =
                                SurrealProducer::set_current_skill_id(skill.into(), user_id)
                                    .await
                                    .expect("Failed to set skill");
                            let _valid_skills = Skill::valid();
                            ctx.send(|b| {
                                b.content(format!("Skill set: {:}\n", set_skill))
                                    .ephemeral(true)
                            })
                            .await?;
                        }
                    }
                }
                Err(err) => {
                    warn!("Invalid skill: {:?}", err);
                    ctx.send(|b| {
                        b.content(format!(
                            "Invalid skill: {:?}\n Valid Skills:\n {}",
                            err,
                            Skill::valid()
                        ))
                        .ephemeral(true)
                    })
                    .await?;
                }
            }
        }
        None => {
            let valid_skills = Skill::valid();
            let current_skill = SurrealConsumer::get_current_skill(user_id).await?;
            let mut responses = String::new();
            if let Some(current_skill) = current_skill {
                responses.push_str(&format!("Current Skill: {}\n", current_skill));
            }
            responses.push_str(&format!("Valid Skills:\n {}", valid_skills));
            ctx.send(|b| b.content(responses).ephemeral(true)).await?;
        }
    }
    Ok(())
}
