#[derive(Debug, Clone)]
pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
}

pub fn devices() -> impl Iterator<Item = PciDevice> {
    (0..=255).flat_map(|bus| {
        (0..32).flat_map(move |slot| {
            (0..8).filter_map(move |func| {
                let vendor_id = pci_read_word(bus, slot, func, 0x00);
                if vendor_id == 0xFFFF {
                    return None;
                }
                let device_id = pci_read_word(bus, slot, func, 0x02);
                let class_sub = pci_read_word(bus, slot, func, 0x0A);
                Some(PciDevice {
                    bus,
                    slot,
                    function: func,
                    vendor_id,
                    device_id,
                    class: (class_sub >> 8) as u8,
                    subclass: class_sub as u8,
                })
            })
        })
    })
}

pub fn scan_pci_devices() {
    println!("[PCI] Сканируем шину...");
    for dev in devices() {
        println!(
            "PCI {:02x}:{:02x}.{} => VID: {:04x}, DID: {:04x}, Class: {:02x}, Subclass: {:02x}",
            dev.bus, dev.slot, dev.function,
            dev.vendor_id, dev.device_id,
            dev.class, dev.subclass
        );
    }
}

impl PciDevice {
    pub fn read_bar(&self, index: u8) -> u64 {
        let offset = 0x10 + index * 4;
        let low = pci_read_dword(self.bus, self.slot, self.function, offset);
        let high = pci_read_dword(self.bus, self.slot, self.function, offset + 4);
        ((high as u64) << 32) | (low as u64)
    }
}

fn pci_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let value = pci_read_dword(bus, slot, func, offset & 0xFC);
    let shift = (offset & 2) * 8;
    ((value >> shift) & 0xFFFF) as u16
}

fn pci_read_dword(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address: u32 =
        (1 << 31) | ((bus as u32) << 16) | ((slot as u32) << 11) | ((func as u32) << 8) | (offset as u32);
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") 0xCF8, in("eax") address);
        let value: u32;
        core::arch::asm!("in eax, dx", in("dx") 0xCFC, out("eax") value);
        value
    }
}
