#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::entry_point;
use x86_64::instructions::interrupts;
mod nvme;
mod logger;
mod pci;
mod memory;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    logger::init();

    println!("[OK] aiSOS booted!");
    pci::scan_pci_devices();
    nvme::init_nvme();
    loop {
        interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    loop {}
}
