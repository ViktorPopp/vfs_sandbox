use crate::{Vfs, VfsOps, Vnode, VnodeOps};

pub struct DummyFs;

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