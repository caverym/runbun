use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Toml(toml::de::Error),
    Io(std::io::Error),
    DaemonNotRunning(String),
    InvalidEnvar(String),
    InvalidDaemonKind(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Toml(err) => write!(f, "Toml error: {}", err),
            Error::Io(err) => write!(f, "Io error: {}", err),
            Error::DaemonNotRunning(desc) => write!(f, "Daemon {} is not running", desc),
            Error::InvalidEnvar(name) => write!(f, "Invalid envar: {}", name),
            Error::InvalidDaemonKind(kind) => write!(f, "Invalid daemon kind: {}", kind),
        }
    }
}
