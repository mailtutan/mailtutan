use crate::models::{Message, MessageEvent};

pub trait Storage {
    fn list(&self) -> &Vec<Message>;
    fn add(&mut self, message: Message) -> usize;
    fn get(&self, item: usize) -> &Message;
    fn size(&self) -> usize;
    fn delete_all(&mut self);
}

pub struct Memory {
    sequence_id: usize,
    records: Vec<Message>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            records: vec![],
            sequence_id: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::Message, store::MemoryStore};
    use std::assert_eq;

    #[test]
    fn test_store() {
        let mut store = MemoryStore::new();
        store.add(Message {
            ..Default::default()
        });

        assert_eq!(store.size(), 1);

        store.delete_all();

        assert_eq!(store.size(), 0);
    }
}
