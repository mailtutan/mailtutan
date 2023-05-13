use crate::models::message::Message;

pub struct MemoryStore {
    records: Vec<Message>,
}

trait Store {
    fn list(&self) -> &Vec<Message>;
    fn new() -> Self;
    fn add(&mut self, message: Message);
    fn size(&self) -> usize;
}

impl Store for MemoryStore {
    fn new() -> Self {
        MemoryStore { records: vec![] }
    }
    fn list(&self) -> &Vec<Message> {
        &self.records
    }

    fn add(&mut self, message: Message) {
        self.records.push(message);
    }

    fn size(&self) -> usize {
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
