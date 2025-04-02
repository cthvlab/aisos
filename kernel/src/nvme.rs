use crate::pci;
use crate::println;

const CLASS_MASS_STORAGE: u8 = 0x01;
const SUBCLASS_NVME: u8 = 0x08;

pub fn init_nvme() {
    println!("[NVMe] Инициализация NVMe-дисков…");

    for dev in pci::devices() {
        if dev.class == CLASS_MASS_STORAGE && dev.subclass == SUBCLASS_NVME {
            println!(
                "[NVMe] Найден контроллер NVMe: {:04x}:{:04x} @ bus {}, slot {}, func {}",
                dev.vendor_id, dev.device_id, dev.bus, dev.slot, dev.function
            );

            let bar = dev.read_bar(0);
            println!("[NVMe] BAR0: {:#X}", bar);

            // Здесь можно сделать:
            // - MMIO mapping (если bar > 0x1000)
            // - проверку CAP регистров
        }
    }
}
