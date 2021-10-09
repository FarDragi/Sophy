mod model;
mod none;
mod test;

use serenity::{
    client::Context,
    http::Http,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteraction},
    Error,
};

use crate::config::Config;

use self::{
    model::{ConfigCommand, RunCommand},
    none::NoneCommand,
    test::TestCommand,
};

pub async fn config_commands(config: &Config, http: &Http) {
    if config.update_commands {
        let application_commands =
            ApplicationCommand::set_global_application_commands(http, |application_commands| {
                TestCommand::create_command(application_commands);

                application_commands
            })
            .await;

        match application_commands {
            Err(err) => {
                panic!("{:#?}", err)
            }
            Ok(cmds) => {
                for cmd in cmds {
                    info!("Update {} command", cmd.name)
                }
            }
        };
    }
}

pub async fn run_command(
    ctx: &Context,
    command_interaction: &ApplicationCommandInteraction,
) -> Result<(), Error> {
    info!("Run command {}", command_interaction.data.name);

    match command_interaction.data.name.as_str() {
        "test" => {
            TestCommand::default()
                .start(ctx, command_interaction)
                .await?
        }
        _ => {
            NoneCommand::default()
                .start(ctx, command_interaction)
                .await?
        }
    }

    Ok(())
}
