use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub output: String,
}

#[derive(Debug)]
pub enum Job {
    UserRequest(Request),
    BackgroundTask(String),
}
