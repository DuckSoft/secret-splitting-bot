use crate::secret_restore::SecretRestore;
use crate::secret_split::SecretSplit;
use anyhow::Error;
use telegram_bot::{Api, CanReplySendMessage, Message, MessageText, ParseMode};

const BOT_COMMAND_HELP: &str = r#"可用命令如下，注意无 / 前缀：

**帮助**：`help`
 - 显示帮助。

**拆分**：`split <K> <N> <SECRET>`
 - 将SECRET拆分为K份，至少需要N份才能还原。
 - 示例：`split 3 5 password`

**合并**：`restore <PART1> <PART2> ...`
 - 将拆分的SECRET还原。
 - 示例：`restore 3-2-r0wgEWDH97w 3-3-nMmaPq9fSm4 3-4-DdM9RzMMylA`

作者：https://github.com/DuckSoft/
"#;

pub async fn handle_bot_command(api: &Api, msg: &Message) -> anyhow::Result<()> {
    let cmd = msg
        .text()
        .ok_or(Error::msg("failed to parse message as text"))?;

    println!("Msg: {}", cmd);

    if cmd.starts_with("/start") || cmd.starts_with("help") {
        api.send(
            msg.text_reply(BOT_COMMAND_HELP)
                .parse_mode(ParseMode::Markdown)
                .reply_to(msg.id),
        )
        .await?;
    } else if cmd.starts_with("split") {
        api.send(
            msg.text_reply(
                cmd.splitn(2, " ")
                    .nth(1)
                    .ok_or(Error::msg("failed to fetch command args"))?
                    .parse::<SecretSplit>()?
                    .execute(),
            )
            .reply_to(msg.id),
        )
        .await?;
    } else if cmd.starts_with("restore") {
        api.send(
            msg.text_reply(
                cmd.splitn(2, " ")
                    .nth(1)
                    .ok_or(Error::msg("failed to fetch command args"))?
                    .parse::<SecretRestore>()?
                    .execute(),
            )
            .reply_to(msg.id),
        )
        .await?;
    }

    Ok(())
}
