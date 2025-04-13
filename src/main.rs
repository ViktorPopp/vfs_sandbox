use vfs_sandbox::mockfs;

fn main() {
    // Create a new mount manager
    let mut mount_manager = vfs_sandbox::MountManager::new();
    
    // Create and mount an in-memory filesystem
    let fs = Box::new(mockfs::InMemoryFilesystem::new());
    mount_manager.mount(fs);
    
    // Try to open the root directory
    match mount_manager.open("/".to_string()) {
        Ok(node) => {
            println!("Successfully opened root directory: {}", node.name);
        },
        Err(e) => {
            println!("Error opening root directory: {}", e);
        }
    }
    
    // Try to open a non-existent file
    match mount_manager.open("/nonexistent.txt".to_string()) {
        Ok(_) => {
            println!("Unexpected: File exists!");
        },
        Err(e) => {
            println!("Expected error: {}", e);
        }
    }
}
