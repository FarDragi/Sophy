use serenity::builder::CreateApplicationCommand;

pub trait BaseCommand {
    fn default_config<'a>(
        &self,
        application_command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        application_command
            .name(self.name())
            .description(self.description())
    }

    fn config<'a>(
        &self,
        application_command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand;

    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;
}
