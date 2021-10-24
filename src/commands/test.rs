use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    model::interactions::{
        application_command::ApplicationCommandInteraction,
        InteractionResponseType::ChannelMessageWithSource,
    },
    Error,
};

use crate::models::commands::{ConfigCommand, RunCommand};

#[derive(Default)]
pub struct TestCommand;

impl ConfigCommand for TestCommand {
    fn config(application_command: &mut CreateApplicationCommand) {
        application_command.name("test").description("Test command");
    }
}

#[async_trait]
impl RunCommand for TestCommand {
    async fn run(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        interaction
            .create_interaction_response(ctx, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content("Test"))
            })
            .await
    }
}
