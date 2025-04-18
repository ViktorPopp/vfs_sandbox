use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

pub mod fs;
pub mod utils;

lazy_static! {
    pub static ref ROOT_VFS: Arc<Mutex<Vfs>> = Arc::new(Mutex::new(Vfs::new()));
}

#[derive(Debug, Clone)]
pub enum VnodeType {
    None,
    Regular,
    Directory,
}

pub trait VfsOps: Send + Sync {
    fn mount(&self, vfs: Arc<Mutex<Vfs>>, path: String);
    fn unmount(&self, vfs: Arc<Mutex<Vfs>>);
}

pub trait VnodeOps: Send + Sync {
    fn lookup(&self, directory: Vnode, name: String) -> Result<Vnode, String>;
}

pub struct Vnode {
    pub vfs_pointer: Option<Arc<Mutex<Vfs>>>,
    pub vfs_mounted_here: Option<Arc<Mutex<Vfs>>>,
    pub ops: Arc<dyn VnodeOps + Send + Sync>,

    pub vtype: VnodeType,

    pub fs_data: Option<Box<dyn utils::AnyClone + Send + Sync>>,
}

pub struct Vfs {
    pub next: Option<Arc<Mutex<Vfs>>>,
    pub vnode_pointer: Option<Arc<Mutex<Vnode>>>,

    pub ops: Option<Arc<dyn VfsOps + Send + Sync>>,

    pub fs_data: Option<Box<dyn utils::AnyClone + Send + Sync>>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            next: None,
            vnode_pointer: None,
            ops: None,
            fs_data: None,
        }
    }

    pub fn from(ops: Arc<dyn VfsOps + Send + Sync>) -> Self {
        Vfs {
            next: None,
            vnode_pointer: None,
            ops: Some(ops),
            fs_data: None,
        }
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}
