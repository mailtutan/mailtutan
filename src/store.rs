use crate::models::message::Message;

pub struct MemoryStore {
    records: Vec<Message>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore { records: vec![] }
    }

    pub fn list(&self) -> &Vec<Message> {
        &self.records
    }

    pub fn add(&mut self, message: Message) {
        self.records.push(message);
    }

    pub fn size(&self) -> usize {
        self.records.len()
    }
}

#[cfg(test)]
mod test {
    use crate::models::message::Message;

    use super::{MemoryStore, Store};

    #[test]
    fn test_add_message() {
        let mut store = MemoryStore::new();
        store.records.len();
        store.add(Message {
            from: "mosi".to_owned(),
            to: "tala".to_owned(),
            data: "something".to_owned(),
        });

        assert_eq!(store.size(), 1);
        assert_eq!(store.list().len(), 1);
    }
}
