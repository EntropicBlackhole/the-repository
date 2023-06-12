use x2apic::lapic::{LocalApic, LocalApicBuilder, TimerDivide, TimerMode};

use crate::kernel_alloc::memory;

const IRQ_INDEX: u8 = 0x20;

#[repr(u8)]
pub enum InterruptIndex {
    Timer = IRQ_INDEX,
    Keyboard = IRQ_INDEX + 1,
    Mouse = IRQ_INDEX + 12,
    ApicError = 151,
}

pub fn init_local_apic(local_apic_address: usize) -> LocalApic {
    memory::identity_map(local_apic_address, None);
    let mut local_apic = LocalApicBuilder::new()
        // https://wiki.osdev.org/APIC_timer
        .timer_vector(InterruptIndex::Timer as usize)
        .timer_divide(TimerDivide::Div64)
        .timer_mode(TimerMode::Periodic)
        .error_vector(InterruptIndex::ApicError as usize)
        .spurious_vector(0xff)
        .set_xapic_base(local_apic_address as u64)
        .build()
        .expect("Failed to build Local APIC");
    unsafe {
        local_apic.enable();
    }
    local_apic
}
