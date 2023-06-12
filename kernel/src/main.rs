#![no_std]
#![no_main]
#![feature(core_intrinsics, panic_info_message, abi_x86_interrupt)]

pub mod console;
pub mod driver;
pub mod graphics;
pub mod init;
pub mod interrupts;
pub mod kernel_alloc;
extern crate alloc;

use bootloader_api::{config::Mapping, entry_point, BootInfo, BootloaderConfig};
use console::{clear, set_background, set_foreground};
use core::panic::PanicInfo;
use driver::{
    acpi::{self, find_table, madt::MADT, rstd_xstd},
    framebuffer,
};
use interrupts::apic::init_local_apic;
use kernel_alloc::{
    allocator,
    memory::{self, MEMORY},
};
use x2apic::lapic::xapic_base;
use x86_64::VirtAddr;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    framebuffer::init(
        boot_info
            .framebuffer
            .as_mut()
            .expect("Failed to init Framebuffer"),
    );

    let physical_memory_offset = boot_info
        .physical_memory_offset
        .into_option()
        .expect("Failed to get physical memory offset");
    let physical_memory_offset_virtaddr: VirtAddr = VirtAddr::new(physical_memory_offset);

    unsafe { memory::init(physical_memory_offset_virtaddr, &boot_info.memory_regions) };

    let mut memory = MEMORY.lock();
    let memory = (*memory)
        .get_mut()
        .expect("`MEMORY` accessed before call to init");

    allocator::init_heap(
        &mut memory.mapper,
        &mut memory.frame_allocator,
        0x1000_0000_0000 as *mut u8,
        512 * 1024, // 512KiB
    )
    .expect("Heap initialization failed");

    console::init();
    interrupts::gdt::init();
    interrupts::idt::init();

    let rsdp = boot_info
        .rsdp_addr
        .into_option()
        .expect("Failed to get rsdp address")
        + physical_memory_offset;
    let rsdp = rsdp as *const acpi::ExtendedRSDP;
    let rsdp = unsafe { &*rsdp };
    let rstd_xstd = rstd_xstd(rsdp, physical_memory_offset as usize);
    let rstd_xstd = unsafe { &*rstd_xstd };
    let madt = find_table::<MADT>(&rstd_xstd, b"APIC", physical_memory_offset as usize);
    if madt.is_some() {
        let madt = madt.unwrap();
        let madt = unsafe { &*madt };
        // (unsafe { xapic_base() } + physical_memory_offset) as usize
        let lapic = init_local_apic(madt.sdt.lapic_address);
        println!("AA");
        // println!("{:?}", madt);
        // let apic = unsafe { apic::XApic::new(madt.sdt.lapic_address) };
        // println!("{:?}", apic);
    }
    // driver::keyboard::init().unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // format(format_args!("{}", info)).as_str()
    // info.message().and_then(|x| x.as_str()).unwrap()
    set_background(console::Colors::Red);
    set_foreground(console::Colors::White);
    clear();
    println!("Kernel panic");
    println!("{}", info);
    // asc16::draw_string("KERNEL PANIC", 10, 10, white);
    // asc16::draw_string(
    //     info.message().and_then(|x| x.as_str()).unwrap(),
    //     10,
    //     26,
    //     white,
    // );
    loop {}
}
