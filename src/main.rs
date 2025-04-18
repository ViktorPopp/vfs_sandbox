use vfs_sandbox::Vfs;
use vfs_sandbox::fs::dummy::DummyFs;

fn main() {
    let dummy_vfs = Box::new(Vfs {
        next: None,
        covers: None,
        ops: Some(Box::new(DummyFs)),
        fs_data: Some(Box::new(())),
    });
}
