# Compiling

Follow your platform-specific instructions first, then follow [Configure rust](#configure-rust)

## Ubuntu

```bash
sudo apt install make build-essential xorriso qemu-system-x86 nasm
```

## Configure rust

First install [rust](https://rustup.rs)

```bash
rustup toolchain add nightly
rustup +nightly component add rust-src
cargo +nightly install cargo-xbuild
```

You can also make `nightly` default by running:

```bash
rustup default nightly
```

## Make commands

`make build`: build the kernel
`make check`: Check the rust code on validity, faster than `build`
`make check_watch`: Run `make watch` every time a file changes
`make run`: build & run the kernel in qemu
`make run_terminal`: build & run the kernel in your terminal with qemu (ctrl+c to quit)

# References

- [QuiltOS](https://github.com/QuiltOS/QuiltOS), OS written in rust
- [Writing a simple OS from scratch (pdf)](https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)
- [Kernel tutorial (C)](https://github.com/cfenollosa/os-tutorial)