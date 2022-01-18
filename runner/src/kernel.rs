use std::process::Command;

pub const DIR: &str = "../kernel";

pub fn objcopy(src: &str, target: &str) {
    let mut cmd = Command::new("rust-objcopy");
    cmd.args(&["--strip-all", "-O", "binary", src, target]);
    cmd.current_dir(DIR);
    crate::utils::run(cmd);
}

pub fn build(release: bool,) -> String {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--target", "aarch64-unknown-none-softfloat"]);
	if release { cmd.arg("--release");}
    cmd.current_dir(DIR);
    cmd.env(
        "RUSTFLAGS",
        "-C link-arg=-Tsrc/bsp/raspberrypi/link.ld -C target-cpu=cortex-a72",
    );
    crate::utils::run(cmd);

    String::from("target/aarch64-unknown-none-softfloat/debug/kernel")
}
