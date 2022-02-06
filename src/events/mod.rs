pub mod level;

use poise::{serenity_prelude::Context, Event, Framework};

use crate::{error::AppError, states::States};

use self::level::level_module_run;

pub async fn event_listener(
    _ctx: &Context,
    event: &Event<'_>,
    _framework: &Framework<States, AppError>,
    states: &States,
) -> Result<(), AppError> {
    if let Event::Message { new_message } = event {
        level_module_run(new_message, states).await
    }

    Ok(())
}
