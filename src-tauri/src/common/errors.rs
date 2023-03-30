#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("{0}")]
    Warning(String),

    #[error("{0}")]
    Danger(String),

    #[error("{0}")]
    Fatal(String),

    #[error("An unexpected issue occured.")]
    WarningUnexpected,

    #[error("An unexpected issue occured.")]
    DangerUnexpected,

    #[error("An unexpected issue occured.")]
    FatalUnexpected,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("severity", &self.severity())?;
        state.serialize_field("error", &self.to_string())?;
        state.end()
    }
}

impl Error {
    #[must_use]
    pub const fn severity(&self) -> u8 {
        match self {
            Error::Warning(_) | Error::WarningUnexpected => Severity::Warning as u8,
            Error::Danger(_) | Error::DangerUnexpected => Severity::Danger as u8,
            Error::Fatal(_) | Error::FatalUnexpected => Severity::Fatal as u8,

            _ => Severity::Fatal as u8,
        }
    }
}

pub type IncResult<T> = Result<T, Error>;

pub enum Severity {
    Warning,
    Danger,
    Fatal,
}
