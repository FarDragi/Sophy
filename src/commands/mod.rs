mod none;
mod test;

use serenity::{
    client::Context,
    http::Http,
    model::interactions::application_command::{ApplicationCommand, ApplicationCommandInteraction},
    Error,
};
use tokio::task;

use crate::{
    database::functions::user::{register_user, UserInput},
    models::commands::{ConfigCommand, RunCommand},
    states::config::Config,
};

use self::{none::NoneCommand, test::TestCommand};

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
    let name = command_interaction.data.name.as_str();

    info!("Run command {}", name);

    let user = UserInput {
        id: command_interaction.user.id.0,
        name: command_interaction.user.name.to_owned(),
    };
    task::spawn(register_user(user));

    match name {
        "test" => {
            TestCommand::default()
                .start(ctx, command_interaction)
                .await?
        }
        _ => {
            warn!("Command {} not found", name);
            NoneCommand::default()
                .start(ctx, command_interaction)
                .await?
        }
    }

    Ok(())
}
