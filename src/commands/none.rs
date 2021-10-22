use serenity::{
    builder::CreateEmbed, client::Context,
    model::interactions::application_command::ApplicationCommandInteraction, Error,
};

use crate::utils::constants::colors::YELLOW;

use super::model::RunCommand;

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
            .create_interaction_response(ctx, |x| {
                x.interaction_response_data(|x| x.add_embed(embed))
            })
            .await?;

        Ok(())
    }
}
