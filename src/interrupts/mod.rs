//! Abstracted interrupt handling. Generic over all platforms.
//!
//! These interrupts get called from the arch/xxx/interrupts.rs module

use crate::VirtualAddress;

pub mod io;

/// Called when a divide error occurs
pub fn divide_error(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

/// Called when debug-exception is enabled, and the debug is triggered
pub fn debug(stack_frame: StackFrame) {
    vga_println!("DEBUG\n{:#?}", stack_frame);
}

/// Called when a non-maskable interrupt is triggered
pub fn non_maskable_interrupt(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Called when a breakpoint is hit
pub fn breakpoint(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub fn overflow(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
}

pub fn bound_range_exceeded(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
}

pub fn invalid_opcode(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

pub fn device_not_available(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
}

/// Occurs when an interrupt is called but no handler is set in place.
/// This is usually the final resting place of the kernel.
pub fn double_fault(stack_frame: StackFrame, error_code: u64) -> ! {
    vga_println!("Double fault, error code {}", error_code);
    vga_println!("{:?}", stack_frame);
    crate::arch::halt_loop();
}

pub fn invalid_tss(stack_frame: StackFrame, code: u64) {
    vga_println!("EXCEPTION: INVALID TSS {:?}\n{:#?}", code, stack_frame);
}

pub fn segment_not_present(_stack_frame: StackFrame, index: SegmentIndex) {
    vga_println!("Segment not present:");
    vga_println!("  index: {:?}", index);
}

pub fn stack_segment_fault(_stack_frame: StackFrame, index: SegmentIndex) {
    vga_println!("Stack segment fault::");
    vga_println!("  index: {:?}", index);
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

pub fn x87_floating_point(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: x87 FLOATING POINT\n{:#?}", stack_frame);
}

/// Alignment check
pub fn alignment_check(stack_frame: StackFrame, code: u64) {
    vga_println!("EXCEPTION: ALIGNMENT CHECK {}\n{:#?}", code, stack_frame);
}

/// Machine check
pub fn machine_check(stack_frame: StackFrame) -> ! {
    vga_println!("MACHINE CHECK");
    vga_println!("{:?}", stack_frame);
    crate::arch::halt_loop();
}

pub fn simd_floating_point(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
}

pub fn virtualization(stack_frame: StackFrame) {
    vga_println!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}

pub fn security_exception(stack_frame: StackFrame, code: u64) {
    vga_println!("EXCEPTION: SECURITY EXCEPTION {}\n{:#?}", code, stack_frame);
}

/// Called when a fixed timer is elapsed.
pub fn timer_interrupt(_stack_frame: StackFrame) {}

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

/// The segment index used for [segment_not_present] and [stack_segment_fault]
pub struct SegmentIndex(u64);

impl From<u64> for SegmentIndex {
    fn from(v: u64) -> Self {
        Self(v)
    }
}

impl SegmentIndex {
    pub fn index(&self) -> u16 {
        (self.0 >> 3) as u16
    }

    pub fn table(&self) -> SelectorTable {
        match self.0 & 0b110 {
            0b000 => SelectorTable::GDT,
            0b010 => SelectorTable::IDT,
            0b100 => SelectorTable::LDT,
            0b110 => SelectorTable::IDT,
            _ => unsafe { core::hint::unreachable_unchecked() }
        }
    }

    pub fn is_external(&self) -> bool {
        self.0 & 1 == 1
    }
}

impl core::fmt::Debug for SegmentIndex {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("SegmentIndex")
            .field("index", &self.index())
            .field("table", &self.table())
            .field("is_external", &self.is_external())
            .finish()
    }
}

#[derive(Debug)]
pub enum SelectorTable {
    GDT,
    IDT,
    LDT
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
