use crate::socket::Request;

pub enum Action {
    DaemonStarted(String),
    DaemonComplete(String),
    DaemonDied(String),
    DaemonConfigWarning(String),
    Request(Request),
}
