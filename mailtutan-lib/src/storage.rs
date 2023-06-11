use crate::models::Message;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;

pub trait Storage: Sync + Send {
    fn list(&self) -> Vec<Message>;
    fn add(&mut self, message: Message) -> Message;
    fn get(&self, item: usize) -> &Message;
    fn size(&self) -> usize;
    fn delete_all(&mut self);
}

pub struct Connection {
    pub storage: Mutex<Box<dyn Storage + 'static>>,
    pub ws_sender: Sender<String>,
}

#[derive(Default)]
pub struct Memory {
    sequence_id: usize,
    messages_limit: usize,
    records: HashMap<usize, Message>,
}

impl Memory {
    pub fn new(capacity: usize) -> Self {
        Self {
            records: HashMap::new(),
            sequence_id: 1,
            messages_limit: capacity,
        }
    }
}

impl Storage for Memory {
    fn list(&self) -> Vec<Message> {
        let mut list: Vec<Message> = vec![];

        for record in self.records.values() {
            list.push(record.clone());
        }

        list
    }

    fn add(&mut self, mut message: Message) -> Message {
        message.id = Some(self.sequence_id);

        let clone = message.clone();

        self.records.insert(self.sequence_id, message);

        // delete oldest record
        if self.records.len() > self.messages_limit {
            self.records
                .remove(&(self.sequence_id - self.messages_limit))
                .unwrap();
        }

        self.sequence_id += 1;

        clone
    }

    fn get(&self, item: usize) -> &Message {
        self.records.get(&item).unwrap()
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.records.len()
    }

    fn delete_all(&mut self) {
        self.records.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;
    use super::Storage;
    use crate::models::Message;
    use std::assert_eq;

    #[test]
    fn test_store() {
        let mut store = Memory::new();

        store.add(Message {
            ..Default::default()
        });

        assert_eq!(store.size(), 1);

        store.delete_all();

        assert_eq!(store.size(), 0);
    }
}
