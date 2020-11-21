#![feature(str_split_once)]

mod command;
mod secret_restore;
mod secret_split;

use std::env;

use crate::command::handle_bot_command;
use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api = Api::new(env::var("BOT_TOKEN")?);
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update.unwrap();
        if let UpdateKind::Message(msg) = update.kind {
            if let Err(e) = handle_bot_command(&api, &msg).await {
                api.send(msg.text_reply(e.to_string()).reply_to(msg.id))
                    .await;
            }
        }
    }

    Ok(())
}
