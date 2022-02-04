use crate::error::AppError;

use super::CommandContext;

#[command(slash_command)]
pub async fn ping(ctx: CommandContext<'_>) -> Result<(), AppError> {
    ping_interaction(ctx).await
}

async fn ping_interaction(ctx: CommandContext<'_>) -> Result<(), AppError> {
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
