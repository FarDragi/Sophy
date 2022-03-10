use poise::{Command, Context};

mod ping;

use crate::{error::AppError, states::States};

pub type CommandContext<'a> = Context<'a, States, AppError>;

pub fn get_commands() -> Vec<Command<States, AppError>> {
    vec![ping::ping()]
}
