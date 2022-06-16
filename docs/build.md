# How to build TKern

First make sure to install [rustup](https://rustup.rs). The latest stable should be fine. Rust will automatically pick the right compiler version and tooling. This is defined in `rust-toolchain.toml` at the root of this project.

**Note:** make sure to read the output of this install. Specifically the part about running `source ~/.cargo/env`.

Also install [cargo-make](https://sagiegurari.github.io/cargo-make/):

```sh
cargo install --locked cargo-make
```

And rust-src:

```sh
rustup component add rust-src
```

Then building this kernel should be as simple as running 

```sh
cargo make
```

Also make sure to install `aarch64-elf-gcc`:
- macos: `brew install aarch64-elf-gcc`
- ubuntu: `apt install gcc-aarch64-linux-gnu`

The following binaries should be available in your path:
- `cargo make --version` (tested with 0.35.12)
- `aarch64-elf-as --version` (tested with 2.38)

# Commands

There are several commands. These are run as `cargo make <command>`

|Command|Description|Notes|
|---|---|---|
|`build`|Build the kernel in qemu.|This is the default action|
|`run qemu`|Run the kernel in qemu. You need to have `qemu-system-aarch64` on your path.|See [Qemu](#qemu) for more info|
|`lints`|Runs both `cargo fmt` and `cargo clippy`|Make sure to run this before committing a PR, and to fix the warnings!|

# Additional tooling

## Qemu

Currently it is recommended to run the kernel in `qemu`. For that you need `qemu-system-aarch64`:

- Macos: `brew install qemu`
- Ubuntu: `apt install qemu-system-arm`
- Windows: Download here: <https://qemu.weilnetz.de/w64/>
