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

impl VNode {
    // Read content from the file
    pub fn read(&self) -> Option<String> {
        if self.node_type == VNodeType::Regular {
            self.fs.read(&self.path)
        } else {
            None // You can't read from a directory
        }
    }

    // Write content to the file
    pub fn write(&self, content: &str) {
        if self.node_type == VNodeType::Regular {
            self.fs.write(&self.path, content);
        }
    }
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

    pub fn lookuppn(&self, full_path: &str) -> Option<VNode> {
        let (fs, rel_path) = self.find_fs(full_path)?;
        let node_type = if rel_path.is_empty() || full_path.ends_with('/') {
            VNodeType::Directory
        } else {
            VNodeType::Regular
        };

        let display_path = if rel_path.is_empty() {
            full_path.strip_prefix("/").unwrap_or(full_path).to_string()
        } else {
            rel_path
        };

        Some(VNode {
            path: display_path,
            node_type,
            fs,
        })
    }
}
