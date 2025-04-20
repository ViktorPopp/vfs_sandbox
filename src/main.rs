extern crate alloc;

use alloc::sync::Arc;
use vfs_sandbox::VFS;
use vfs_sandbox::fs::memfs::InMemoryFS;

fn main() {
    let vfs = VFS::new();

    let root_fs = Arc::new(InMemoryFS::default());
    let nested_fs = Arc::new(InMemoryFS::default());

    vfs.mount("/root", root_fs.clone());
    vfs.mount("/root/nested", nested_fs.clone());

    vfs.write("/root/file.txt", "Hello from root");
    vfs.write("/root/nested/other.txt", "Greetings from nested");

    if let Some(content) = vfs.read("/root/file.txt") {
        println!("Read root: {}", content);
    }

    if let Some(content) = vfs.read("/root/nested/other.txt") {
        println!("Read nested: {}", content);
    }

    if let Some(vnode) = vfs.lookup_pn("/root/file.txt") {
        println!("VNode: {:?}, Type: {:?}", vnode.path, vnode.node_type);
    }

    if let Some(vnode) = vfs.lookup_pn("/root/") {
        println!("VNode: {:?}, Type: {:?}", vnode.path, vnode.node_type);
    }

    if let Some(vnode) = vfs.lookup_pn("/root/nested/other.txt") {
        println!("VNode: {:?}, Type: {:?}", vnode.path, vnode.node_type);
    }
}
