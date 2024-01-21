#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove, not permanent.

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
