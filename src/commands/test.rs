use serenity::builder::CreateApplicationCommand;

use super::BaseCommand;

#[derive(Default)]
pub struct TestCommand;

impl BaseCommand for TestCommand {
    fn config<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
    }

    fn name(&self) -> &'static str {
        "teste"
    }

    fn description(&self) -> &'static str {
        "Command teste"
    }
}
