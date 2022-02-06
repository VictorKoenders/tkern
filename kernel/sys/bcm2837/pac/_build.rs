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
    std::fs::write(
        "/home/trangar/development/rust/kernel/kernel/sys/bcm2837/pac/src/lib.rs",
        result.lib_rs,
    )
    .unwrap();
}
