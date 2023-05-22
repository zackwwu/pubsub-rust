mod broker;
mod models;
use broker::broker::Broker;
use models::operation;
use std::{result::Result, sync::mpsc};
use uuid::Uuid;

pub struct PubSub {
    sender: std::sync::mpsc::Sender<operation::Operation>,
}

impl PubSub {
    pub fn new() -> PubSub {
        let (broker, sender) = Broker::new();
        let _thread = std::thread::spawn(move || broker.run());
        return PubSub { sender: sender };
    }

    pub fn subscribe(&self) -> Result<(String, mpsc::Receiver<String>), String> {
        let id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::channel();
        match self.sender.send(operation::Operation::Sub {
            id: id.clone(),
            sender: sender,
        }) {
            Ok(_) => Ok((id, receiver)),
            // improve error handling
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn publish(&self, msg: &str) -> Result<(), String> {
        match self.sender.send(operation::Operation::Pub(msg.to_string())) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn unsubscribe(&self, id: &str) -> Result<(), String> {
        match self
            .sender
            .send(operation::Operation::Unsub(id.to_string()))
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn close(&self) -> Result<(), String> {
        match self.sender.send(operation::Operation::Close) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Drop for PubSub {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}
