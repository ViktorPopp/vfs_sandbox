extern crate alloc;

use alloc::sync::Arc;
use vfs_sandbox::VFS;
use vfs_sandbox::fs::memfs::InMemoryFS;

fn main() {
    let vfs = VFS::new();

    let root_fs = Arc::new(InMemoryFS::default());
    let nested_fs = Arc::new(InMemoryFS::default());

    vfs.mount("/root/", root_fs.clone());
    vfs.mount("/root/nested/", nested_fs.clone());

    // Writing through VNodes
    if let Some(vnode) = vfs.lookuppn("/root/file.txt") {
        vnode.write("Hello from root");
    }

    if let Some(vnode) = vfs.lookuppn("/root/nested/other.txt") {
        vnode.write("Greetings from nested");
    }

    print!("\n");

    // Reading through VNodes
    if let Some(vnode) = vfs.lookuppn("/root/file.txt") {
        if let Some(content) = vnode.read() {
            println!("Read from root: \t\t{}", content);
        }
    }

    if let Some(vnode) = vfs.lookuppn("/root/nested/other.txt") {
        if let Some(content) = vnode.read() {
            println!("Read from nested: \t\t{}", content);
        }
    }

    print!("\n");

    // Lookup directories and files
    if let Some(vnode) = vfs.lookuppn("/root/") {
        println!("VNode: {:?}, \t\tType: {:?}", vnode.path, vnode.node_type);
    }

    if let Some(vnode) = vfs.lookuppn("/root/nested/other.txt") {
        println!("VNode: {:?}, \t\tType: {:?}", vnode.path, vnode.node_type);
    }
}
