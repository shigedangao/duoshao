use tauri::InvokeError;

#[derive(Debug)]
pub enum Error {
    Lock(String),
    NoDefinitions
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lock(target) => write!(f, "Unable to acquired the lock for: {target}"),
            Self::NoDefinitions => write!(f, "No definition has been founded for the targeted language")
        }
    }
}

impl std::error::Error for Error {}

impl From<Error> for InvokeError {
    fn from(err: Error) -> Self {
        InvokeError::from_anyhow(anyhow::Error::new(err))
    }
}