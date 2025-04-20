extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::RwLock;

pub mod fs;

pub trait FileSystem: Send + Sync {
    fn read(&self, path: &str) -> Option<String>;
    fn write(&self, path: &str, content: &str);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VNodeType {
    Directory,
    Regular,
}

pub struct VNode {
    pub path: String,
    pub node_type: VNodeType,
    pub fs: Arc<dyn FileSystem>,
}

pub struct VFS {
    mounts: RwLock<BTreeMap<String, Arc<dyn FileSystem>>>,
}

impl VFS {
    pub fn new() -> Self {
        VFS {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn mount(&self, mount_point: &str, fs: Arc<dyn FileSystem>) {
        self.mounts.write().insert(mount_point.to_string(), fs);
    }

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

    pub fn write(&self, path: &str, content: &str) {
        if let Some((fs, rel_path)) = self.find_fs(path) {
            fs.write(&rel_path, content);
        } else {
            println!("No filesystem mounted for path: {}", path);
        }
    }

    pub fn read(&self, path: &str) -> Option<String> {
        self.find_fs(path).and_then(|(fs, rel_path)| fs.read(&rel_path))
    }

    pub fn lookup_pn(&self, full_path: &str) -> Option<VNode> {
        let (fs, rel_path) = self.find_fs(full_path)?;
        let node_type = if rel_path.is_empty() || full_path.ends_with('/') {
            VNodeType::Directory
        } else {
            VNodeType::Regular
        };
    
        // If it's a directory, use the parent folder name.
        let display_path = if rel_path.is_empty() {
            // For the root directory of the mount, show the mount name (e.g., "mem1")
            full_path.strip_prefix("/").unwrap_or(full_path).to_string()
        } else {
            // Otherwise, show the relative path
            rel_path
        };
    
        Some(VNode {
            path: display_path,
            node_type,
            fs,
        })
    }
}
