#[allow(deprecated)]
use serenity::all::{
    Context, Message,
    standard::{CommandResult, macros::command},
};

use crate::CommandCounter;

#[allow(deprecated)]
#[command]
pub async fn counter(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let counter = data
        .get::<CommandCounter>()
        .expect("Expected CommandCounter in TypeMap");
    let count: u64 = counter.values().sum();

    let content = format!("Commands has been used {} times.", count);

    msg.channel_id.say(&ctx.http, content).await?;

    Ok(())
}
