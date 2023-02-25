use std::{error::Error as StdError, fmt};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorImpl>,
}

type BoxError = Box<dyn StdError + Sync + Send>;

pub(super) struct ErrorImpl {
    kind: ErrorKind,
    cause: Option<BoxError>,
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    Builder,
    Request,
    Response,
    ServerStatus(u16),
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.inner.kind.clone()
    }

    pub fn into_cause(self) -> Option<BoxError> {
        self.inner.cause
    }

    // Crate private methods
    pub(super) fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Box::new(ErrorImpl { kind, cause: None }),
        }
    }

    pub(super) fn with<C: Into<BoxError>>(mut self, cause: C) -> Self {
        self.inner.cause = Some(cause.into());
        self
    }

    #[inline]
    pub(super) fn builder<C: Into<BoxError>>(cause: C) -> Self {
        Self::new(ErrorKind::Builder).with(cause)
    }

    #[inline]
    pub(super) fn request<C: Into<BoxError>>(cause: C) -> Self {
        Self::new(ErrorKind::Request).with(cause)
    }

    #[inline]
    pub(super) fn response<C: Into<BoxError>>(cause: C) -> Self {
        Self::new(ErrorKind::Response).with(cause)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.cause.as_ref().map(|err| &**err as &dyn StdError)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.kind {
            ErrorKind::Builder => f.write_str("failed to create the API client")?,
            ErrorKind::Request => f.write_str("failed to make the API request")?,
            ErrorKind::Response => f.write_str("failed to parse API response data")?,
            ErrorKind::ServerStatus(code) => write!(f, "server returned an error code {code}")?,
        }

        if let Some(ref cause) = self.inner.cause {
            let cause = match cause.source() {
                Some(root_cause) => root_cause as &dyn StdError,
                None => &**cause as &dyn StdError,
            };

            write!(f, ": {}", cause)?
        }

        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const TUPLE_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "::Error");
        let mut tuple = f.debug_struct(TUPLE_NAME);

        tuple.field("kind", &self.inner.kind);
        if let Some(ref cause) = self.inner.cause {
            tuple.field("source", cause);
        }

        tuple.finish()
    }
}
