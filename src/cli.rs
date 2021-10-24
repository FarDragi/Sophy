use clap::{AppSettings, Clap};

use crate::states::config::Config;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(short, long)]
    update_commands: bool,
}

impl Args {
    pub fn aplay_configs(&self, config: &mut Config) {
        if self.update_commands {
            debug!("Update commands");
            config.update_commands();
        }
    }
}
