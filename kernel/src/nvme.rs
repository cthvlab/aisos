use crate::{memory::mmio_map, println};
use crate::pci::{self, PciDevice};
use core::ptr::{read_volatile, write_volatile};
use core::alloc::Layout;
use alloc::alloc::{alloc_zeroed, dealloc};
use x86_64::VirtAddr;

const CLASS_MASS_STORAGE: u8 = 0x01;
const SUBCLASS_NVME: u8 = 0x08;
const QUEUE_SIZE: usize = 64;
const IDENTIFY_BUFFER_SIZE: usize = 4096;

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


    // Выделим память под ASQ/ACQ и Identify buffer
    let layout = Layout::from_size_align(QUEUE_SIZE * 64, 4096).unwrap();
    let asq = unsafe { alloc_zeroed(layout) };
    let acq = unsafe { alloc_zeroed(layout) };

    let identify_buf_layout = Layout::from_size_align(IDENTIFY_BUFFER_SIZE, 4096).unwrap();
    let identify_buf = unsafe { alloc_zeroed(identify_buf_layout) };

    println!("[NVMe] Выделена память для ASQ/ACQ/Identify");

    // Пропишем регистры
    unsafe {
        write_volatile(&mut (*regs).aqa, ((QUEUE_SIZE as u32 - 1) << 16) | (QUEUE_SIZE as u32 - 1));
        write_volatile(&mut (*regs).asq, asq as u64);
        write_volatile(&mut (*regs).acq, acq as u64);
    }

    println!("[NVMe] Очереди инициализированы");

    // Соберём команду Identify
    let cmd = NvmeCommand {
        opcode: 0x06, // Identify
        flags: 0,
        cid: 1,
        nsid: 0,
        mptr: 0,
        prp1: identify_buf as u64,
        prp2: 0,
        cdw10: 1, // CNS = 1 (Identify Controller)
        cdw11: 0, cdw12: 0, cdw13: 0, cdw14: 0, cdw15: 0,
    };

    let asq_ptr = asq as *mut NvmeCommand;
    unsafe {
        core::ptr::write_volatile(asq_ptr, cmd);
    }

    // Doorbell (Submission queue tail = 1)
    let doorbell = (bar0 + 0x1000) as *mut u32; // SQ0TDBL = BAR0 + 0x1000
    unsafe {
        core::ptr::write_volatile(doorbell, 1);
    }

    println!("[NVMe] Команда Identify отправлена");

    // Ожидаем Completion
    let acq_ptr = acq as *const u32;
    let mut status: u32 = 0;
    for _ in 0..1_000_000 {
        status = unsafe { core::ptr::read_volatile(acq_ptr.add(3)) };
        if status & (1 << 0) != 0 {
            break;
        }
    }

    println!("[NVMe] Completion status = {:#X}", status);

    // Распечатаем первые байты ответа
    let buffer = unsafe {
        core::slice::from_raw_parts(identify_buf, IDENTIFY_BUFFER_SIZE)
    };

    println!("[NVMe] Первые байты ответа:");
    for b in &buffer[..64] {
        print!("{:02X} ", b);
    }
    println!();


    
}
