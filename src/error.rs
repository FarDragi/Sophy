use std::fmt::{Display, Formatter};

use poise::serenity_prelude::Error as BotErr;
use sqlx::error::DatabaseError;

#[derive(Debug)]
pub enum AppError {
    BotError(BotErr),
    DatabaseError(Option<Box<dyn DatabaseError + 'static>>),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait MapDatabaseError<T> {
    fn map_database_error(self) -> Result<T, AppError>;
}

impl<T> MapDatabaseError<T> for Result<T, sqlx::error::Error> {
    fn map_database_error(self) -> Result<T, AppError> {
        self.map_err(|err| AppError::DatabaseError(err.into_database_error()))
    }
}
