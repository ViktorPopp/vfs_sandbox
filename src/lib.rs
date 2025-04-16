pub mod utils;

#[derive(Debug)]
pub enum VnodeType {
    None,
    Regular,
    Directory,
    BlockDevice,
    CharacterDevice,
    SymbolicLink,
    Socket,
    Bad,
}


