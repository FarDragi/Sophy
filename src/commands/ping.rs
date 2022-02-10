use crate::error::{AppError, AppResult};

use super::CommandContext;

#[command(slash_command)]
pub async fn ping(ctx: CommandContext<'_>) -> AppResult<()> {
    handler(ctx).await
}

async fn handler(ctx: CommandContext<'_>) -> AppResult<()> {
    let duration = {
        let latency = ctx.data().shards_latency.lock().await;
        latency.get(&ctx.discord().shard_id).cloned()
    };

    if let Some(duration) = duration {
        ctx.say(format!("{:?}", duration))
            .await
            .map_err(AppError::BotError)?;
    }

    Ok(())
}
