fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("tkern") {
        println!("cargo:rerun-if-changed=linker.ld");
        println!("cargo:rustc-link-arg=-Tlinker.ld");
    }
}
