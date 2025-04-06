#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::entry_point;
use x86_64::instructions::interrupts;

mod nvme;
mod logger;
mod pci;
mod memory;
mod dma;
mod fs;
mod pkg;
mod time;
mod gpu;
mod net;
mod cpu;
mod syscall;
mod user;
mod proc;

#[global_allocator]
static ALLOCATOR: memory::KernelAllocator = memory::KernelAllocator;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    logger::init();
    println!("[OK] aiSOS booted!");

    let memory_manager = memory::init_memory(boot_info);
    dma::init_dma(&memory_manager);
    pci::scan_pci_devices();
    nvme::init_nvme();
    fs::init_filesystem();
    pkg::load_rpk_modules();
    time::init_timers();
    gpu::init_gpu();
    net::init_network();
    cpu::init_smp();
    syscall::init_syscalls();
    user::init_users();
    proc::init_scheduler();

    println!("[OK] aiSOS fully initialized!");
    loop {
        interrupts::enable_and_hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!");
    if let Some(location) = info.location() {
        println!("At {}:{}", location.file(), location.line());
    }
    if let Some(message) = info.message() {
        println!("Reason: {}", message);
    }
    loop {
        x86_64::instructions::hlt();
    }
}
