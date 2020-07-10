pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod port;

pub use x86_64::PrivilegeLevel;

fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    halt_loop();
}

#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu_success() -> ! {
    exit_qemu(QemuExitCode::Success)
}

pub fn exit_qemu_failed() -> ! {
    exit_qemu(QemuExitCode::Failed)
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn current_privilege() -> PrivilegeLevel {
    x86_64::instructions::segmentation::cs().rpl()
}
