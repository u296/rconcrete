#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("the server does not exist")]
    ServerDoesNotExist,
    #[error("the server already exists")]
    ServerAlreadyExists,
    #[error("connection error: {0}")]
    ConnectionError(std::io::Error),
    #[error("authentication error")]
    AuthenticationError,
    #[error("command too long")]
    CommandTooLong,
    #[error("I/O error: {0}")]
    IoError(std::io::Error),
    #[error("bad input: {0}")]
    BadInput(BadInputClass),
    #[error("failed to fetch the configuration path")]
    FailedToFetchConfigPath,
    #[error("failed to parse the configuration: {0}")]
    ConfigParseError(serde_json::Error),
    #[error("failed to write configuration to json: {0}")]
    ConfigWriteError(serde_json::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum BadInputClass {
    #[error("malformed address, did you specify the port?")]
    MalformedAddress,
}
