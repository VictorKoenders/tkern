arch ?= x86_64
kernel := build/kernel-$(arch).bin
iso := build/kernel-$(arch).iso
target ?= $(arch)-target
opt_level ?= debug
rust_os := target/$(target)/${opt_level}/libtkern.a
hdd_img := build/hdd.img

linker_script := arch/$(arch)/linker.ld
grub_cfg := arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard arch/$(arch)/*.asm)
assembly_object_files := $(patsubst arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	rm -r build target

run: $(iso) $(hdd_img)
	qemu-system-x86_64 -cdrom $(iso) -m 5G -smp 4 -drive file=$(hdd_img),format=raw \
		-boot d -vga qxl -device isa-debug-exit,iobase=0xf4,iosize=0x04

run_terminal: $(iso) $(hdd_img)
	qemu-system-x86_64 -cdrom $(iso) -m 5G -smp 4 -drive file=$(hdd_img),format=raw \
		-boot d -vga qxl -curses -device isa-debug-exit,iobase=0xf4,iosize=0x04

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	mkdir -p build/isofiles/boot/grub
	cp $(kernel) build/isofiles/boot/kernel.bin
	cp $(grub_cfg) build/isofiles/boot/grub
	grub-mkrescue -o $(iso) build/isofiles
	rm -r build/isofiles

$(hdd_img):
	qemu-img create -f raw $(hdd_img) 4G

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

kernel:
ifeq ($(opt_level), debug)
	cargo xbuild --target $(target).json
else
	cargo xbuild --target $(target).json --release
endif

# compile assembly files
build/arch/$(arch)/%.o: arch/$(arch)/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

test:
	cargo test

check:
	cargo xcheck --target $(target).json

check_watch:
	watchexec -cre ".rs,.toml" -- cargo xcheck --target $(target).json

clippy:
	cargo xclippy --target $(target).json