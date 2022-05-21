// To regenerate `bcm2837_lpa.svd`:
// - rename this file to `build.rs`
// - run `cargo clean; cargo build; cargo fmt` in the `pac` folder
// - Undo this rename

use svd2rust::*;

fn main() {
    let config = Config {
        nightly: true,
        const_generic: true,
        target: Target::None,
        ..Default::default()
    };
    let result = generate(
        &std::fs::read_to_string("bcm2837_lpa.svd").unwrap(),
        &config,
    )
    .unwrap();
    let current_dir = std::env::current_dir().unwrap();
    if !current_dir.ends_with("libs/bcm2837/pac") {
        panic!("Not running in the `libs/bcm2837/pac` dir, exiting now to protect your file system (found {:?})", current_dir);
    }

    std::fs::write("src/lib.rs", result.lib_rs).unwrap();
}
