use vfs_sandbox::Vfs;

fn main() {
    print!("----- START OF PROGRAM -----\n");

    println!("> Creating new Vfs instance...");
    let mut vfs: Vfs = Vfs::new();
    println!("> Initializing the Vfs instance...");
    vfs.init();

    print!("------ END OF PROGRAM ------\n");
}
