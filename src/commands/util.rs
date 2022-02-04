use crate::error::AppError;

use super::CommandContext;

#[command(slash_command)]
pub async fn ping(_ctx: CommandContext<'_>) -> Result<(), AppError> {
    Ok(())
}
