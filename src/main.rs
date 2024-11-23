mod commands;

use dotenv::dotenv;
#[allow(deprecated)]
use serenity::all::standard::Configuration;
use serenity::all::standard::macros::{group, help, hook};
#[allow(deprecated)]
use serenity::all::standard::{Args, CommandGroup, CommandResult, HelpOptions, help_commands};
#[allow(deprecated)]
use serenity::all::{
    Context, EventHandler, GatewayIntents, Http, Ready, ShardManager, StandardFramework,
};
use serenity::all::{Message, UserId};
use serenity::prelude::TypeMapKey;
use serenity::{Client, async_trait};
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::Arc;
use tokio;
use tracing::{error, info};

use crate::commands::counter::*;
use crate::commands::ping::*;

pub struct ShardManagerContainer;
pub struct CommandCounter;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

struct Handler;
struct BotConfig {
    token: String,
    // prefix: String,
}

impl BotConfig {
    fn new(token: Option<String>) -> Self {
        match token {
            Some(token) => BotConfig { token },
            None => {
                let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
                BotConfig { token }
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[hook]
async fn before(ctx: &Context, _: &Message, command_name: &str) -> bool {
    info!("Got command '{}'", command_name);

    let mut data = ctx.data.write().await;
    let counter = data
        .get_mut::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true
}

#[allow(deprecated)]
#[group]
#[commands(ping)]
struct General;

#[allow(deprecated)]
#[group]
#[owners_only]
#[commands(counter)]
struct DevOnly;

#[allow(deprecated)]
#[help]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let config = BotConfig::new(None);

    let http = Http::new(&config.token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            let bot_id = match http.get_current_user().await {
                Ok(bot) => bot.id,
                Err(why) => panic!("Could not access bot id: {:?}", why),
            };

            (owners, bot_id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    #[allow(deprecated)]
    let framework = StandardFramework::new()
        .group(&GENERAL_GROUP)
        .group(&DEVONLY_GROUP)
        .before(before)
        .help(&MY_HELP);
    #[allow(deprecated)]
    framework.configure(
        Configuration::new()
            .owners(owners)
            .prefix("sul.")
            .on_mention(Some(bot_id)),
    );
    let mut client = Client::builder(&config.token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<CommandCounter>(HashMap::new());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error waiting for ctrl-c");

        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
