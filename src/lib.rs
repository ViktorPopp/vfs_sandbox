use spin::Mutex;

pub mod utils;

static ROOT_VFS: Mutex<Option<Box<Vfs>>> = Mutex::new(None);

pub fn init(vfs: Box<Vfs>) {
    let mut root_vfs = ROOT_VFS.lock();
    *root_vfs = Some(vfs);
}

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

    pub ops: Box<dyn VfsOps + Send + Sync>,
}
