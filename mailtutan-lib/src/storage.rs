use crate::models::Message;

mod memdir;
mod memory;
pub use memdir::Memdir;
pub use memory::Memory;

pub trait Storage: Sync + Send {
    fn list(&self) -> Vec<Message>;
    fn add(&mut self, message: Message) -> Message;
    fn get(&self, item: usize) -> Option<Message>;
    fn remove(&mut self, item: usize);
    fn size(&self) -> usize;
    fn delete_all(&mut self);
}

#[cfg(test)]
mod tests {
    use super::Storage;
    use super::{Memdir, Memory};
    use crate::models::Message;
    use std::{assert_eq, fs};

    #[test]
    fn test_memory_store() {
        let mut store = Memory::new(1000);
        test_storage(&mut store);
    }

    #[test]
    fn test_memdir_store() {
        let dir = "./tmp";
        let mut store = Memdir::new(1000, dir);
        test_storage(&mut store);
        fs::remove_dir(dir).unwrap();
    }

    fn test_storage<T: Storage>(store: &mut T) {
        store.delete_all();
        let data = concat!(
            "From: Private Person <me@fromdomain.com>\n",
            "To: A Test User <test@todomain.com>\n",
            "Subject: Hello, Testing, Testing\n",
            "\n",
            "This is a test e-mail message.\n"
        )
        .as_bytes()
        .to_vec();

        let message = Message::from(&data).unwrap();
        let added = store.add(message);
        assert_eq!(store.size(), 1);
        let received = store.get(added.id.unwrap()).unwrap();
        assert_eq!(received.subject, "Hello, Testing, Testing");
        let received = &store.list()[0];
        assert_eq!(received.subject, "Hello, Testing, Testing");
        store.remove(received.id.unwrap());
        assert_eq!(store.size(), 0);
        assert!(store.get(42).is_none());
    }
}
