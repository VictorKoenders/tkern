# How to run

```bash
wget https://github.com/rust-osdev/ovmf-prebuilt/releases/download/v0.20220719.209%2Bgf0064ac3af/OVMF-pure-efi.fd  -o OVMF-pure-efi.fd
cargo build --target x86_64-unknown-uefi -Zbuild-std=core -Z build-std-features=compiler-builtins-mem
cargo run --target x86_64-pc-windows-msvc --bin disk_image --features disk_image -- ../../target/x86_64-unknown-uefi/debug/tkern_arch_x86_64.efi
qemu-system-x86_64 -drive format=raw,file=../../target/x86_64-unknown-uefi/debug/tkern_arch_x86_64.gdt -bios OVMF-pure-efi.fd
```