$arch = $Args[0]

cargo bootimage --target ./arch/$arch/os.json
& "qemu-system-$arch" -drive "format=raw,file=target/os/debug/bootimage-tkern_arch_$arch.bin"