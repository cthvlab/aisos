pub unsafe fn mmio_map<T>(phys_addr: u64) -> *mut T {
    phys_addr as *mut T
}
pub fn wait_cycles(cycles: u64) {
    for _ in 0..cycles {
        unsafe { core::arch::asm!("pause") };
    }
}
