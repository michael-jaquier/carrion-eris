use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::player::Character;
use crate::{Context, Error};

use crate::classes::Classes;
use crate::traits::CharacterTraits;
use tracing::info;

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
    }
    Ok(())
}
/// Delete your character and start over
#[poise::command(prefix_command, slash_command)]
pub async fn delete_character(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>,
) -> Result<(), Error> {
    info!("delete_character");
    info!("Command: {:?}", command);
    let e = SurrealProducer::delete_character(ctx.author().id.0).await?;
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
                let valid_classes = Classes::valid_classes();
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
        let valid_classes = Classes::valid_classes();
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
    command: Option<String>,
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
