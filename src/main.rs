extern crate alloc;

use alloc::sync::Arc;

use vfs_sandbox::mockfs::*;
use vfs_sandbox::*;

fn main() {
    let vfs = Vfs::new();

    // Create root vnode
    let root_vnode = Arc::new(Vnode {
        vtype: VNodeType::Directory,
        ops: Arc::new(MockFs),
    });

    // Mount root
    vfs.mount("/".to_string(), root_vnode)
        .expect("Mount failed");

    // Lookup paths
    match vfs.lookuppn("/home") {
        Ok(vnode) => println!("Found /home: {:?}", vnode.vtype),
        Err(err) => println!("Error: {:?}", err),
    }

    match vfs.lookuppn("/home/file.txt") {
        Ok(vnode) => println!("Found /home/file.txt: {:?}", vnode.vtype),
        Err(err) => println!("Error: {:?}", err),
    }

    match vfs.lookuppn("/nonexistent") {
        Ok(vnode) => println!("Found /nonexistent: {:?}", vnode.vtype),
        Err(err) => println!("Error: {:?}", err),
    }
}
