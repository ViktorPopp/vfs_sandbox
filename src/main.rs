use std::sync::{Arc, Mutex};
use vfs_sandbox::*;
use crate::Vfs;
use crate::fs::dummy::init;

use lazy_static::lazy_static;

lazy_static! {
    static ref ROOT_VFS: Arc<Mutex<Vfs>> = Arc::new(Mutex::new(Vfs::new()));
}

fn main() {
    let index = init(&ROOT_VFS);
    println!("DummyFS mounted at index {} in VFS chain", index);

    let mut curr = Arc::clone(&ROOT_VFS);
    let mut i = 0;
    loop {
        let next = {
            let guard = curr.lock().unwrap();
            println!("VFS at index {} -> vnode present: {}", i, guard.vnode_pointer.is_some());
            guard.next.clone()
        };

        match next {
            Some(next_vfs) => {
                curr = next_vfs;
                i += 1;
            }
            None => break,
        }
    }
}
