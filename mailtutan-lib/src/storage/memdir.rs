use crate::models::Message;
use crate::Storage;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

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

        if self.size() > self.messages_limit {
            let record_to_delete = self.sequence_id - self.messages_limit;
            self.remove(record_to_delete);
        }

        self.sequence_id += 1;

        message
    }

    fn get(&self, item: usize) -> Message {
        let filename = format!("{}.eml", item);

        let bytes = fs::read(self.path.join(filename)).unwrap();

        let mut message = Message::from(&bytes).unwrap();
        message.id = Some(item);

        message
    }

    fn remove(&mut self, item: usize) {
        let filename = format!("{}.eml", item);

        let path = self.path.join(filename);
        fs::remove_file(path).unwrap();
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
