use tauri::InvokeError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum Error {
    Lock(String),
    NoDefinitions,
    Export(String),
    IO(String),
    Load(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Lock(target) => write!(f, "Unable to acquired the lock for: {target}"),
            Error::NoDefinitions => write!(f, "No definition has been founded for the targeted language"),
            Error::Export(msg) => write!(f, "Unable generate export for reasons: {msg}"),
            Error::IO(msg) => write!(f, "Unable to write the targeted file for reasons: {msg}"),
            Error::Load(msg) => write!(f, "Unable to load dictionary {msg}")
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from_anyhow(anyhow::Error::new(err))
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Self {
        Error::Load(err.to_string())
    }
}
