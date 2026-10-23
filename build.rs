use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main(){
    // Get the path to Cargo's temporary build directory
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Read memory.x from your project root and write it to OUT_DIR
    let path  = out_dir.join("memory.x");
    File::create(path)
    .unwrap()
    .write_all(include_bytes!("memory.x"))
    .unwrap();

    //Tell Cargo to pass the OUT_DIR path to the linker (-L flag)
    println!{"Cargo:rustc-link-search={}", out_dir.display()};

    //if memory changes
    println!("Cargo:rerun-if-changed=memory.x");
}