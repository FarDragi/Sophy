use clap::{AppSettings, Clap};

use crate::states::config::Config;

#[derive(Clap)]
#[clap(version = "0.1.0", setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(short, long, about = "Update bot commands")]
    update_commands: bool,
    #[clap(short, long, about = "Migrate database")]
    migrate: bool,
}

impl Args {
    pub fn aplay_configs(&self, config: &mut Config) {
        if self.update_commands {
            debug!("Update commands");
            config.update_commands = true;
        }
        if self.migrate {
            debug!("Migrate database");
            config.migrate = true;
        }
    }
}
