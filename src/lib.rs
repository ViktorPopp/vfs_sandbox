extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::RwLock;

pub mod fs;

/// Trait representing a filesystem
pub trait FileSystem: Send + Sync {
    fn read(&self, path: &str) -> Option<String>;
    fn write(&self, path: &str, content: &str);
}

/// The Virtual Filesystem
pub struct VFS {
    mounts: RwLock<BTreeMap<String, Arc<dyn FileSystem>>>,
}

impl VFS {
    pub fn new() -> Self {
        VFS {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }

    /// Mount a filesystem at a given path
    pub fn mount(&self, mount_point: &str, fs: Arc<dyn FileSystem>) {
        self.mounts.write().insert(mount_point.to_string(), fs);
    }

    /// Find the best mount point match for the given path
    fn find_fs(&self, path: &str) -> Option<(Arc<dyn FileSystem>, String)> {
        let mounts = self.mounts.read();
        let mut best_match = "";

        for key in mounts.keys() {
            if path.starts_with(key) && key.len() > best_match.len() {
                best_match = key;
            }
        }

        if let Some(fs) = mounts.get(best_match) {
            let relative_path = path
                .strip_prefix(best_match)
                .unwrap_or("")
                .trim_start_matches('/');
            Some((fs.clone(), relative_path.to_string()))
        } else {
            None
        }
    }

    pub fn read(&self, path: &str) -> Option<String> {
        self.find_fs(path).and_then(|(fs, rel_path)| fs.read(&rel_path))
    }

    pub fn write(&self, path: &str, content: &str) {
        if let Some((fs, rel_path)) = self.find_fs(path) {
            fs.write(&rel_path, content);
        } else {
            println!("No filesystem mounted for path: {}", path);
        }
    }
}
