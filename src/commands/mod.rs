mod base;
mod test;

pub use base::BaseCommand;
use serenity::{http::Http, model::interactions::application_command::ApplicationCommand};

use crate::{commands::test::TestCommand, config::Config};

pub fn get_all_commands() -> Vec<impl BaseCommand + Default> {
    vec![TestCommand]
}

pub async fn config_commands(
    config: &Config,
    http: &Http,
    commands: &[impl BaseCommand + Default],
) {
    if config.update_commands {
        let application_command =
            ApplicationCommand::set_global_application_commands(http, |application_commands| {
                for command in commands {
                    application_commands.create_application_command(|application_command| {
                        command.default_config(application_command);
                        command.config(application_command)
                    });
                }

                application_commands
            })
            .await;

        match application_command {
            Err(err) => {
                panic!("{:#?}", err)
            }
            Ok(cmds) => {
                for cmd in cmds {
                    println!("Update {} command", cmd.name)
                }
            }
        };
    }
}
