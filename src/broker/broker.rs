use crate::models::operation::Operation;
use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct Broker {
    subs: HashMap<String, mpsc::Sender<String>>,
    receiver: Arc<Mutex<mpsc::Receiver<Operation>>>,
}

impl Broker {
    pub fn new() -> (Broker, mpsc::Sender<Operation>) {
        let (sender, receiver) = mpsc::channel();
        (
            Broker {
                subs: HashMap::new(),
                receiver: Arc::new(Mutex::new(receiver)),
            },
            sender,
        )
    }

    pub fn run(mut self) {
        let receiver = Arc::clone(&self.receiver);
        let _thread = thread::spawn(move || loop {
            if let Ok(operation) = receiver.lock().unwrap().recv() {
                match operation {
                    Operation::Sub { id, sender } => self.subscribe(id, sender),
                    Operation::Unsub(id) => self.unsubscribe(&id),
                    Operation::Pub(msg) => self.publish(&msg),
                    Operation::Close => self.close(),
                }
            }
        });
    }

    fn subscribe(&mut self, id: String, sender: mpsc::Sender<String>) {
        self.subs.insert(id, sender);
    }

    fn publish(&mut self, msg: &str) {
        let mut subs = self.subs.clone();
        subs = subs
            .into_iter()
            .filter_map(|(id, sender)| match sender.send(msg.to_string()) {
                Ok(_) => Some((id, sender)),
                Err(_) => None,
            })
            .collect();
        self.subs = subs;
    }

    fn unsubscribe(&mut self, id: &str) {
        self.subs.remove(id);
    }

    fn close(&mut self) {
        for (_, sender) in self.subs.drain() {
            drop(sender);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, sync::mpsc};

    #[test]
    fn test_broker_pubsub() -> Result<(), Box<dyn Error>> {
        let (broker, op_sender) = Broker::new();
        broker.run();

        let (sender, receiver) = mpsc::channel();

        op_sender.send(Operation::Sub {
            id: "1".to_string(),
            sender: sender,
        })?;

        op_sender.send(Operation::Pub("hello".to_string()))?;

        if let Ok(msg) = receiver.recv() {
            assert_eq!(msg, "hello");
        }
        Ok(())
    }

    #[test]
    fn test_broker_unsub() -> Result<(), Box<dyn Error>> {
        let (broker, op_sender) = Broker::new();
        broker.run();

        let (sender, receiver) = mpsc::channel();

        op_sender.send(Operation::Sub {
            id: "1".to_string(),
            sender: sender,
        })?;

        op_sender.send(Operation::Unsub("1".to_string()))?;

        op_sender.send(Operation::Pub("hello".to_string()))?;

        match receiver.recv() {
            Ok(_) => Err(Box::<dyn Error>::from(
                "Received message after unsubscribing",
            )),
            Err(_) => Ok(()),
        }
    }
}
