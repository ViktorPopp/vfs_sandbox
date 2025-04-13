use vfs_sandbox::mockfs;

fn main() {
    // Create a new mount manager
    let mut mount_manager = vfs_sandbox::MountManager::new();
    
    // Create and mount an in-memory filesystem at /dev
    let fs1 = Box::new(mockfs::InMemoryFilesystem::new());
    mount_manager.mount("/dev".to_string(), fs1);
    
    // Create and mount another in-memory filesystem at /mnt/main
    let fs2 = Box::new(mockfs::InMemoryFilesystem::new());
    mount_manager.mount("/mnt/main".to_string(), fs2);
    
    // Try to open the mounted directories
    match mount_manager.open("/dev".to_string()) {
        Ok(node) => println!("Successfully opened /dev: {}", node.name),
        Err(e) => println!("Error opening /dev: {}", e),
    }
    
    match mount_manager.open("/mnt/main".to_string()) {
        Ok(node) => println!("Successfully opened /mnt/main: {}", node.name),
        Err(e) => println!("Error opening /mnt/main: {}", e),
    }
}
