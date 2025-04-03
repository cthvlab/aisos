use crate::pci;
use crate::println;
use crate::{memory::mmio_map, println};
use crate::pci::{self, PciDevice};

const CLASS_MASS_STORAGE: u8 = 0x01;
const SUBCLASS_NVME: u8 = 0x08;

#[repr(C)]
struct NvmeRegs {
    cap: u64,    // 0x00
    vs: u32,     // 0x08
    _rsvd1: u32, // 0x0C
    cc: u32,     // 0x14
    _rsvd2: u32, // 0x18
    csts: u32,   // 0x1C
    // остальные — позже
}

pub fn init_nvme() {
    println!("[NVMe] Инициализация…");

    for dev in pci::devices() {
        if dev.class == 0x01 && dev.subclass == 0x08 {
            println!("[NVMe] Контроллер {:04x}:{:04x}", dev.vendor_id, dev.device_id);
            let bar0 = dev.read_bar(0) & 0xFFFF_FFF0; // маска на MMIO
            println!("[NVMe] BAR0 = {:#X}", bar0);

            let regs = unsafe { mmio_map::<NvmeRegs>(bar0) };

            let cap = unsafe { core::ptr::read_volatile(&(*regs).cap) };
            let vs = unsafe { core::ptr::read_volatile(&(*regs).vs) };
            let csts = unsafe { core::ptr::read_volatile(&(*regs).csts) };

            println!("[NVMe] CAP  = {:#X}", cap);
            println!("[NVMe] VS   = {}.{}", vs >> 16, vs & 0xFFFF);
            println!("[NVMe] CSTS = {:#X}", csts);
        }
    }
}
