use dotenv::dotenv;
use std::env;
use serenity::async_trait;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::Client;
use reqwest::Client as HttpClient;
use serde_json::Value;
use crate::commands::register;
use crate::commands::ask_command;
mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {

        if let Interaction::Command(command) = interaction {
            if let "question" = command.data.name.as_str() {
                let prompt_option = command.data.options.get(0).unwrap();
                println!("{:?}", prompt_option.value);
                if let Some(prompt) = prompt_option.value.as_str() {
                    match gen_res(prompt).await {
                        Ok(response) => ask_command(ctx, command, &response).await,
                        _ => {
                            command.create_response(&ctx.http, CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .ephemeral(true)
                                    .content("generation failed")
                            ))
                            .await
                            .expect("generation failed");
                        }
                    }
                }

            } else {
                command.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                            .ephemeral(true)
                            .content("Invalid command!")
                ))
                .await
                .expect("failed to create response");
            }
        }     
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} has connected!", ready.user.name);

        let guild = ready.guilds[1];
        assert_eq!(guild.unavailable, true);
        let guild_id = guild.id;

        guild_id.set_commands(&ctx.http, vec![
            register(), 
        ])
        .await
        .expect("failed to create application command");
    }
}

async fn gen_res(prompt: &str) -> Result<String, reqwest::Error> {
    let data = format!(r#"
        {{
            "model": "llama3.1:8b",
            "prompt": "{}",
            "stream": false
        }}
    "#, prompt);
    let url = "http://localhost:11434/api/generate";
    let client = HttpClient::new();
    let res = client.post(url)
        .body(data)
        .send()
        .await?;
    let text = res.text().await?;
    let mut result = String::new();

    for line in text.lines() {
        if let Ok(parsed_json) = serde_json::from_str::<Value>(line) {
            if let Some(response) = parsed_json["response"].as_str() {
                result.push_str(response);
            }
        }
    }

    Ok(result)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DS_TOKEN").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::DIRECT_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Ошибка при создании клиента");

    if let Err(why) = client.start().await {
        println!("Ошибка при запуске бота: {:?}", why);
    }
}
