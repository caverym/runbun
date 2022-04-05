use std::path::PathBuf;

use crate::action::Action;

pub struct Request {
    pub send_to: PathBuf,
    pub action: ReqAction,
    pub damon: String,
}

pub enum ReqAction {
    Status,
    Start,
    Stop,
    Kill,
}

pub enum Response {
    Success,
    Failure,
}

pub struct Response(Option<String>);
