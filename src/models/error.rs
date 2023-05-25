use crate::operation::Operation;
use std::sync::mpsc;

#[derive(Debug)]
pub struct PubSubError {
    message: String,
    cause: Box<dyn std::error::Error>,
}

impl std::error::Error for PubSubError {}

impl std::fmt::Display for PubSubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PubSubError: {}, caused by {}", self.message, self.cause)
    }
}

impl From<mpsc::SendError<Operation>> for PubSubError {
    fn from(error: mpsc::SendError<Operation>) -> Self {
        PubSubError {
            message: "Failed to send operation to broker".to_string(),
            cause: Box::new(error),
        }
    }
}

impl PubSubError {
    #[allow(dead_code)]
    pub fn new(message: String, cause: Box<dyn std::error::Error>) -> Self {
        PubSubError { message, cause }
    }
}
