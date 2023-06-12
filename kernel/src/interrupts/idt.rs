use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{
    console::{self, clear, set_background, set_foreground},
    interrupts::gdt,
    println,
};
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint);
        // idt.alignment_check.set_handler_fn(alignment_check);
        // idt.bound_range_exceeded
        //     .set_handler_fn(bound_range_exceeded);
        // idt.debug.set_handler_fn(debug);
        // idt.device_not_available
        //     .set_handler_fn(device_not_available);
        idt.divide_error.set_handler_fn(divide_error);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault);
        // idt.invalid_opcode.set_handler_fn(invalid_opcode);
        // idt.invalid_tss.set_handler_fn(invalid_tss);
        // idt.machine_check.set_handler_fn(machine_check);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt);
        idt.overflow.set_handler_fn(overflow);
        idt.page_fault.set_handler_fn(page_fault);
        // idt.security_exception.set_handler_fn(security_exception);
        // idt.segment_not_present.set_handler_fn(segment_not_present);
        // idt.simd_floating_point.set_handler_fn(simd_floating_point);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault);
        // idt.virtualization.set_handler_fn(virtualization);
        // idt.vmm_communication_exception
        //     .set_handler_fn(vmm_communication_exception);
        // idt.x87_floating_point.set_handler_fn(x87_floating_point);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    non_diverge("breakpoint", stack_frame);
}

extern "x86-interrupt" fn divide_error(stack_frame: InterruptStackFrame) {
    non_diverge("divide by zero", stack_frame);
}

extern "x86-interrupt" fn overflow(stack_frame: InterruptStackFrame) {
    non_diverge("overflow", stack_frame);
}

extern "x86-interrupt" fn non_maskable_interrupt(stack_frame: InterruptStackFrame) {
    non_diverge("non maskable interrupt", stack_frame);
}

extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    set_background(console::Colors::Red);
    set_foreground(console::Colors::White);
    clear();
    let cr2 = stack_frame.instruction_pointer;
    let is_nullptr = (cr2.as_u64() >> 5) < 0x1000;
    if is_nullptr {
        non_diverge("null ptr", stack_frame);
    } else {
        let protection_violation = error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION);
        let caused_by_write = error_code.contains(PageFaultErrorCode::CAUSED_BY_WRITE);
        let user_mode = error_code.contains(PageFaultErrorCode::USER_MODE);
        let malformed_table = error_code.contains(PageFaultErrorCode::MALFORMED_TABLE);
        let instruction_fetch = error_code.contains(PageFaultErrorCode::INSTRUCTION_FETCH);
        let protection_key = error_code.contains(PageFaultErrorCode::PROTECTION_KEY);
        let shadow_stack = error_code.contains(PageFaultErrorCode::SHADOW_STACK);

        non_diverge("page fault", stack_frame);
        println!("PageFaultErrorCode {{");
        println!("    protection_violation: {},", protection_violation);
        println!("    caused_by_write: {},", caused_by_write);
        println!("    user_mode: {},", user_mode);
        println!("    malformed_table: {},", malformed_table);
        println!("    instruction_fetch: {},", instruction_fetch);
        println!("    protection_key: {},", protection_key);
        println!("    shadow_stack: {},", shadow_stack);
        println!("}}");
    }
    loop {}
}

extern "x86-interrupt" fn general_protection_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    diverge("general protection", stack_frame);
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    diverge("double fault", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault(stack_frame: InterruptStackFrame, _error_code: u64) {
    diverge("stack segment fault", stack_frame);
}

fn diverge(name: &str, stack_frame: InterruptStackFrame) -> ! {
    set_background(console::Colors::Red);
    set_foreground(console::Colors::White);
    clear();
    non_diverge(name, stack_frame);
    loop {}
}

fn non_diverge(name: &str, stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: {}", name);
    println!("{:#?}", stack_frame);
}
