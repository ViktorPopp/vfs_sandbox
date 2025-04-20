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
    pub fn read(&self) -> Option<String> {
        if self.node_type == VNodeType::Regular {
            self.fs.read(&self.path)
        } else {
            None
        }
    }

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
        self.mounts.write().insert(String::from(mount_point), fs);
    }

    fn find_fs(&self, path: &str) -> Option<(Arc<dyn FileSystem>, String)> {
        let mounts = self.mounts.read();
        let mut best_match: Option<(&str, &Arc<dyn FileSystem>)> = None;

        for (key, fs) in mounts.iter() {
            if path.starts_with(key) {
                match best_match {
                    Some((best_key, _)) if key.len() <= best_key.len() => {}
                    _ => best_match = Some((key.as_str(), fs)),
                }
            }
        }

        if let Some((key, fs)) = best_match {
            let relative_path = path
                .strip_prefix(key)
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
            full_path.strip_prefix('/').unwrap_or(full_path)
        } else {
            &rel_path
        };

        Some(VNode {
            path: display_path.to_string(),
            node_type,
            fs,
        })
    }
}
