use clap::Parser;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serenity::all::CreateCommand;
use serenity::all::CreateWebhook;
use serenity::all::ExecuteWebhook;
use serenity::model::application::Command;
use serenity::model::application::CommandInteraction;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::ChannelId;
use serenity::{async_trait, model::channel::Message, prelude::*};
use std::collections::HashMap;
use std::fs;
use std::time::Duration;
use std::{env, error::Error};
use tokio::main;
use tokio::sync::mpsc::{channel as tokio_channel, Sender};

#[derive(Deserialize)]
struct Record {
    emoji: String,
}

static WEBHOOK_USERNAME_SENDERS: Lazy<Mutex<HashMap<ChannelId, Sender<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

async fn start(handler: &Handler, ctx: Context, command: CommandInteraction) {
    let output = handler.output.clone();

    let webhook = command
        .channel_id
        .create_webhook(&ctx.http, CreateWebhook::new("corrupt-finder"))
        .await
        .expect("Failed to hook web-ily");
    let (sender, mut receiver) = tokio_channel(50);
    {
        WEBHOOK_USERNAME_SENDERS
            .lock()
            .await
            .insert(command.channel_id, sender);
    }

    let file = fs::File::open(&handler.input).expect("Expected emoji.csv");
    let mut emojis = csv::Reader::from_reader(file);

    let mut corruptions = String::from("expected,actual\n");

    tokio::spawn(async move {
        for result in emojis.deserialize::<Record>() {
            tokio::time::sleep(Duration::from_millis(1000)).await;
            if let Ok(record) = result {
                let expected = record.emoji;
                let result = webhook
                    .execute(
                        &ctx.http,
                        false,
                        ExecuteWebhook::new()
                            .content("<message>")
                            .username(&expected),
                    )
                    .await;
                if let Err(e) = result {
                    println!("Failure during `{}`. {}", expected, e);
                    continue;
                }
                let actual = receiver.recv().await.expect("oops");

                corruptions.push_str(&format!("{expected},{actual}\n"));
            }
        }

        fs::write(&output, corruptions).expect("OOOPS");
    });
}

#[derive(Parser)]
struct Handler {
    input: String,
    output: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        Command::create_global_command(
            &ctx.http,
            CreateCommand::new("start").description("Start corrupted emoji search"),
        )
        .await
        .expect("Failed to register start command");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if command.data.name == "start" {
                start(&self, ctx, command).await;
            }
        }
    }

    async fn message(&self, _ctx: Context, new_message: Message) {
        let sender = WEBHOOK_USERNAME_SENDERS
            .lock()
            .await
            .get(&new_message.channel_id)
            .cloned();
        if let Some(sender) = sender {
            if new_message.webhook_id.is_some() {
                sender.send(new_message.author.name).await.ok();
            }
        }
    }
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("BOT_TOKEN")?;
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_WEBHOOKS;
    let handler = Handler::parse();

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await?;

    Ok(client.start().await?)
}
