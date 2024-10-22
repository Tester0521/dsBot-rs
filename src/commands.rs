use serenity::all::{CommandInteraction, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::CommandOptionType;
use serenity::prelude::Context;
use serenity::builder::CreateCommandOption;

pub fn register() -> CreateCommand {
    let option = CreateCommandOption::new(CommandOptionType::String, "prompt", "Ваше сообщение");
    CreateCommand::new("question")
        .description("Узнай у бота").add_option(option)
}

pub async fn ask_command(ctx: Context, interaction: CommandInteraction, res: &str) {
    interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .content(res)
    ))
    .await
    .expect("failed to create interaction");
}