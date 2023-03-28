use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};
use chrono::Duration;
use regex::Regex;
use std::{
    collections::HashMap,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
};
use tokio::sync::RwLock;
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!spend") {
            let time_str = msg.content.trim_start_matches("!spend").trim();
            if let Ok(duration) = parse_gitlab_duration(time_str) {
                let stop_flag = Arc::new(AtomicBool::new(false));
                let stop_flag_clone = Arc::clone(&stop_flag);

                tokio::spawn(async move {
                    let mut elapsed = Duration::zero();
                    let check_interval = Duration::milliseconds(100);
                    while elapsed < duration {
                        tokio::time::sleep(check_interval.to_std().unwrap()).await;
                        if stop_flag_clone.load(Ordering::SeqCst) {
                            break;
                        }
                        elapsed = elapsed + check_interval;
                    }
                    if !stop_flag_clone.load(Ordering::SeqCst) {
                        let _ = msg.channel_id.say(&ctx.http, "Timer finished!").await;
                    } else {
                        let _ = msg.channel_id.say(&ctx.http, "Timer stopped!").await;
                    }
                });

                let data = ctx.data.read().await;
                let mut timers = data.get::<Timers>().expect("Expected Timers in TypeMap").write().await;
                timers.insert(msg.author.id, stop_flag);
            } else {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, "Invalid time format. Please use GitLab's spend time format.")
                    .await;
            }
        }
        else if msg.content == "!stop" {
            let data = ctx.data.read().await;
            let timers = data.get::<Timers>().expect("Expected Timers in TypeMap").read().await;
            if let Some(stop_flag) = timers.get(&msg.author.id) {
                stop_flag.store(true, Ordering::SeqCst);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let application_id: u64 = std::env::var("APPLICATION_ID")
        .expect("Expected an application ID in the environment")
        .parse()
        .expect("Application ID is not a valid u64");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;


    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Timers>(Arc::new(RwLock::new(HashMap::new())));
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn parse_gitlab_duration(s: &str) -> Result<Duration, &'static str> {
    let re = Regex::new(r"(?P<value>\d+)(?P<unit>[wdhm])").unwrap();
    let mut total_duration = Duration::zero();
    for cap in re.captures_iter(s) {
        let value: i64 = cap["value"].parse().map_err(|_| "Invalid number")?;
        let unit = &cap["unit"];

        let duration = match unit {
            "w" => Duration::weeks(value),
            "d" => Duration::days(value),
            "h" => Duration::hours(value),
            "m" => Duration::minutes(value),
            _ => return Err("Invalid unit"),
        };

        total_duration = total_duration + duration;
    }

    if total_duration == Duration::zero() {
        return Err("No duration found");
    }

    Ok(total_duration)
}

struct Timers;

impl TypeMapKey for Timers {
    type Value = Arc<RwLock<HashMap<UserId, Arc<AtomicBool>>>>;
}
