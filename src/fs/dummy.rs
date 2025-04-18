use crate::{ROOT_VFS, Vfs, VfsOps, Vnode, VnodeOps};

pub struct DummyFs {
    pub base: Vfs,
    pub root: Vnode,
}

impl VfsOps for DummyFs {
    fn mount(&self, _vfs: Box<Vfs>, _path: String) {
        println!("Mounted dummy FS");
    }

    fn unmount(&self, _vfs: Box<Vfs>) {
        println!("Unmounted dummy FS");
    }
}

pub struct DummyNode;

impl VnodeOps for DummyNode {
    fn lookup(&self, _directory: Vnode, _name: String) -> Result<Vnode, String> {
        Err("Not found in dummy FS".to_string())
    }
}

impl DummyFs {
    pub fn init() {
        let mut index: i32 = 0;
        let mut current_vfs: &Box<Vfs> = &ROOT_VFS;

        while let Some(ref next) = current_vfs.next {
            current_vfs = next;
            index += 1;
        }

        let mut dummy_fs: Box<DummyFs> = Box::new(DummyFs::new());

        dummy_fs.as_mut().base.vnode_pointer = None;

        println!("Initialized DummyFS:");
        println!("\t- index: {}", index)
    }

    pub fn new() -> Self {
        DummyFs {
            base: Vfs {
                ops: None,
                vnode_pointer: None,
                next: None,
                fs_data: None,
            },
            root: Vnode {
                ops: Box::new(DummyNode),
                vfs_mounted_here: None,
                vfs_pointer: None,
                fs_data: None,
                vtype: crate::VnodeType::None,
            },
        }
    }
}
