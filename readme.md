# Build and run

Install the required tools:

- Rustup [https://rustup.rs]
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - This project uses `nightly` but will automatically install it if you'd like to install `stable` instead.
- Cargo make [https://sagiegurari.github.io/cargo-make/]
  - `cargo install cargo-make`
- Qemu aarch64 [https://www.qemu.org/]
  - Ubuntu: `sudo apt install qemu-system-arm`

For browsing in an IDE, e.g. vscode, install the `aarch64-unknown-none-softfloat` target:

```bash
rustup target add aarch64-unknown-none-softfloat
```

Then run

```bash
cargo make qemu
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
