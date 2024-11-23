#[allow(deprecated)]
use serenity::{
    all::{Context, EditMessage, Message},
    framework::standard::{CommandResult, macros::command},
};
use std::time::Instant;

#[allow(deprecated)]
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = Instant::now();

    // Calculate bot latency
    let m = msg.channel_id.say(&ctx.http, "Calculating ping...").await;
    let bot_ping = start.elapsed();

    match m {
        Ok(mut message) => {
            let content = format!("🏓 Pong!\nBot latency: {}ms", bot_ping.as_millis(),);

            if let Err(why) = message
                .edit(&ctx.http, EditMessage::new().content(content))
                .await
            {
                println!("Error editing message: {:?}", why);
            }
        }
        Err(why) => println!("Error sending message: {:?}", why),
    }

    Ok(())
}
