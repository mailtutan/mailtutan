use crate::models::Message;

mod memdir;
mod memory;
pub use memdir::Memdir;
pub use memory::Memory;

pub trait Storage: Sync + Send {
    fn list(&self) -> Vec<Message>;
    fn add(&mut self, message: Message) -> Message;
    fn get(&self, item: usize) -> Message;
    fn remove(&mut self, item: usize);
    fn size(&self) -> usize;
    fn delete_all(&mut self);
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
