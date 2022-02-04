use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AppError;

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
