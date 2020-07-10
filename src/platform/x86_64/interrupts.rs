use super::halt_loop;
use crate::platform::pic;
use lazy_static::lazy_static;
pub use x86_64::instructions::interrupts::without_interrupts;
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

pub fn init() {
    IDT.load();
}

pub fn bkpt() {
    x86_64::instructions::interrupts::int3();
}

pub fn enable() {
    x86_64::instructions::interrupts::enable();
}

pub fn disable() {
    x86_64::instructions::interrupts::disable();
}

pub fn enable_and_halt() {
    x86_64::instructions::interrupts::enable_interrupts_and_hlt();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point
            .set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception
            .set_handler_fn(security_exception_handler);

        idt[pic::InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[pic::InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn debug_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("DEBUG\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: NON MASKABLE INTERRUPT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n{:#?}\nCODE: {}",
        stack_frame, error_code
    );
}

extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vga_println!(
        "EXCEPTION: INVALID TSS\n{:#?}\nCODE: {}",
        stack_frame,
        error_code
    );
}

extern "x86-interrupt" fn segment_not_present_handler(
    _stack_frame: &mut InterruptStackFrame,
    segment_id: u64,
) {
    // TODO: Load in this segment
    vga_println!("[WARN] Segment not present: 0x{:04X}", segment_id);
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vga_println!(
        "EXCEPTION: STACK SEGMENT FAULT\n{:#?}\nCODE: {}",
        stack_frame,
        error_code
    );
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vga_println!(
        "EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}\nCODE: {}",
        stack_frame,
        error_code
    );
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    vga_println!("EXCEPTION: PAGE FAULT");
    vga_println!("Accessed Address: {:?}", Cr2::read());
    vga_println!("Error Code: {:?}", error_code);
    vga_println!("{:#?}", stack_frame);
    halt_loop();
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: x87 FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vga_println!(
        "EXCEPTION: ALIGNMENT CHECK\n{:#?}\nCODE: {}",
        stack_frame,
        error_code
    );
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut InterruptStackFrame) -> ! {
    vga_println!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame,);
    crate::platform::halt_loop();
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: &mut InterruptStackFrame) {
    vga_println!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn security_exception_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: u64,
) {
    vga_println!(
        "EXCEPTION: SECURITY EXCEPTION\n{:#?}\nCODE: {}",
        stack_frame,
        error_code
    );
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    pic::interrupt_timer();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    pic::interrupt_keyboard(scancode);
}
