use lazy_static::lazy_static;

pub mod fs;
pub mod utils;

lazy_static! {
    static ref ROOT_VFS: Box<Vfs> = Box::new(Vfs::new());
}

#[derive(Debug, Clone)]
pub enum VnodeType {
    None,
    Regular,
    Directory,
}

pub trait VfsOps: Send + Sync {
    fn mount(&self, vfs: Box<Vfs>, path: String);
    fn unmount(&self, vfs: Box<Vfs>);
}

pub trait VnodeOps: Send + Sync {
    fn lookup(&self, directory: Vnode, name: String) -> Result<Vnode, String>;
}

pub struct Vnode {
    pub vfs_pointer: Box<Vfs>,
    pub vfs_mounted_here: Option<Box<Vfs>>,
    pub ops: Box<dyn VnodeOps + Send + Sync>,

    pub vtype: VnodeType,

    pub fs_data: Box<dyn utils::AnyClone + Send + Sync>,
}

pub struct Vfs {
    pub next: Option<Box<Vfs>>,
    pub covers: Option<Box<Vnode>>,

    pub ops: Option<Box<dyn VfsOps + Send + Sync>>,

    pub fs_data: Option<Box<dyn utils::AnyClone + Send + Sync>>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            next: None,
            covers: None,
            ops: None,
            fs_data: None,
        }
    }

    pub fn from(ops: Box<dyn VfsOps + Send + Sync>) -> Self {
        Vfs {
            next: None,
            covers: None,
            ops: Some(ops),
            fs_data: None,
        }
    }
}
