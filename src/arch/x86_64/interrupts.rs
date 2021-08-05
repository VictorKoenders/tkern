//! x86_64-specific interrupt logic. This serves as a glue between the x86_64 hardware and the [interrupts](../../interrupts/index.html) module

use crate::VirtualAddress;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
pub use x86_64::instructions::{
    interrupts::{disable, enable, without_interrupts},
    segmentation::Segment,
};
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

pub type CustomInterruptFn = extern "x86-interrupt" fn(InterruptStackFrame);

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

// #[macro_use]
macro_rules! custom_interrupt {
    (fn $name:ident ( $stack_frame: ident : StackFrame) $block:block) => {
        pub extern "x86-interrupt" fn $name(frame: x86_64::structures::idt::InterruptStackFrame) {
            let mut frame: StackFrame = frame.into();
            let $stack_frame = &mut frame;

            $block
        }
    };
}
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum PicIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

const DOUBLE_FAULT_IST_INDEX: u16 = 0;

macro_rules! proxy_simple_exception_to_kernel {
    ($idt:expr => $($err:tt),*) => {
        $(
            extern "x86-interrupt" fn $err(stack_frame: InterruptStackFrame) {
                crate::interrupts::$err(stack_frame.into());
            }
            $idt.$err.set_handler_fn($err);
        )*
    };
}

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

unsafe fn init_idt() {
    fn init_idt_inner() {
        let idt = unsafe { &mut IDT };
        proxy_simple_exception_to_kernel!(idt =>
            divide_error,
            debug,
            non_maskable_interrupt,
            breakpoint,
            overflow,
            bound_range_exceeded,
            invalid_opcode,
            device_not_available
        );

        let double_fault = idt.double_fault.set_handler_fn(double_fault_handler);
        unsafe {
            double_fault.set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt.invalid_tss.set_handler_fn(invalid_tss);
        idt.segment_not_present.set_handler_fn(segment_not_present);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        proxy_simple_exception_to_kernel!(idt => x87_floating_point);

        idt.alignment_check.set_handler_fn(alignment_check);
        idt.machine_check.set_handler_fn(machine_check);

        proxy_simple_exception_to_kernel!(idt =>
            simd_floating_point,
            virtualization
        );
        idt.security_exception.set_handler_fn(security_exception);

        idt[PicIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
        idt[PicIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);

        idt.load();
    }
    init_idt_inner();
}
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            stack_start + STACK_SIZE
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/// Initializes the interrupt handler.
/// Before this is called, any interrupt will cause the kernel to reboot.
/// After this is called, all the interrupts are routed to the [interrupts](../../interrupts/index.html) module.
pub fn init() {
    GDT.0.load();
    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(GDT.1.code_selector);
        x86_64::instructions::tables::load_tss(GDT.1.tss_selector);
        init_idt();
    }
    unsafe { PICS.lock().initialize() };
}

pub unsafe fn register_custom_interrupt(interrupt_id: u16, handler: CustomInterruptFn) {
    IDT[interrupt_id as usize]
        .set_handler_fn(handler)
        .set_present(true);
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    crate::interrupts::double_fault(stack_frame.into(), error_code);
}

extern "x86-interrupt" fn invalid_tss(stack_frame: InterruptStackFrame, error_code: u64) {
    crate::interrupts::invalid_tss(stack_frame.into(), error_code);
}

extern "x86-interrupt" fn segment_not_present(stack_frame: InterruptStackFrame, error_code: u64) {
    crate::interrupts::segment_not_present(stack_frame.into(), error_code.into());
}

extern "x86-interrupt" fn stack_segment_fault(stack_frame: InterruptStackFrame, error_code: u64) {
    crate::interrupts::stack_segment_fault(stack_frame.into(), error_code.into());
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    code: u64,
) {
    crate::interrupts::general_protection(stack_frame.into(), code);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    code: PageFaultErrorCode,
) {
    crate::interrupts::page_fault(stack_frame.into(), code.into());
}

extern "x86-interrupt" fn alignment_check(stack_frame: InterruptStackFrame, code: u64) {
    crate::interrupts::alignment_check(stack_frame.into(), code);
}

extern "x86-interrupt" fn machine_check(stack_frame: InterruptStackFrame) -> ! {
    crate::interrupts::machine_check(stack_frame.into());
}

extern "x86-interrupt" fn security_exception(stack_frame: InterruptStackFrame, code: u64) {
    crate::interrupts::security_exception(stack_frame.into(), code);
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    crate::interrupts::timer_interrupt(stack_frame.into());

    unsafe {
        PICS.lock().notify_end_of_interrupt(PicIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    crate::interrupts::io::keyboard_interrupt(stack_frame.into(), scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(PicIndex::Keyboard as u8);
    }
}

#[allow(clippy::from_over_into)]
impl Into<crate::interrupts::StackFrame> for InterruptStackFrame {
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

#[allow(clippy::from_over_into)]
impl Into<crate::interrupts::PageFaultCode> for PageFaultErrorCode {
    fn into(self) -> crate::interrupts::PageFaultCode {
        unsafe { crate::interrupts::PageFaultCode::from_bits_unchecked(self.bits()) }
    }
}
