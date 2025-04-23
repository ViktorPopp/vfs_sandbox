use std::sync::Arc;

use crate::{VNodeType, VfsError, Vnode};

use super::VNodeOps;

pub struct MockFs;

impl MockFs {
    pub fn instance() -> Arc<Self> {
        static INSTANCE: once_cell::sync::Lazy<Arc<MockFs>> =
            once_cell::sync::Lazy::new(|| Arc::new(MockFs));
        INSTANCE.clone()
    }
}

impl VNodeOps for MockFs {
    fn lookup(&self, _directory: Vnode, name: String) -> Result<Vnode, VfsError> {
        match name.as_str() {
            "home" => Ok(Vnode {
                vtype: VNodeType::Directory,
                ops: MockFs::instance(),
            }),
            "file.txt" => Ok(Vnode {
                vtype: VNodeType::Regular,
                ops: MockFs::instance(),
            }),
            _ => Err(VfsError::NotFound),
        }
    }
}
