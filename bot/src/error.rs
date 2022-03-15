use std::fmt::{Display, Formatter};

use poise::serenity_prelude::Error as BotErr;
use tonic::{transport::Error as TonicErr, Status as GrpcErr};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Bot(BotErr),
    Tonic(TonicErr),
    Grpc(GrpcErr),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait MapError<T> {
    fn map_app_err(self) -> Result<T, AppError>;
}

impl<T> MapError<T> for Result<T, BotErr> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Bot)
    }
}

impl<T> MapError<T> for Result<T, TonicErr> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Tonic)
    }
}

impl<T> MapError<T> for Result<T, GrpcErr> {
    fn map_app_err(self) -> Result<T, AppError> {
        self.map_err(AppError::Grpc)
    }
}
