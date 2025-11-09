use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    Unauthorized(&'static str),
}
