#![allow(dead_code, unused_imports)]

use bitflags::bitflags;
use std::rc::Rc;
use std::cell::RefCell;

/* UTILS */

pub mod utils;

/// Read/write direction
const READ_DIRECTION: bool = false;
const WRITE_DIRECTION: bool = true;

/* VNODE */

#[derive(Clone, Default)]
pub struct Vnode {
    name: String,
    pub vtype: VnodeType,
    pub ops: Option<VnodeOps>,
    pub flags: VnodeFlags,
    pub mount: Rc<RefCell<Mount>>,
    pub fs_data: Option<Box<dyn utils::AnyClone>>,
}

#[derive(Clone)]
pub enum VnodeType {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
}

impl Default for VnodeType {
    fn default() -> Self {
        VnodeType::Regular
    }
}

#[derive(Clone)]
pub struct VnodeOps {
    // TODO: Need flags and permissions
    pub rdwr: fn(vnode: &Vnode, size: usize, offset: usize, direction: bool) -> Result<*mut u8, VfsError>,
}

bitflags! {
    #[derive(Clone)]
    pub struct VnodeFlags: u8 {
        const ROOT = 0x1;
        const USED = 0x2;
    }
}

impl Default for VnodeFlags {
    fn default() -> Self {
        VnodeFlags::empty()
    }
}

/* MOUNT */

#[derive(Clone, Default)]
pub struct Mount {
    root: Rc<RefCell<Vnode>>,
    next: Option<Rc<RefCell<Mount>>>,
    prev: Option<Rc<RefCell<Mount>>>,
    mountpoint: String,
    fs_data: Option<Box<dyn utils::AnyClone>>,
}

/* VFS */

pub struct Vfs {
    pub root_mount: Box<Mount>,
}

impl Vfs {
    pub fn new() -> Vfs {
        print!("Running new()");
        // Create root mount first
        let root_mount = Mount {
            root: Rc::new(RefCell::new(Vnode::default())), // Temporary default vnode
            next: None,
            prev: None,
            mountpoint: String::from("/"),
            fs_data: None,
        };
        let root_mount_rc = Rc::new(RefCell::new(root_mount));
        print!("Created root mount");

        // Create the actual root vnode
        let root_vnode = Rc::new(RefCell::new(Vnode {
            name: String::from("/"),
            vtype: VnodeType::Directory,
            flags: VnodeFlags::ROOT,
            ops: None,
            mount: root_mount_rc.clone(),
            fs_data: None,
        }));
        print!("Created root vnode");

        // Update root mount with correct root vnode
        root_mount_rc.borrow_mut().root = root_vnode.clone();

        // Create the final Vfs structure
        Vfs {
            root_mount: Box::new(Mount {
                root: root_vnode,
                next: None,
                prev: None,
                mountpoint: String::from("/"),
                fs_data: None,
            })
        }
    }
}

pub struct VfsError {

}
