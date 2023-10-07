use crate::classes::Classes;
use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::enemies::Mob;
use surrealdb::iam::verify::token;

use crate::player::Character;
use crate::skills::Skill;
use crate::traits::CharacterTraits;
use crate::{Context, Error};
use crate::{EnemyEvents, ValidEnum};

use crate::items::EquipmentSlot;

use tracing::{info, warn};

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
    info!("character_trait input {:?}", character_trait);
    let now = tokio::time::Instant::now();
    let id = ctx.author().id.0;
    let mut character: Character = SurrealConsumer::get_character(id)
        .await?
        .expect("Failed to create character");
    if character.available_traits == 0 {
        ctx.reply("You have no traits to spend").await?;
        return Ok(());
    }
    if let Some(ctrait) = character_trait {
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
    info!("character_trait finish {:?}", now.elapsed());
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
    let e = SurrealProducer::delete_character(ctx.author().id.0).await?;
    let x = SurrealProducer::drop_character_skills(ctx.author().id.0).await?;
    let y = SurrealProducer::delete_related(ctx.author().id.0).await?;
    info!(?e, ?x, ?y);
    match e {
        None => {
            ctx.reply(format!("No character to delete")).await?;
            info!("delete_character finish {:?}", now.elapsed());
            Ok(())
        }
        Some(_e) => {
            ctx.reply(format!("Deleted character")).await?;
            info!("delete_character finish {:?}", now.elapsed());
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
    class: Option<String>,
) -> Result<(), Error> {
    info!("create_character");
    info!("Command: {:?}", class);
    let now = tokio::time::Instant::now();
    if let Some(class) = class {
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
    info!("create_character finish {:?}", now.elapsed());
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
    info!("me finish {:?}", now.elapsed());
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
    info!("skill finish {:?}", now.elapsed());
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
    let items = SurrealConsumer::get_items(user_id).await?;
    if items.is_none() {
        ctx.send(|b| b.content(format!("No items found")).ephemeral(true))
            .await?;
        return Ok(());
    }
    let mut items = items.unwrap();
    if let Some(slot) = slot {
        let slot = EquipmentSlot::try_from(slot);
        match slot {
            Ok(slot) => {
                items.sell(Some(slot));
                ctx.send(|b| b.content(format!("{}", items)).ephemeral(true))
                    .await?;
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
        let items = items.sell(None);
        ctx.send(|b| b.content(format!("{}", items)).ephemeral(true))
            .await?;
    }
    SurrealProducer::store_user_items(items, user_id)
        .await
        .expect("Failed to store items");
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
            Ok(slot) => match SurrealConsumer::get_items(user_id).await? {
                None => {
                    ctx.send(|b| b.content(format!("No items found")).ephemeral(true))
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
                info!("items finish {:?}", now.elapsed());
                Ok(())
            }
        };
    }
    match SurrealConsumer::get_items(user_id).await? {
        Some(items) => {
            ctx.send(|b| b.content(format!("{}", items)).ephemeral(true))
                .await?;
        }
        None => {
            ctx.send(|b| b.content(format!("No items found")).ephemeral(true))
                .await?;
        }
    }
    info!("items finish {:?}", now.elapsed());
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
        let user_id = ctx.author().id.0;
        let items = SurrealConsumer::get_items(user_id).await?;
        if items.is_none() {
            ctx.send(|b| b.content(format!("No items found")).ephemeral(true))
                .await?;
            info!("equip finish {:?}", now.elapsed());
            return Ok(());
        }
        let mut items = items.unwrap();
        let selected_item = items.take(item);
        if selected_item.is_none() {
            ctx.send(|b| {
                b.content(format!("Item to equip not found"))
                    .ephemeral(true)
            })
            .await?;
            info!("equip finish {:?}", now.elapsed());
            return Ok(());
        }
        let mut character = SurrealConsumer::get_character(user_id)
            .await?
            .expect("Failed to get character");

        let old_item = character.equipment.equip(selected_item.unwrap().clone());
        if let Some(old_item) = old_item {
            items.push(old_item);
            SurrealProducer::store_user_items(items, user_id)
                .await
                .expect("Failed to store items");
        }

        SurrealProducer::create_or_update_character(character).await?;
        ctx.send(|b| b.content(format!("Equipped item")).ephemeral(true))
            .await?;
    }
    info!("equip finish {:?}", now.elapsed());
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
            let n = num_entries.unwrap_or(1);
            let mut total_battle_cost = 0;
            let mut total_fights = 0;
            match Mob::try_from(command.clone()) {
                Ok(mob) => {
                    let gold = SurrealConsumer::get_items(user_id).await?;
                    match gold {
                        None => {
                            ctx.send(|b| {
                                b.content("You have no gold to spend on a battle")
                                    .ephemeral(true)
                            })
                            .await?;
                        }
                        Some(gold) => {
                            let old_gold = gold.gold as i128;
                            let mut gold = gold.gold as i128;
                            let character = SurrealConsumer::get_character(user_id)
                                .await?
                                .expect("Failed to get character");
                            let gen_enemy = || mob.generate(&character);
                            let mut vec_enemys = vec![];
                            for _ in 0..n {
                                let enemy = gen_enemy();
                                let cost = (enemy.gold * 3) / enemy.kind.grade() as u64;
                                gold -= cost as i128;
                                if gold < 0 {
                                    gold += cost as i128;
                                    break;
                                }
                                vec_enemys.push(enemy);
                            }
                            if vec_enemys.len() == 0 {
                                ctx.send(|b| {
                                    b.content(format!(
                                        "You do not have enough gold to battle a {}",
                                        Mob::try_from(command.clone()).unwrap()
                                    ))
                                    .ephemeral(true)
                                })
                                .await?;
                                info!("battle finish {:?}", now.elapsed());
                                return Ok(());
                            }
                            total_battle_cost = old_gold - gold;
                            total_fights = vec_enemys.len();
                            SurrealProducer::store_related_enemies(&character, vec_enemys)
                                .await
                                .expect("Failed to store enemy");

                            SurrealProducer::patch_user_gold(
                                total_battle_cost as u64,
                                user_id,
                                true,
                            )
                            .await
                            .expect("Failed to patch gold");
                        }
                    }
                }
                Err(huh) => {
                    ctx.send(|b| {
                        b.content(format!("Invalid enemy: {:}", huh))
                            .ephemeral(true)
                    })
                    .await?;
                }
            }

            if total_battle_cost > 0 {
                let mut response = String::new();
                response.push_str("`");
                response.push_str(&format!(
                    "You spent {} gold to battle {} {}'s\n",
                    total_battle_cost,
                    total_fights,
                    Mob::try_from(command.clone()).unwrap()
                ));
                response.push_str(&format!(
                    "Your enemy: {} was added to your battle queue\n",
                    Mob::try_from(command.clone()).unwrap()
                ));
                response.push_str("`");
                ctx.send(|b| b.content(response).ephemeral(true)).await?;
            } else {
                ctx.send(|b| {
                    b.content(format!(
                        "You do not have enough gold to battle a {}",
                        Mob::try_from(command.clone()).unwrap()
                    ))
                    .ephemeral(true)
                })
                .await?;
            }
        }
    }
    info!("battle finish {:?}", now.elapsed());
    Ok(())
}
