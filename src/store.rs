use crate::models::Message;

pub struct MemoryStore {
    sequence_id: usize,
    records: Vec<Message>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            records: vec![],
            sequence_id: 1,
        }
    }

    pub fn list(&self) -> &Vec<Message> {
        &self.records
    }

    pub fn add(&mut self, mut message: Message) {
        message.id = Some(self.sequence_id);
        self.sequence_id += 1;
        self.records.push(message);
    }

    pub fn get(&self, item: usize) -> &Message {
        &self.records.get(item - 1).unwrap()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.records.len()
    }

    pub fn delete_all(&mut self) {
        self.records.clear()
    }
}

#[cfg(test)]
mod test {
    use crate::models::Message;

    use super::MemoryStore;

    #[test]
    fn test_add_message() {
        let mut store = MemoryStore::new();
        store.records.len();
        store.add(Message {
            sender: "mosi".to_owned(),
            recipients: ["felan".to_owned()].to_vec(),
            subject: "something".to_owned(),
            ..Default::default()
        });

        assert_eq!(store.size(), 1);
        assert_eq!(store.list().len(), 1);
    }
}
