use std::fmt::{Display, Formatter};

use poise::serenity_prelude::Error as BotErr;

#[derive(Debug)]
pub enum AppError {
    BotError(BotErr),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
