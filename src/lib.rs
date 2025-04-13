#![allow(dead_code, unused_imports)]

use bitflags::bitflags;

pub mod utils;

/* VNODE */

#[derive(Clone)]
pub struct Vnode {
    name: String,
    pub vtype: VnodeType,
    pub ops: Option<VnodeOps>,
    pub flags: VnodeFlags,
    pub mount: Box<Mount>,
    pub fs_data: Option<Box<dyn utils::AnyClone>>,
}

#[derive(Clone)]
pub enum VnodeType {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
}

#[derive(Clone)]
pub struct VnodeOps {
    pub read: fn(&Vnode, &mut [u8], usize) -> i32,
    pub lookup: fn(&str) -> Vnode,
}

bitflags! {
    #[derive(Clone)]
    pub struct VnodeFlags: u8 {
        const ROOT = 0x1;
        const USED = 0x2;
    }
}

/* MOUNT */

#[derive(Clone)]
pub struct Mount {
    root: Vnode,
    next: Option<Box<Mount>>,
    prev: Option<Box<Mount>>,
    mountpoint: String,
    fs_data: Option<Box<dyn utils::AnyClone>>,
}

/* VFS */

pub struct VFS {
    pub root_mount: Box<Mount>,
}

impl VFS {
    pub fn init(&mut self) {
        self.root_mount = Box::new(Mount {
            root: Vnode {
                name: String::from("/"),
                vtype: VnodeType::Directory,
                flags: VnodeFlags::ROOT,
                ops: None,
                mount: self.root_mount.clone(),
                fs_data: None,
            },
            next: None,
            prev: None,
            mountpoint: String::from("/"),
            fs_data: None,
        });
    }
}
