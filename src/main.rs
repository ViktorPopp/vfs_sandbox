extern crate alloc;

use alloc::sync::Arc;
use vfs_sandbox::fs::memfs::InMemoryFS;
use vfs_sandbox::*;

fn main() {
    let vfs = VFS::new();

    let root_fs = Arc::new(InMemoryFS::default());
    let nested_fs = Arc::new(InMemoryFS::default());

    vfs.mount("/root/", root_fs.clone());
    vfs.mount("/root/nested/", nested_fs.clone());

    println!("\n-- Writing Files --");

    // Writing through VNodes
    match vfs.lookuppn("/root/file.txt") {
        Ok(vnode) => {
            vnode.write("Hello from root").expect("Write failed");
            println!("Wrote to /root/file.txt");
        }
        Err(e) => println!("Error looking up /root/file.txt: {}", e),
    }

    match vfs.lookuppn("/root/nested/other.txt") {
        Ok(vnode) => {
            vnode.write("Greetings from nested").expect("Write failed");
            println!("Wrote to /root/nested/other.txt");
        }
        Err(e) => println!("Error looking up /root/nested/other.txt: {}", e),
    }

    println!("\n-- Reading Files --");

    match vfs.lookuppn("/root/file.txt") {
        Ok(vnode) => match vnode.read() {
            Ok(content) => println!("Read from root:   \t{}", content),
            Err(e) => println!("Error reading /root/file.txt: {}", e),
        },
        Err(e) => println!("Lookup failed: {}", e),
    }

    match vfs.lookuppn("/root/nested/other.txt") {
        Ok(vnode) => match vnode.read() {
            Ok(content) => println!("Read from nested:\t{}", content),
            Err(e) => println!("Error reading /root/nested/other.txt: {}", e),
        },
        Err(e) => println!("Lookup failed: {}", e),
    }

    println!("\n-- Directory Info --");

    match vfs.lookuppn("/root/") {
        Ok(vnode) => println!("VNode: {:?} \t\tType: {:?}", vnode.path, vnode.node_type),
        Err(e) => println!("Error looking up /root/: {}", e),
    }

    match vfs.lookuppn("/root/nested/other.txt") {
        Ok(vnode) => println!("VNode: {:?} \tType: {:?}", vnode.path, vnode.node_type),
        Err(e) => println!("Error looking up /root/nested/other.txt: {}", e),
    }
}
