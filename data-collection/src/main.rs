use serenity::model::application::interaction::Interaction;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use tokio::main;
use serenity::{prelude::*, async_trait, model::channel::Message};
use serenity::model::gateway::Ready;
use serenity::model::application::command::Command;
use std::collections::HashMap;
use std::time::Duration;
use std::{error::Error, env};
use tokio::sync::mpsc::{Sender, channel as tokio_channel};
use once_cell::sync::Lazy;
use std::fs;
use serde::Deserialize;
use clap::Parser;

#[derive(Deserialize)]
struct Record {
    emoji: String,
}

static WEBHOOK_USERNAME_SENDERS: Lazy<Mutex<HashMap<ChannelId, Sender<String>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

async fn start(handler: &Handler, ctx: Context, command: ApplicationCommandInteraction) {
    let output = handler.output.clone();

    let webhook = command.channel_id.create_webhook(&ctx.http, "corrupt-finder").await.expect("Failed to hook web-ily");
    let (sender, mut receiver) = tokio_channel(50);
    {
        WEBHOOK_USERNAME_SENDERS.lock().await.insert(command.channel_id, sender);
    }

    let file = fs::File::open(&handler.input).expect("Expected emoji.csv");
    let mut emojis = csv::Reader::from_reader(file);
    
    let mut corruptions = String::from("expected,actual\n");

    tokio::spawn(async move {
        for result in emojis.deserialize::<Record>() {
            tokio::time::sleep(Duration::from_millis(25)).await;
            if let Ok(record) = result {
                let expected = record.emoji;
                let result = webhook.execute(&ctx.http, false, |w| w.content("<message>").username(&expected)).await;
                match result {
                    Ok(Some(msg)) => println!("Response: {}", &msg.author),
                    Ok(None) => println!("Response: None"),
                    Err(e) => println!("Response: Error! {}", e),
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
        Command::create_global_application_command(&ctx.http, |command| {
            command.name("start").description("Start corrupted emoji search")
        }).await.expect("Failed to register start command");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name == "start" {
                start(&self, ctx, command).await;
            }
        }
    }

    async fn message(&self, _ctx: Context, new_message: Message) {
        let sender = WEBHOOK_USERNAME_SENDERS.lock().await.get(&new_message.channel_id).cloned();
        if let Some(sender) = sender {
            if new_message.webhook_id.is_some()  {
                sender.send(new_message.author.name).await.ok();
            }
        }
    }
}

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("BOT_TOKEN")?;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_WEBHOOKS;
    let handler = Handler::parse();

    let mut client = Client::builder(token, intents).event_handler(handler).await?;

    Ok(client.start().await?)
}
