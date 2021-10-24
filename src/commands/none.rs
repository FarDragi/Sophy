use serenity::{
    builder::CreateEmbed, client::Context,
    model::interactions::application_command::ApplicationCommandInteraction, Error,
};

use crate::{models::commands::RunCommand, utils::constants::colors::YELLOW};

#[derive(Default)]
pub struct NoneCommand;

#[async_trait]
impl RunCommand for NoneCommand {
    async fn run(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        let mut embed = CreateEmbed::default();
        embed.description("Comando não enconstrado");
        embed.color(YELLOW);

        interaction
            .create_interaction_response(ctx, |response| {
                response.interaction_response_data(|msg| msg.add_embed(embed))
            })
            .await?;

        Ok(())
    }
}
