pub mod mockfs;

#[derive(PartialEq, Clone)]
pub enum Type {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
    SymbolicLink,
    Socket,
}

pub struct MountPoint {
    path: String,
    fs: Box<dyn Filesystem>,
}

pub struct MountManager {
    mounts: Vec<MountPoint>,
}

impl MountManager {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn mount(&mut self, mount_path: String, fs: Box<dyn Filesystem>) {
        // Ensure mount path starts with /
        let path = if !mount_path.starts_with('/') {
            format!("/{}", mount_path)
        } else {
            mount_path
        };
        
        self.mounts.push(MountPoint { path, fs });
    }

    pub fn open(&self, path: String) -> Result<Vnode, String> {
        // Ensure path starts with /
        let path = if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.clone()
        };

        // Try each mount point, longest paths first to handle nested mounts
        for mount in self.mounts.iter().rev() {
            if path == mount.path || path.starts_with(&format!("{}/", mount.path)) {
                // Remove mount path prefix to get relative path within filesystem
                let relative_path = if path == mount.path {
                    "/".to_string()
                } else {
                    path[mount.path.len()..].to_string()
                };
                
                if let Ok(mut node) = mount.fs.open(relative_path) {
                    // Adjust node name to include full path
                    if path != "/" {
                        node.name = path;
                    }
                    return Ok(node);
                }
            }
        }
        Err("File not found".to_string())
    }
}

pub trait Filesystem {
    fn open(&self, path: String) -> Result<Vnode, String>;
    fn read(&self, node: &Vnode) -> Result<Vec<u8>, String>;
    fn close(&self, node: &Vnode) -> Result<(), String>;
}

pub struct Vnode {
    pub name: String,
    pub file_type: Type,
    pub size: u64,
    pub parent: Option<Box<Vnode>>,
    pub children: Option<Vec<Vnode>>,
    data: Vec<u8>, // Private Fs-specific data
}

impl Clone for Vnode {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            file_type: self.file_type.clone(),
            size: self.size,
            parent: self.parent.clone(),
            children: self.children.clone(),
            data: self.data.clone(),
        }
    }
}
