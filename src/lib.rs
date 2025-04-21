extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt;
use spin::RwLock;

pub mod fs;

/// Common errors that might occur in the VFS.
#[derive(Debug)]
pub enum VFSError {
    NotFound,
    PermissionDenied,
    InvalidOperation,
    AlreadyExists,
    Other(String),
}

impl fmt::Display for VFSError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VFSError::NotFound => write!(f, "Not Found"),
            VFSError::PermissionDenied => write!(f, "Permission Denied"),
            VFSError::InvalidOperation => write!(f, "Invalid Operation"),
            VFSError::AlreadyExists => write!(f, "Already Exists"),
            VFSError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

/// Operations a VNode can perform, implemented per filesystem.
pub trait VNodeOps: Send + Sync {
    fn read(&self, path: &str) -> Result<String, VFSError>;
    fn write(&self, path: &str, content: &str) -> Result<(), VFSError>;
    fn open(&self, path: &str) -> Result<(), VFSError>;
    fn close(&self, path: &str) -> Result<(), VFSError>;
    fn exists(&self, path: &str) -> Result<bool, VFSError>;
    fn remove(&self, path: &str) -> Result<(), VFSError>;
    fn list_dir(&self, path: &str) -> Result<Vec<String>, VFSError>;
}

/// Type of the node: file or directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VNodeType {
    Directory,
    Regular,
}

/// Represents a single file or directory node.
pub struct VNode {
    pub path: String,
    pub node_type: VNodeType,
    pub fs: Arc<dyn VNodeOps>,
}

impl VNode {
    pub fn read(&self) -> Result<String, VFSError> {
        if self.node_type == VNodeType::Regular {
            self.fs.read(&self.path)
        } else {
            Err(VFSError::InvalidOperation)
        }
    }

    pub fn write(&self, content: &str) -> Result<(), VFSError> {
        if self.node_type == VNodeType::Regular {
            self.fs.write(&self.path, content)
        } else {
            Err(VFSError::InvalidOperation)
        }
    }

    pub fn open(&self) -> Result<(), VFSError> {
        self.fs.open(&self.path)
    }

    pub fn close(&self) -> Result<(), VFSError> {
        self.fs.close(&self.path)
    }

    pub fn exists(&self) -> Result<bool, VFSError> {
        self.fs.exists(&self.path)
    }

    pub fn remove(&self) -> Result<(), VFSError> {
        self.fs.remove(&self.path)
    }

    pub fn list_dir(&self) -> Result<Vec<String>, VFSError> {
        if self.node_type == VNodeType::Directory {
            self.fs.list_dir(&self.path)
        } else {
            Err(VFSError::InvalidOperation)
        }
    }
}

/// Virtual file system structure.
pub struct VFS {
    mounts: RwLock<BTreeMap<String, Arc<dyn VNodeOps>>>,
}

impl VFS {
    pub fn new() -> Self {
        VFS {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn mount(&self, mount_point: &str, fs: Arc<dyn VNodeOps>) {
        self.mounts.write().insert(String::from(mount_point), fs);
    }

    pub fn unmount(&self, mount_point: &str) -> Result<(), VFSError> {
        let mut mounts = self.mounts.write();
        if mounts.remove(mount_point).is_some() {
            Ok(())
        } else {
            Err(VFSError::NotFound)
        }
    }

    fn find_fs(&self, path: &str) -> Result<(Arc<dyn VNodeOps>, String), VFSError> {
        let mounts = self.mounts.read();
        let mut best_match: Option<(&str, &Arc<dyn VNodeOps>)> = None;

        for (key, fs) in mounts.iter() {
            if path.starts_with(key) {
                match best_match {
                    Some((best_key, _)) if key.len() <= best_key.len() => {}
                    _ => best_match = Some((key.as_str(), fs)),
                }
            }
        }

        if let Some((key, fs)) = best_match {
            let relative_path = path.strip_prefix(key).unwrap_or("").trim_start_matches('/');
            Ok((fs.clone(), relative_path.to_string()))
        } else {
            Err(VFSError::NotFound)
        }
    }

    pub fn lookuppn(&self, full_path: &str) -> Result<VNode, VFSError> {
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

        Ok(VNode {
            path: display_path.to_string(),
            node_type,
            fs,
        })
    }
}
