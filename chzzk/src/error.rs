pub struct Error(
    pub String,
    pub Option<Box<dyn std::error::Error + Sync + Send>>,
);

pub fn chain_error<T: std::error::Error + Send + Sync + 'static>(
    message: &str,
) -> impl FnOnce(T) -> Error + use<'_, T> {
    move |error| Error(message.into(), Some(Box::new(error)))
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    #[deny(useless_deprecated)]
    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ChzzkError")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self(value, None)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self(value.to_string(), None)
    }
}
