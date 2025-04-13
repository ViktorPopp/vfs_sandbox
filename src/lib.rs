enum Type {
    Regular,
    Directory,
    BlockDevice,
    CharDevice,
    SymbolicLink,
    Socket,
}

enum OpenMode {
    Read,
    Write,
    Append,
    Create,
}

struct MountManager {

}

trait Filesystem {
    fn open(path: String, mode: OpenMode) -> Result<Vnode, String>;
    fn read(node: Vnode) -> Result<Vec<u8>, String>;
    fn close(node: Vnode) -> Result<(), String>;
}

struct Vnode {
    pub name: String,
    pub mode: OpenMode,
    pub file_type: Type,
    pub size: u64,
    pub parent: Option<Box<Vnode>>,
    pub children: Option<Vec<Vnode>>,
    data: Vec<u8>, // Private Fs-specific data
}
