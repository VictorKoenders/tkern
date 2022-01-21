use std::process::Command;

// pub const DIR: &str = "../kernel";

pub fn find_dir() -> &'static str {
    for path in ["kernel", "../kernel/"] {
        if std::path::Path::new(path).join("Cargo.toml").exists() {
            return path;
        }
    }
    panic!("Could not find kernel");
}

pub fn objcopy(src: &str, target: &str) {
    let mut cmd = Command::new("rust-objcopy");
    cmd.args(&["--strip-all", "-O", "binary", src, target]);
    cmd.current_dir(find_dir());
    crate::utils::run(cmd);
}

pub fn build(release: bool) -> String {
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "--target",
        "aarch64-kernel.json",
        "-Zbuild-std=alloc,core",
    ]);
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(find_dir());
    cmd.env(
        "RUSTFLAGS",
        "-C link-arg=-Tsrc/bsp/raspberrypi/link.ld -C target-cpu=cortex-a72",
    );
    crate::utils::run(cmd);

    String::from("target/aarch64-kernel/debug/kernel")
}

pub fn check() {
    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "check",
        "--target",
        "aarch64-kernel.json",
        "-Zbuild-std=alloc,core",
    ]);
    cmd.current_dir(find_dir());
    cmd.env(
        "RUSTFLAGS",
        "-C link-arg=-Tkernel/src/bsp/raspberrypi/link.ld -C target-cpu=cortex-a72",
    );
    crate::utils::run(cmd);
}
