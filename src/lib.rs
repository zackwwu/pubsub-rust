mod broker;
mod models;

use broker::broker::Broker;
use models::{error, operation};
use std::{error::Error, result::Result, sync::mpsc};
use uuid::Uuid;

pub struct PubSub {
    sender: std::sync::mpsc::Sender<operation::Operation>,
}

impl PubSub {
    pub fn new() -> PubSub {
        let (broker, sender) = Broker::new();
        let _thread = std::thread::spawn(move || broker.run());
        PubSub { sender }
    }

    pub fn subscribe(&self) -> Result<(String, mpsc::Receiver<String>), Box<dyn Error>> {
        let id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::channel();
        match self.sender.send(operation::Operation::Sub {
            id: id.clone(),
            sender,
        }) {
            Ok(_) => Ok((id, receiver)),
            // improve error handling
            Err(e) => Err(Box::new(error::PubSubError::from(e))),
        }
    }

    pub fn publish(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        match self.sender.send(operation::Operation::Pub(msg.to_string())) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(error::PubSubError::from(e))),
        }
    }

    pub fn unsubscribe(&self, id: &str) -> Result<(), Box<dyn Error>> {
        match self
            .sender
            .send(operation::Operation::Unsub(id.to_string()))
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(error::PubSubError::from(e))),
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn Error>> {
        match self.sender.send(operation::Operation::Close) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(error::PubSubError::from(e))),
        }
    }
}

impl Drop for PubSub {
    fn drop(&mut self) {
        self.close().unwrap();
    }
}

impl Default for PubSub {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, sync::mpsc};

    #[test]
    fn test_pubsub() -> Result<(), Box<dyn Error>> {
        let pubsub = PubSub::new();
        let (id, receiver) = pubsub.subscribe()?;
        Uuid::parse_str(&id)?;
        pubsub.publish("hello")?;

        let msg = receiver.recv()?;
        assert_eq!(msg, "hello");

        Ok(())
    }

    #[test]
    fn test_unsub() -> Result<(), Box<dyn Error>> {
        let pubsub = PubSub::new();
        let (id, receiver) = pubsub.subscribe()?;
        Uuid::parse_str(&id)?;

        pubsub.unsubscribe(&id)?;
        pubsub.publish("hello")?;

        match receiver.recv() {
            Ok(_) => Err(Box::<dyn Error>::from(
                "Received message after unsubscribing",
            )),
            Err(_) => Ok(()),
        }
    }
}
