use super::*;

pub struct InMemoryFilesystem {
    root: Vnode,
}

impl InMemoryFilesystem {
    pub fn new() -> Self {
        let root = Vnode {
            name: "/".to_string(),
            file_type: Type::Directory,
            size: 0,
            parent: None,
            children: Some(Vec::new()),
            data: Vec::new(),
        };
        Self { root }
    }
}

impl Filesystem for InMemoryFilesystem {
    fn open(&self, path: String) -> Result<Vnode, String> {
        let mut current = &self.root;
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        for part in parts {
            if let Some(children) = &current.children {
                if let Some(child) = children.iter().find(|c| c.name == part) {
                    current = child;
                } else {
                    return Err("File not found".to_string());
                }
            } else {
                return Err("Not a directory".to_string());
            }
        }

        Ok(current.clone())
    }

    fn read(&self, node: &Vnode) -> Result<Vec<u8>, String> {
        if node.file_type == Type::Regular {
            Ok(node.data.clone())
        } else {
            Err("Cannot read from a non-regular file".to_string())
        }
    }

    fn close(&self, _node: &Vnode) -> Result<(), String> {
        Ok(())
    }
}
