use std::{
    borrow::Cow,
    io::Error as IoError,
    result::Result as StdResult,
    sync::mpsc::RecvTimeoutError as ChannelTimeout,
    fmt::{
        self,
        Display,
        Formatter
    },
};
use serde_json::Error as JsonError;


#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    JsonError(JsonError),
    Timeout(ChannelTimeout),
    Conversion,
    SubscriptionFailed,
    ConnectionClosed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let msg = match self {
            Error::Conversion => Cow::Borrowed("Failed to convert values"),
            Error::SubscriptionFailed => Cow::Borrowed("Failed to subscribe to event"),
            Error::ConnectionClosed => Cow::Borrowed("Connection closed"),
            Error::IoError(ref err) => Cow::Owned(err.to_string()),
            Error::JsonError(ref err) => Cow::Owned(err.to_string()),
            Error::Timeout(ref err) => Cow::Owned(err.to_string()),
        };

        f.write_str(&msg)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::JsonError(err)
    }
}

impl From<ChannelTimeout> for Error {
    fn from(err: ChannelTimeout) -> Self {
        Error::Timeout(err)
    }
}

pub type Result<T> = StdResult<T, Error>;
