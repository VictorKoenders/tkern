//! Abstracted interrupt handling. Generic over all platforms.
//!
//! These interrupts get called from the arch/xxx/interrupts.rs module

use crate::VirtualAddress;
use core::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
use spin::Mutex;

/// Called when a breakpoint is hit
pub fn breakpoint(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Called when a fixed timer is elapsed.
pub fn timer_interrupt(_stack_frame: StackFrame) {}

/// Called when a keyboard button is pressed.
pub fn keyboard_interrupt(_stack_frame: StackFrame, scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    if let Some(key) = keyboard.update(scancode) {
        match key {
            DecodedKey::Unicode(character) => {
                if character == 'c' && keyboard.ctrl_pressed {
                    unsafe { crate::dev_utils::exit_qemu(crate::dev_utils::QemuExitCode::Success) };
                } else if character == '\n' {
                    ENTER_PRESSED_COUNT.fetch_add(1, Ordering::Relaxed);
                } else {
                    vga_print!("{}", character);
                }
            }
            DecodedKey::RawKey(key) => vga_print!("{:?}", key),
        }
    }
}

/// Occurs when an interrupt is called but no handler is set in place.
/// This is usually the final resting place of the kernel.
pub fn double_fault(stack_frame: StackFrame, error_code: u64) -> ! {
    vga_println!("Double fault, error code {}", error_code);
    vga_println!("{:?}", stack_frame);
    crate::arch::halt_loop();
}

/// General protection fault
pub fn general_protection(stack_frame: StackFrame, code: u64) {
    vga_println!("General protection fault {:?}", code);
    vga_println!("{:?}", stack_frame);
}

/// Page fault
pub fn page_fault(stack_frame: StackFrame, code: PageFaultCode) {
    vga_println!("Page fault {:?}", code);
    vga_println!("{:?}", stack_frame);
}

struct KeyboardState {
    state: Keyboard<layouts::Us104Key, ScancodeSet1>,
    ctrl_pressed: bool,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            state: Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore),
            ctrl_pressed: false,
        }
    }

    pub fn update(&mut self, scancode: u8) -> Option<DecodedKey> {
        if let Ok(Some(key_event)) = self.state.add_byte(scancode) {
            if key_event.code == KeyCode::ControlLeft || key_event.code == KeyCode::ControlRight {
                self.ctrl_pressed = key_event.state == KeyState::Down;
            }
            if let Some(key) = self.state.process_keyevent(key_event) {
                return Some(key);
            }
        }
        None
    }
}

lazy_static! {
    static ref KEYBOARD: Mutex<KeyboardState> = Mutex::new(KeyboardState::new());
}
static ENTER_PRESSED_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Puts the CPU to sleep until the enter key is pressed.
pub fn wait_for_enter() {
    let count = ENTER_PRESSED_COUNT.load(Ordering::Relaxed);
    while count == ENTER_PRESSED_COUNT.load(Ordering::Relaxed) {
        crate::arch::halt();
    }
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Debug)]
pub struct StackFrame {
    /// This value points to the instruction that should be executed when the interrupt
    /// handler returns. For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction. However, for some exceptions (e.g., page faults),
    /// this value points to the faulting instruction, so that the instruction is restarted on
    /// return. See the documentation of the `InterruptDescriptorTable` fields for more details.
    pub instruction_pointer: VirtualAddress,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: VirtualAddress,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}

bitflags::bitflags! {
    /// Describes an page fault error code.
    #[repr(transparent)]
    pub struct PageFaultCode: u64 {
        /// If this flag is set, the page fault was caused by a page-protection violation,
        /// else the page fault was caused by a not-present page.
        const PROTECTION_VIOLATION = 1;

        /// If this flag is set, the memory access that caused the page fault was a write.
        /// Else the access that caused the page fault is a memory read. This bit does not
        /// necessarily indicate the cause of the page fault was a read or write violation.
        const CAUSED_BY_WRITE = 1 << 1;

        /// If this flag is set, an access in user mode (CPL=3) caused the page fault. Else
        /// an access in supervisor mode (CPL=0, 1, or 2) caused the page fault. This bit
        /// does not necessarily indicate the cause of the page fault was a privilege violation.
        const USER_MODE = 1 << 2;

        /// If this flag is set, the page fault is a result of the processor reading a 1 from
        /// a reserved field within a page-translation-table entry.
        const MALFORMED_TABLE = 1 << 3;

        /// If this flag is set, it indicates that the access that caused the page fault was an
        /// instruction fetch.
        const INSTRUCTION_FETCH = 1 << 4;
    }
}
