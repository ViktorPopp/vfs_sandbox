use std::sync::Arc;

use crate::{Vfs, Vnode, VfsOps, VnodeOps, VnodeType};

pub struct DummyFs;

impl VfsOps for DummyFs {
    fn mount(&self, _vfs: Arc<std::sync::Mutex<Vfs>>, _path: String) {
        // Dummy implementation: does nothing
    }

    fn unmount(&self, _vfs: Arc<std::sync::Mutex<Vfs>>) {
        // Dummy implementation: does nothing
    }
}

pub struct DummyVnodeOps;

impl VnodeOps for DummyVnodeOps {
    fn lookup(&self, _directory: Vnode, _name: String) -> Result<Vnode, String> {
        Err("DummyVnodeOps: lookup not implemented".to_string())
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DUMMY_FS_OPS: Arc<dyn VfsOps + Send + Sync> = Arc::new(DummyFs);
    pub static ref DUMMY_VNODE_OPS: Arc<dyn VnodeOps + Send + Sync> = Arc::new(DummyVnodeOps);
}

pub fn init(root_vfs: &Arc<std::sync::Mutex<Vfs>>) -> usize {
    let mut index = 0;
    let mut curr = Arc::clone(root_vfs);

    loop {
        let next = {
            let guard = curr.lock().unwrap();
            guard.next.clone()
        };

        match next {
            Some(next_vfs) => {
                curr = next_vfs;
                index += 1;
            }
            None => break,
        }
    }

    let root_vnode = Vnode {
        vfs_pointer: None,
        vfs_mounted_here: None,
        ops: Arc::clone(&DUMMY_VNODE_OPS),
        vtype: VnodeType::Directory,
        fs_data: None,
    };

    let dummy_vfs = Arc::new(std::sync::Mutex::new(Vfs {
        next: None,
        vnode_pointer: Some(Arc::new(std::sync::Mutex::new(root_vnode))),
        ops: Some(Arc::clone(&DUMMY_FS_OPS)),
        fs_data: None,
    }));

    {
        let mut curr_guard = curr.lock().unwrap();
        curr_guard.next = Some(dummy_vfs);
        index += 1;
    }

    index
}
