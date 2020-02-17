pub mod load_yaml;

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Key {
    pub key: String,
    pub uses: AtomicUsize,
    pub partial: bool,
    pub origin: u8,
    pub trusted: u8,
}

impl Key {
    pub fn new(key: String, partial: bool, origin: u8) -> Self {
        Key {
            uses: AtomicUsize::new(0),
            key,
            partial,
            origin: origin,
            trusted: 0,
        }
    }
}

impl Clone for Key {
    fn clone(&self) -> Self {
        Self {
            uses: AtomicUsize::new(self.uses.load(Ordering::Relaxed)),
            key: self.key.clone(),
            partial: self.partial,
            origin: self.origin,
            trusted: self.trusted,
        }
    }
}
