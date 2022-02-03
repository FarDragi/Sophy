use crate::states::config::Config;

#[derive(Parser)]
#[clap(version = "0.1.0")]
pub struct Args {
    /// Update bot commands
    #[clap(short, long)]
    update_commands: bool,
    /// Migrate database
    #[clap(short, long)]
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
