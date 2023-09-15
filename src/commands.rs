use std::time::Duration;
use tokio::time::sleep;
use crate::database::surreal::consumer::SurrealConsumer;
use crate::database::surreal::producer::SurrealProducer;
use crate::{CarrionError, Character, Classes, Context, Error};
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
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[autocomplete = "poise::builtins::autocomplete_command"]
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
                    Some(record) => {
                        let created_character = SurrealConsumer::get_character(id)
                            .await?
                            .expect("Failed to create character");
                        ctx.reply(format!("Created character: {}", created_character))
                            .await?;
                    }
                    None => {
                        ctx.reply("Failed to create character").await?;
                    }
                }
            }
            Err(_) => {
                // TODO: Give a list of valid classes
                ctx.reply("Invalid class").await?;
            }
        }
    }
    else {
        // TODO: Give a list of valid classes
        ctx.reply("No class provided").await?;
    }

    Ok(())
}
