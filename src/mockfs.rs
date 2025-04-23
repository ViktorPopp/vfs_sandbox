use std::sync::Arc;

use crate::{VNodeType, VfsError, Vnode};

use super::VNodeOps;

pub struct MockFs;

impl VNodeOps for MockFs {
    fn lookup(&self, _directory: Vnode, name: String) -> Result<Vnode, VfsError> {
        match name.as_str() {
            "home" => Ok(Vnode {
                vtype: VNodeType::Directory,
                ops: Arc::new(MockFs),
            }),
            "file.txt" => Ok(Vnode {
                vtype: VNodeType::Regular,
                ops: Arc::new(MockFs),
            }),
            _ => Err(VfsError::NotFound),
        }
    }
}
