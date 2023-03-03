use std::path::Path;

use ethers::contract::MultiAbigen;

fn main() {
    let abi_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("abi");

    MultiAbigen::from_json_files(&abi_path)
        .unwrap()
        .build()
        .unwrap()
        .write_to_module(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/abi"), false)
        .unwrap();

    println!("cargo:rerun-if-changed={}", abi_path.display());
}
