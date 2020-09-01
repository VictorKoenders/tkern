//! x86_64-specific interrupt logic. This serves as a glue between the x86_64 hardware and the [interrupts](../../interrupts/index.html) module

use crate::memory::VirtualAddress;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
pub use x86_64::instructions::interrupts::{enable, without_interrupts};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum PicIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        idt[PicIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
        idt[PicIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

/// Initializes the interrupt handler.
/// Before this is called, any interrupt will cause the kernel to reboot.
/// After this is called, all the interrupts are routed to the [interrupts](../../interrupts/index.html) module.
pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    crate::interrupts::breakpoint(stack_frame.into());
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    crate::interrupts::timer_interrupt(stack_frame.into());

    unsafe {
        PICS.lock().notify_end_of_interrupt(PicIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::interrupts::keyboard_interrupt(stack_frame.into(), scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(PicIndex::Keyboard as u8);
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::interrupts::double_fault(stack_frame.into(), error_code);
}

impl<'a> Into<crate::interrupts::StackFrame> for &'a mut InterruptStackFrame {
    fn into(self) -> crate::interrupts::StackFrame {
        crate::interrupts::StackFrame {
            instruction_pointer: VirtualAddress(self.instruction_pointer.as_u64()),
            code_segment: self.code_segment,
            cpu_flags: self.cpu_flags,
            stack_pointer: VirtualAddress(self.stack_pointer.as_u64()),
            stack_segment: self.stack_segment,
        }
    }
}
