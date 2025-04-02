#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::entry_point;
use x86_64::instructions::interrupts;

mod logger;
mod pci;
mod memory;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    logger::init();

    println!("[OK] Rust Server OS booted!");
    pci::scan_pci_devices();

    loop {
        interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    loop {}
}
