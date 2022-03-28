fn main() {
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").expect("Missing env var 'TARGET'")
    );
}
