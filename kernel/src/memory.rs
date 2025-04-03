pub unsafe fn mmio_map<T>(phys_addr: u64) -> *mut T {
    phys_addr as *mut T
}
