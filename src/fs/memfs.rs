use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::RwLock;

use crate::{VFSError, VNodeOps};

#[derive(Default)]
pub struct InMemoryFS {
    files: RwLock<BTreeMap<String, String>>,
}

impl VNodeOps for InMemoryFS {
    fn read(&self, path: &str) -> Result<String, VFSError> {
        let files = self.files.read();
        files.get(path).cloned().ok_or(VFSError::NotFound)
    }

    fn write(&self, path: &str, content: &str) -> Result<(), VFSError> {
        self.files
            .write()
            .insert(path.to_string(), content.to_string());
        Ok(())
    }

    fn open(&self, _path: &str) -> Result<(), VFSError> {
        // No-op for in-memory FS
        Ok(())
    }

    fn close(&self, _path: &str) -> Result<(), VFSError> {
        // No-op for in-memory FS
        Ok(())
    }

    fn exists(&self, path: &str) -> Result<bool, VFSError> {
        let files = self.files.read();
        Ok(files.contains_key(path))
    }

    fn remove(&self, path: &str) -> Result<(), VFSError> {
        let mut files = self.files.write();
        if files.remove(path).is_some() {
            Ok(())
        } else {
            Err(VFSError::NotFound)
        }
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>, VFSError> {
        let files = self.files.read();
        let mut results = Vec::new();
        let prefix = if path.is_empty() {
            "".to_string()
        } else {
            format!("{}/", path.trim_end_matches('/'))
        };

        for key in files.keys() {
            if key.starts_with(&prefix) {
                let trimmed = key.strip_prefix(&prefix).unwrap_or(key);
                if !trimmed.contains('/') {
                    results.push(trimmed.to_string());
                }
            }
        }

        Ok(results)
    }
}
