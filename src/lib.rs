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

pub struct MountManager {
    mounts: Vec<Box<dyn Filesystem>>,
}

impl MountManager {
    pub fn new() -> Self {
        Self { mounts: Vec::new() }
    }

    pub fn mount(&mut self, fs: Box<dyn Filesystem>) {
        self.mounts.push(fs);
    }

    pub fn open(&self, path: String) -> Result<Vnode, String> {
        for fs in &self.mounts {
            if let Ok(node) = fs.open(path.clone()) {
                return Ok(node);
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
