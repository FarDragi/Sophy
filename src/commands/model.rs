use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    client::Context,
    model::interactions::application_command::ApplicationCommandInteraction,
    Error,
};

pub trait ConfigCommand {
    fn create_command(application_commands: &mut CreateApplicationCommands) {
        application_commands.create_application_command(|application_command| {
            Self::config(application_command);

            application_command
        });
    }

    fn config(application_command: &mut CreateApplicationCommand);
}

#[async_trait]
pub trait RunCommand {
    async fn start(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        self.begin_run(ctx, interaction).await?;
        self.run(ctx, interaction).await?;
        self.after_run(ctx, interaction).await?;
        Ok(())
    }

    async fn begin_run(
        &self,
        _ctx: &Context,
        _interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        Ok(())
    }

    async fn run(
        &self,
        _ctx: &Context,
        _interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error>;

    async fn after_run(
        &self,
        _ctx: &Context,
        _interaction: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        Ok(())
    }
}
