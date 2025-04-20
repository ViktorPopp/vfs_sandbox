use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use spin::RwLock;
use crate::FileSystem;

#[derive(Default)]
pub struct InMemoryFS {
    files: RwLock<BTreeMap<String, String>>,
}

impl FileSystem for InMemoryFS {
    fn read(&self, path: &str) -> Option<String> {
        self.files.read().get(path).cloned()
    }

    fn write(&self, path: &str, content: &str) {
        self.files.write().insert(path.to_string(), content.to_string());
    }
}
