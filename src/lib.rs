#![allow(dead_code, unused_imports)]

use bitflags::bitflags;

pub mod utils;

/* VNODE */

#[derive(Clone)]
pub struct Vnode {
    pub name: String,
    pub vtype: VnodeType,
    pub ops: Option<Box<dyn utils::AnyClone>>,
    pub flags: Option<OpenFlags>,
    pub mount: Box<Mount>,
    pub fs_data: Option<Box<dyn utils::AnyClone>>,
}

#[derive(Clone)]
pub enum VnodeType {
    Unknown,
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
    Bad,
}

type RdWrDir = bool;

pub trait VnodeOps: Send + Sync + utils::AnyClone {
    fn lookup(directory: Vnode, name: String) -> Result<Vnode, String>;
    fn open(old_vnode: Vnode, flags: OpenFlags) -> Result<Vnode, String>;
    fn rdwr(
        &self,
        vnode: &Vnode,
        length: usize,
        offset: usize,
        direction: RdWrDir,
    ) -> Result<*mut u8, String>;
}

#[derive(Clone)]
pub enum OpenFlags {
    // File Access Flags
    ReadOnly,  // Open for reading only
    WriteOnly, // Open for writing only
    ReadWrite, // Open for both reading and writing

    // File Creation and Status Flags
    Create,    // Create file if it doesn't exist
    Exclusive, // Ensure that the file is created; if it already exists, return an error
    NoCTTY,    // If the file is a terminal, do not make it the controlling terminal
    Truncate,  // Truncate the file to zero length if it exists and is opened for writing
    Append,    // Open the file in append mode (writing at the end)
    NonBlock,  // Non-blocking mode; operations on the file won’t block
    Dsync,     // Writes will be synchronized with disk before returning (for data)
    Rsync,     // Data and metadata are synchronized before returning

    // File Locking Flags
    Async, // Enable asynchronous notification when data is ready for I/O
    Sync,  // Writes to the file will be synchronized with disk before returning

    // File Type Flags
    Directory, // Open the file as a directory
    LargeFile, // Allow opening files larger than 2 GB (on older systems)

    // Additional Flags
    NoFollow, // Do not follow symbolic links
    CloExec,  // Set close-on-exec flag, automatically closing the file on exec
}

/* MOUNT */

#[derive(Clone)]
pub struct Mount {
    root: Vnode,
    //next: Option<Box<Mount>>,
    //prev: Option<Box<Mount>>,
    mountpoint: String,
    fs_data: Option<Box<dyn utils::AnyClone>>,
}

/* VFS */

pub struct Vfs {
    pub root_mount: Option<Box<Mount>>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs { root_mount: None }
    }

    pub fn init(&mut self) {

    }
}
