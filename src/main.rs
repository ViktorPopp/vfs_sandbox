use bitflags::bitflags;

#[derive(Debug, Clone)]
enum VnodeType {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
    SymbolicLink,
    Socket,
}

#[derive(Debug, Clone)]
enum VfsError {
    FileNotFound,
    FileExists,
    NotADirectory,
    NotAFile,
    MountPointNotFound,
    MountPointInUse,
    InvalidPath,
    Other(String),
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FilePermissions: u16 {
        const MOUNTPOINT  = 0x0001;

        const USER_READ   = 0x0100;
        const USER_WRITE  = 0x0080;
        const USER_EXEC   = 0x0040;

        const GROUP_READ  = 0x0020;
        const GROUP_WRITE = 0x0010;
        const GROUP_EXEC  = 0x0008;

        const OTHER_READ  = 0x0004;
        const OTHER_WRITE = 0x0002;
        const OTHER_EXEC  = 0x0001;
    }
}


pub struct Vfs {
    pub vfs_next: Option<Box<Vfs>>,

}

pub struct VfsOperations {}

pub struct Vnode {}

fn main() {
    println!("Hello, world!");
}
