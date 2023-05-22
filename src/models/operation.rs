use std::sync::mpsc;

pub enum Operation {
    Sub {
        id: String,
        sender: mpsc::Sender<String>,
    },
    Unsub(String),
    Pub(String),
    Close,
}
