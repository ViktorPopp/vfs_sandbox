use vfs_sandbox::Vfs;

fn main() {
    println!("----- START OF PROGRAM -----");

    println!("> Creating new Vfs instance...");
    let mut vfs: Vfs = Vfs::new();
    println!("> Initializing the Vfs instance...");
    vfs.init();

    println!("Root Vnode type: {:?}", vfs.root_mount.unwrap().root.unwrap().vtype);

    println!("------ END OF PROGRAM ------");
}
