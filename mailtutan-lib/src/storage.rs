use crate::models::Message;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::str;
use std::str::FromStr;
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;

pub trait Storage: Sync + Send {
    fn list(&self) -> Vec<Message>;
    fn add(&mut self, message: Message) -> Message;
    fn get(&self, item: usize) -> Message;
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

        self.records.insert(self.sequence_id, message.clone());

        // delete oldest record
        if self.records.len() > self.messages_limit {
            self.records
                .remove(&(self.sequence_id - self.messages_limit))
                .unwrap();
        }

        self.sequence_id += 1;

        message
    }

    fn get(&self, item: usize) -> Message {
        self.records.get(&item).unwrap().clone()
    }

    #[allow(dead_code)]
    fn size(&self) -> usize {
        self.records.len()
    }

    fn delete_all(&mut self) {
        self.records.clear()
    }
}

pub struct Memdir {
    sequence_id: usize,
    messages_limit: usize,
    path: Box<Path>,
}

impl Memdir {
    pub fn new<T: AsRef<str>>(capacity: usize, path: T) -> Self {
        let path = Path::new(path.as_ref());

        if path.exists() {
            if !path.is_dir() {
                panic!("path exists, but it is not a dir");
            }
        } else {
            fs::create_dir(path).unwrap();
        }

        Self {
            sequence_id: Self::find_last_sequence_id(path) + 1,
            messages_limit: capacity,
            path: path.into(),
        }
    }

    fn find_last_sequence_id(path: &Path) -> usize {
        let mut last_sequence_id = 0;

        for path in fs::read_dir(path).unwrap() {
            let path = path.unwrap().path();
            let id = usize::from_str(path.file_stem().unwrap().to_str().unwrap()).unwrap();

            if id > last_sequence_id {
                last_sequence_id = id;
            }
        }

        last_sequence_id
    }
}

impl Storage for Memdir {
    fn list(&self) -> Vec<Message> {
        let mut list: Vec<Message> = vec![];

        for path in fs::read_dir(&self.path).unwrap() {
            let path = path.unwrap().path();
            let id = usize::from_str(path.file_stem().unwrap().to_str().unwrap()).unwrap();

            list.push(self.get(id));
        }

        list
    }

    fn add(&mut self, mut message: Message) -> Message {
        message.id = Some(self.sequence_id);

        let filename = format!("{}.eml", self.sequence_id);
        let mut file = fs::File::create(self.path.join(filename)).unwrap();
        file.write_all(&message.source).unwrap();

        self.sequence_id += 1;

        message
    }

    fn get(&self, item: usize) -> Message {
        let filename = format!("{}.eml", item);

        let bytes = fs::read(self.path.join(filename)).unwrap();

        let mut message = Message::from(&bytes);
        message.id = Some(item);

        message
    }

    fn size(&self) -> usize {
        fs::read_dir(&self.path).unwrap().count()
    }

    fn delete_all(&mut self) {
        for path in fs::read_dir(&self.path).unwrap() {
            let path = path.unwrap().path();
            if path.extension().unwrap() == "eml" {
                fs::remove_file(path).unwrap();
            }
        }
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
        let mut store = Memory::new(1000);

        store.add(Message {
            ..Default::default()
        });

        assert_eq!(store.size(), 1);

        store.delete_all();

        assert_eq!(store.size(), 0);
    }
}
