# How to build

```bash
RUSTFLAGS="-C link-arg=--script=$pwd/linker.ld"
cargo build --target os.json
qemu-system-aarch64 -M raspi3b -kernel ../../target/os/debug/tkern_arch_aarch64 -display none -serial stdio
```