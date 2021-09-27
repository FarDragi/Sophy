use serenity::{model::interactions::application_command::ApplicationCommandInteraction, Error};

use super::model::RunCommand;

#[derive(Default)]
pub struct NoneCommand;

#[async_trait]
impl RunCommand for NoneCommand {
    async fn run(
        &self,
        _ctx: &serenity::client::Context,
        _interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        Ok(())
    }
}
