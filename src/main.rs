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

fn main() {
    println!("Hello, world!");
}
