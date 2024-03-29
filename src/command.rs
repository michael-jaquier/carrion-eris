use crate::class::Classes;

use crate::enemy::Mob;

use crate::character::Character;
use crate::r#trait::CharacterTraits;
use crate::skill::Skill;
use crate::ValidEnum;
use crate::{Context, Error};

use crate::item::EquipmentSlot;

use crate::constructed::ItemsWeHave;
use crate::game::mutations::Mutations;
use crate::game_loop::{get_buffer, get_game};
use tracing::{info, warn};

fn tracing_span(user_id: u64, now: tokio::time::Instant, request: String) -> tracing::Span {
    let span = tracing::info_span!("commands", user_id = user_id, time = ?now.elapsed(), request = request);
    info!(parent: &span, "command finished");
    span
}
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
    character_trait: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let id = ctx.author().id.0;
    if let Some(ctrait) = character_trait {
        let ctrait = CharacterTraits::try_from(ctrait);
        match ctrait {
            Ok(ctrait) => {
                get_buffer().await.add(Mutations::Trait(id, ctrait));
                ctx.reply(format!(
                    "If the trait is unique will be set soon: {:}",
                    ctrait
                ))
                .await?;
            }
            Err(e) => {
                ctx.reply(format!(
                    "Invalid trait: {}\n Valid Traits:\n {}",
                    e,
                    CharacterTraits::valid()
                ))
                .await?;
            }
        }
    } else {
        ctx.reply(format!(
            "No trait provided\n Valid Traits:\n {}",
            CharacterTraits::valid()
        ))
        .await?;
    }
    tracing_span(id, now, "character_trait".to_string());
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
    let now = tokio::time::Instant::now();
    get_buffer().await.add(Mutations::Delete(ctx.author().id.0));

    ctx.reply("Deleted character").await?;
    tracing_span(ctx.author().id.0, now, "delete".to_string());
    Ok(())
}

/// Create your character using a class
#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Create a character form the list of valid classes"]
    class: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    if let Some(class) = class {
        let class = Classes::try_from(class);
        match class {
            Ok(class) => {
                let name = ctx.author().name.clone();
                let id = ctx.author().id.0;
                let new_character = Character::new(name, id, class);
                ctx.send(|b| {
                    b.content(format!("Created character: {}", new_character))
                        .ephemeral(true)
                })
                .await?;
                get_buffer()
                    .await
                    .add(Mutations::Create(Box::new(new_character)));
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
    tracing_span(ctx.author().id.0, now, "create".to_string());
    Ok(())
}

/// Information about your character
#[poise::command(prefix_command, slash_command)]
pub async fn me(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Details about your self"]
    _command: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let user_id = ctx.author().id.0;
    let character = get_game().await.get_character(user_id);
    match character {
        Some(character) => {
            ctx.send(|b| {
                b.content(format!("Character: {}", character))
                    .ephemeral(true)
            })
            .await?;
        }
        None => {
            ctx.send(|b| b.content("No character found").ephemeral(true))
                .await?;
        }
    }

    tracing_span(user_id, now, "me".to_string());
    Ok(())
}

/// Change your skill
#[poise::command(prefix_command, slash_command)]
pub async fn skill(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Select your skill from the list of valid skills"]
    skill: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let user_id = ctx.author().id.0;
    match skill {
        Some(command) => {
            let skill = Skill::try_from(command);
            match skill {
                Ok(skill) => {
                    get_buffer().await.add(Mutations::Skill(user_id, skill));

                    ctx.send(|b| {
                        b.content(format!("Skill set: {:}\n", skill))
                            .ephemeral(true)
                    })
                    .await?;
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
            let mut responses = String::new();
            responses.push_str(&format!("Valid Skills:\n {}", valid_skills));
            ctx.send(|b| b.content(responses).ephemeral(true)).await?;
        }
    }
    tracing_span(user_id, now, "skill".to_string());
    Ok(())
}
/// Sell Items
#[poise::command(prefix_command, slash_command)]
pub async fn sell(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Details about your item stash"]
    slot: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let user_id = ctx.author().id.0;
    if let Some(slot) = slot {
        let slot = EquipmentSlot::try_from(slot);
        match slot {
            Ok(slot) => {
                let current_items = get_game().await.get_items(user_id);

                ctx.send(|b| {
                    b.content(format!("Selling all items of type: {}", slot))
                        .ephemeral(true)
                })
                .await?;
                get_buffer()
                    .await
                    .add(Mutations::Sell(user_id, Some(slot), current_items));
            }

            Err(_) => {
                ctx.send(|b| {
                    b.content(format!(
                        "Invalid slot: {:?}\n Valid Slots:\n {}",
                        slot,
                        EquipmentSlot::valid()
                    ))
                    .ephemeral(true)
                })
                .await?;
            }
        };
    } else {
        ctx.send(|b| b.content("Selling all items").ephemeral(true))
            .await?;

        let current_items = get_game().await.get_items(user_id);
        get_buffer()
            .await
            .add(Mutations::Sell(user_id, None, current_items));
    }
    info!("sell finish {:?}", now.elapsed());
    Ok(())
}
/// Current Item Stash
#[poise::command(prefix_command, slash_command)]
pub async fn items(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Details about your item stash"]
    slot: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let user_id = ctx.author().id.0;
    if let Some(slot) = slot {
        let slot = EquipmentSlot::try_from(slot);
        return match slot {
            Ok(slot) => match get_game().await.get_items(user_id) {
                None => {
                    ctx.send(|b| b.content("No items found").ephemeral(true))
                        .await?;
                    Ok(())
                }
                Some(items) => {
                    let filtered_items = items.slot(slot);
                    ctx.send(|b| b.content(format!("{}", filtered_items)).ephemeral(true))
                        .await?;
                    info!("items finish {:?}", now.elapsed());
                    Ok(())
                }
            },
            Err(_) => {
                ctx.send(|b| {
                    b.content(format!(
                        "Invalid slot: {:?}\n Valid Slots:\n {}",
                        slot,
                        EquipmentSlot::valid()
                    ))
                    .ephemeral(true)
                })
                .await?;
                Ok(())
            }
        };
    }
    match get_game().await.get_items(user_id) {
        Some(items) => {
            ctx.send(|b| b.content(format!("{}", items)).ephemeral(true))
                .await?;
        }
        None => {
            ctx.send(|b| b.content("No items found").ephemeral(true))
                .await?;
        }
    }
    tracing_span(user_id, now, "items".to_string());
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn sum(ctx: Context<'_>) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    let user_id = ctx.author().id.0;
    match get_game().await.get_character(user_id) {
        Some(items) => {
            ctx.send(|b| {
                b.content(format!("{}", items.equipment.sum()))
                    .ephemeral(true)
            })
            .await?;
        }
        None => {
            ctx.send(|b| b.content("No items found").ephemeral(true))
                .await?;
        }
    }
    tracing_span(user_id, now, "sum".to_string());
    Ok(())
}

/// Equip a specific item
#[poise::command(prefix_command, slash_command)]
pub async fn equip(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Item to be equipped"]
    item: Option<String>,
) -> Result<(), Error> {
    let now = tokio::time::Instant::now();
    if let Some(item) = item {
        ctx.send(|b| b.content("Equipped item").ephemeral(true))
            .await?;
        let user_id = ctx.author().id.0;
        let item = ItemsWeHave::try_from(item);
        match item {
            Ok(item) => {
                get_buffer()
                    .await
                    .add(Mutations::Equip(user_id, item.generate()));
                info!("equip finish {:?}", now.elapsed());
            }
            Err(_) => {
                ctx.send(|b| {
                    b.content(format!(
                        "Invalid item: {:?}\n Valid Items:\n {}",
                        item,
                        ItemsWeHave::valid()
                    ))
                    .ephemeral(true)
                })
                .await?;
            }
        }
    }
    Ok(())
}

/// Battle an enemy
#[poise::command(prefix_command, slash_command)]
pub async fn battle(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
    #[description = "Battle an enemy from the list of valid enemies"]
    enemy: Option<String>,
    #[description = "Number of the enemy to queue"] num_entries: Option<u32>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.0;
    let now = tokio::time::Instant::now();
    info!("battle start");
    match enemy {
        None => {
            let valid_enemy = Mob::valid();
            ctx.send(|b| b.content(valid_enemy).ephemeral(true)).await?;
        }
        Some(command) => {
            let enemy = Mob::try_from(command);
            match enemy {
                Ok(enemy) => {
                    ctx.send(|b| {
                        b.content(format!("Battle queued: {:}", enemy))
                            .ephemeral(true)
                    })
                    .await?;
                    get_buffer().await.add(Mutations::AddEnemy(
                        user_id,
                        enemy,
                        num_entries.unwrap_or(1),
                    ));
                }
                Err(_) => {
                    ctx.send(|b| {
                        b.content(format!(
                            "Invalid enemy: {:?}\n Valid Enemies:\n {}",
                            enemy,
                            Mob::valid()
                        ))
                        .ephemeral(true)
                    })
                    .await?;
                }
            }
        }
    };

    info!("battle finish {:?}", now.elapsed());
    Ok(())
}
