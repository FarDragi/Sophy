use sea_orm::DbErr;
use serenity::Error as BotErr;
use tokio::task::JoinError as ThreadErr;

#[derive(Debug)]
pub enum AppErr {
    Database(DbErr),
    Bot(BotErr),
    Thread(ThreadErr),
}

pub type AppResult<T> = Result<T, AppErr>;
