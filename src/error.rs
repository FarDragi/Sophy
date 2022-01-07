use serenity::Error as BotErr;
use sqlx::Error as DbErr;
use tokio::task::JoinError as ThreadErr;

#[derive(Debug)]
pub enum AppErr {
    Database(DbErr),
    Bot(BotErr),
    Thread(ThreadErr),
}

pub type AppResult<T> = Result<T, AppErr>;

impl AppErr {
    pub fn log(&self) {
        match self {
            AppErr::Database(err) => {
                error!("Database: {:?}", err)
            }
            _ => {}
        }
    }
}
