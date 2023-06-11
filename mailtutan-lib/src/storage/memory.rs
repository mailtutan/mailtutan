use crate::models::Message;
use crate::Storage;
use std::collections::HashMap;

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

        self.records.insert(self.sequence_id, message.clone());

        if self.size() > self.messages_limit {
            let record_to_delete = self.sequence_id - self.messages_limit;
            self.remove(record_to_delete);
        }

        self.sequence_id += 1;

        message
    }

    fn get(&self, item: usize) -> Message {
        self.records.get(&item).unwrap().clone()
    }

    fn remove(&mut self, item: usize) {
        self.records.remove(&item).unwrap();
    }

    fn size(&self) -> usize {
        self.records.len()
    }

    fn delete_all(&mut self) {
        self.records.clear()
    }
}
