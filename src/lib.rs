extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::RwLock;

pub mod mockfs;

#[derive(Debug)]
pub enum VfsError {
    MountPointAlreadyExists,
    NotFound,
}

#[derive(Clone)]
pub struct Vnode {
    pub vtype: VNodeType,
    pub ops: Arc<dyn VNodeOps + Send + Sync>,
}

#[derive(Clone, Debug)]
pub enum VNodeType {
    None,
    Directory,
    Regular,
    BlockDevice,
    CharDevice,
    SymbolicLink,
    Socket,
    Invalid,
}

pub trait VNodeOps {
    fn lookup(&self, directory: Vnode, name: String) -> Result<Vnode, VfsError>;
}

pub struct Vfs {
    mounts: RwLock<BTreeMap<String, Arc<Vnode>>>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            mounts: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn mount(&self, path: String, root: Arc<Vnode>) -> Result<(), VfsError> {
        let mut mounts = self.mounts.write();
        if mounts.contains_key(&path) {
            return Err(VfsError::MountPointAlreadyExists);
        }
        mounts.insert(path, root);
        Ok(())
    }

    pub fn findfs(&self, path: &str) -> Option<Arc<Vnode>> {
        let mounts = self.mounts.read();
        let mut best_match: Option<Arc<Vnode>> = None;
        let mut best_match_length = 0;

        for (mount_path, vnode) in mounts.iter() {
            if path.starts_with(mount_path) && mount_path.len() > best_match_length {
                best_match = Some(vnode.clone());
                best_match_length = mount_path.len();
            }
        }

        best_match
    }

    pub fn lookuppn(&self, path: &str) -> Result<Arc<Vnode>, VfsError> {
        let mut components = path.split('/').peekable();
        let mut current_vnode = if path.starts_with('/') {
            self.findfs("/") // Use the root vnode for absolute paths
        } else {
            self.findfs(components.next().unwrap_or(""))
        };

        while let Some(component) = components.next() {
            if component.is_empty() {
                continue; // Skip empty components (e.g., from "//")
            }
            if let Some(vnode) = current_vnode {
                current_vnode = Some(Arc::new(
                    vnode.ops.lookup((*vnode).clone(), component.to_string())?,
                ));
            } else {
                return Err(VfsError::NotFound);
            }
        }

        current_vnode.ok_or(VfsError::NotFound)
    }
}
