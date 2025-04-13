use std::collections::HashMap;

#[derive(Clone)]
enum Type {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
    SymbolicLink,
    Socket,
}

#[derive(PartialEq, Clone)]
enum OpenMode {
    Read,
    Write,
    Append,
    Create,
}

struct MountManager {
    mounts: HashMap<String, Box<dyn Filesystem>>,
}

impl MountManager {
    fn new() -> Self {
        Self {
            mounts: HashMap::new(),
        }
    }

    fn mount(&mut self, path: String, fs: Box<dyn Filesystem>) {
        self.mounts.insert(path, fs);
    }

    fn unmount(&mut self, path: &str) {
        self.mounts.remove(path);
    }

    fn get_filesystem(&self, path: &str) -> Option<&Box<dyn Filesystem>> {
        self.mounts.get(path)
    }
}

trait Filesystem {
    fn open(&mut self, path: String, mode: OpenMode) -> Result<Vnode, String>;
    fn read(&self, node: &Vnode) -> Result<Vec<u8>, String>;
    fn write(&mut self, node: &mut Vnode, data: Vec<u8>) -> Result<(), String>;
    fn close(&mut self, node: &Vnode) -> Result<(), String>;
}

#[derive(Clone)]
struct Vnode {
    pub name: String,
    pub mode: OpenMode,
    pub file_type: Type,
    pub size: u64,
    pub parent: Option<Box<Vnode>>,
    pub children: Option<Vec<Vnode>>,
    pub content: Option<Vec<u8>>, // For storing file content
}

struct InMemoryFilesystem {
    root: Vnode,
}

impl InMemoryFilesystem {
    fn new() -> Self {
        Self {
            root: Vnode {
                name: "/".to_string(),
                mode: OpenMode::Read,
                file_type: Type::Directory,
                size: 0,
                parent: None,
                children: Some(vec![]),
                content: None,
            },
        }
    }

    fn find_node(&self, path: &str) -> Option<&Vnode> {
        let mut current = &self.root;
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        for part in parts {
            if let Some(children) = &current.children {
                if let Some(child) = children.iter().find(|c| c.name == part) {
                    current = child;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(current)
    }

    fn find_node_mut(&mut self, path: &str) -> Option<&mut Vnode> {
        let mut current = &mut self.root;
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        for part in parts {
            if let Some(children) = &mut current.children {
                if let Some(child) = children.iter_mut().find(|c| c.name == part) {
                    current = child;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }

        Some(current)
    }
}

impl Filesystem for InMemoryFilesystem {
    fn open(&mut self, path: String, mode: OpenMode) -> Result<Vnode, String> {
        if let Some(node) = self.find_node(&path) {
            Ok((*node).clone())
        } else if mode == OpenMode::Create {
            let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
            let file_name = parts.last().ok_or("Invalid path")?;
            let parent_path = &path[..path.len() - file_name.len() - 1];
            let parent = self.find_node_mut(parent_path).ok_or("Parent directory not found")?;

            if let Some(children) = &mut parent.children {
                let new_node = Vnode {
                    name: file_name.to_string(),
                    mode: OpenMode::Read,
                    file_type: Type::Regular,
                    size: 0,
                    parent: None,
                    children: None,
                    content: Some(vec![]),
                };
                children.push(new_node.clone());
                Ok(new_node)
            } else {
                Err("Parent is not a directory".to_string())
            }
        } else {
            Err("File not found".to_string())
        }
    }

    fn read(&self, node: &Vnode) -> Result<Vec<u8>, String> {
        if let Some(content) = &node.content {
            Ok(content.clone())
        } else {
            Err("Node is not a file".to_string())
        }
    }

    fn write(&mut self, node: &mut Vnode, data: Vec<u8>) -> Result<(), String> {
        if let Some(content) = &mut node.content {
            content.extend(data);
            node.size = content.len() as u64;
            Ok(())
        } else {
            Err("Node is not a file".to_string())
        }
    }

    fn close(&mut self, _node: &Vnode) -> Result<(), String> {
        Ok(())
    }
}
