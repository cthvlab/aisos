use crate::{memory::mmio_map, println};
use crate::pci::{self, PciDevice};
use core::ptr::{read_volatile, write_volatile};

const CLASS_MASS_STORAGE: u8 = 0x01;
const SUBCLASS_NVME: u8 = 0x08;

#[repr(C)]
struct NvmeRegs {
    cap: u64,    // 0x00: Controller Capabilities
    vs: u32,     // 0x08: Version
    _rsvd1: u32, // 0x0C
    intms: u32,  // 0x10
    intmc: u32,  // 0x14
    cc: u32,     // 0x14: Controller Configuration
    _rsvd2: u32, // 0x18
    csts: u32,   // 0x1C: Controller Status
    _rsvd3: [u32; 3], // 0x20–0x2C
    aqa: u32,    // 0x24: Admin Queue Attributes
    asq: u64,    // 0x28: Admin Submission Queue Base Address
    acq: u64,    // 0x30: Admin Completion Queue Base Address
}

pub fn init_nvme() {
    println!("[NVMe] Инициализация…");

    for dev in pci::devices() {
        if dev.class == 0x01 && dev.subclass == 0x08 {
            println!("[NVMe] Контроллер {:04x}:{:04x}", dev.vendor_id, dev.device_id);
            let bar0 = dev.read_bar(0) & 0xFFFF_FFF0;
            println!("[NVMe] BAR0 = {:#X}", bar0);

            let regs = unsafe { mmio_map::<NvmeRegs>(bar0) };

            let cap = unsafe { read_volatile(&(*regs).cap) };
            let timeout_ms = ((cap >> 24) & 0xFF) * 500;
            let mqes = (cap & 0xFFFF) as u16;
            let dstrd = ((cap >> 32) & 0xF) as u8;

            println!("[NVMe] CAP.MQES   = {}", mqes);
            println!("[NVMe] CAP.DSTRD  = {}", dstrd);
            println!("[NVMe] CAP.TO     = {} ms", timeout_ms);

            // Сбросим INT маски
            unsafe {
                write_volatile(&mut (*regs).intms, 0xFFFF_FFFF);
                write_volatile(&mut (*regs).intmc, 0xFFFF_FFFF);
            }

            // Настройка CC (EN = 1)
            let cc_value = (6 << 20) // I/O command set (NVM)
                | (0 << 16)          // AMS
                | (4 << 11)          // MPS (4 = 2^16 = 64 KB)
                | (dstrd as u32)     // CSS default
                | (1 << 0);          // EN = 1

            unsafe {
                write_volatile(&mut (*regs).cc, cc_value);
            }

            println!("[NVMe] CC записан, ожидаем CSTS.RDY…");

            let mut ready = false;
            for _ in 0..1000 {
                let csts = unsafe { read_volatile(&(*regs).csts) };
                if csts & 1 == 1 {
                    ready = true;
                    break;
                }
                crate::memory::wait_cycles(1_000_000); // задержка
            }

            if ready {
                println!("[NVMe] Контроллер готов, CSTS.RDY = 1!");
            } else {
                println!("[NVMe] Ошибка: контроллер не вышел в Ready.");
            }

            return;
        }
    }
}
