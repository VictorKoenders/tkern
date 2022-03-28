# Build and run

Install the required tools:

- Rustup [https://rustup.rs]
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    - Make sure to select `nightly`
    - You can also switch to nightly with `rustup default nightly`
- Rust components: 
  - rust-src: `rustup component add rust-src`
  - llvm-tools-preview: `rustup component add llvm-tools-preview`
- Cargo crates:
  - cargo-binutils: `cargo install cargo-binutils`
  - cargo-make: `cargo install cargo-make`
- Qemu aarch64 [https://www.qemu.org/]
  - Ubuntu: `sudo apt install qemu-system-arm`

Then run

```bash
cargo make qemu
```

For browsing in an IDE, e.g. vscode, install the `aarch64-unknown-none-softfloat` target:

```bash
rustup target add aarch64-unknown-none-softfloat
```

# Commands

To interact with the project, [cargo-make](https://sagiegurari.github.io/cargo-make/) is used. The following commands are configured:

- `cargo make check`: Quickly checks if the project is in a compilable state.
- `cargo make qemu`: Build the project and run it in `qemu-system-aarch64`
- `cargo make fmt` or `cargo make format`: format the code in the project

# Project structure

The kernel is located in `src`.

Drivers for interacting with the system are located in the `driver` folder.

Implementations of those drivers can be found in the `sys` folder.
