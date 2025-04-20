extern crate alloc;

use alloc::sync::Arc;
use vfs_sandbox::VFS;
use vfs_sandbox::fs::memfs::InMemoryFS;

fn main() {
    let vfs = VFS::new();

    // Create two different in-memory filesystems
    let mem_fs1 = Arc::new(InMemoryFS::default());
    let mem_fs2 = Arc::new(InMemoryFS::default());

    // Mount them at different paths
    vfs.mount("/mem1", mem_fs1.clone());
    vfs.mount("/mem1/mem2", mem_fs2.clone());

    // Write to each filesystem
    vfs.write("/mem1/file.txt", "Hello from mem1");
    vfs.write("/mem1/mem2/other.txt", "Greetings from mem2");

    // Read from each filesystem
    if let Some(content) = vfs.read("/mem1/file.txt") {
        println!("mem1: {}", content);
    }

    if let Some(content) = vfs.read("/mem1/mem2/other.txt") {
        println!("mem1/mem2: {}", content);
    }

    // Try reading a file not written
    if vfs.read("/mem1/missing.txt").is_none() {
        println!("mem1/missing.txt not found, as expected.");
    }

    // Write and read deeper paths
    vfs.write("/mem1/nested/data.txt", "Nested in mem1");
    if let Some(content) = vfs.read("/mem1/nested/data.txt") {
        println!("mem1 nested: {}", content);
    }
}
