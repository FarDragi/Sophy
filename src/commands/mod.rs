use poise::{Command, Context};

mod util;

use crate::error::AppError;

type Data = ();
type CommandContext<'a> = Context<'a, Data, AppError>;

pub fn get_commands() -> Vec<Command<(), AppError>> {
    vec![util::ping()]
}
