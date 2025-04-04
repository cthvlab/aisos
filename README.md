# aiSOS: artificial intelligence Server Operating System at heavy metal 👻

**aiSOS** — минималистичная операционная система на Rust (`no_std`), с поддержкой AI-инференса на CPU или GPU через NVIDIA Tesla, NVMe-дисков, сетей на DPDK и чего-то еще

### **Технические требования к тяжелому железу для разработки на Rust `no_std`**  

| **Компонент**       | **Критерии**                                                                 | **Минимальные требования**                     | **Рекомендуемые требования**                          |
|---------------------|-----------------------------------------------------------------------------|-----------------------------------------------|------------------------------------------------------|
| **Процессор (CPU)** | • Поддержка `x86_64`/`aarch64` (для `no_std`-таргетов) <br> • Документированные регистры (MMIO, MSI/MSI-X) <br> • Многоядерность (SMP для тестирования) <br> • PCIe 3.0+/4.0 (минимум 16 линий) | • 4+ ядер <br> • 16+ PCIe линий <br> • Поддержка UEFI | • 8+ ядер (AMD EPYC, Intel Xeon) <br> • 64+ PCIe линий <br> • Документированные PPR/даташиты |
| **Материнская плата** | • UEFI (не Legacy BIOS) <br> • Разъёмы PCIe x8/x16 (для NVMe/FPGA) <br> • Отладочные интерфейсы (COM-порт, JTAG) <br> • Поддержка ECC RAM | • 2+ PCIe x8 слотов <br> • Заглушка для UART | • Чипсет с открытой документацией (AMD SP3, Intel W680) <br> • Поддержка SlimSAS (для U.2 NVMe) |
| **Накопители (NVMe)** | • Формат U.2 (для прямого подключения) <br> • Поддержка PCIe 3.0+/4.0 <br> • Протокол NVMe 1.3+ <br> • Энергонезависимость (Power Loss Protection) | • 1TB PCIe 3.0 x4 <br> • Без HW-шифрования | • Intel SSD DC P4510 (enterprise) <br> • Samsung PM9A3 (высокая надёжность) |
| **Сетевая карта (NIC)** | • Поддержка DPDK (`ixgbe`, `mlx4`) <br> • Скорость 10G+ <br> • Документированные регистры (для `no_std`-драйверов) <br> • PCIe x4/x8 | • Intel I350-T4 (1G, DPDK) <br> • Mellanox ConnectX-3 (10G) | • Intel X550-T2 (10G, полная документация) <br> • NVIDIA ConnectX-6 (25G, RDMA) |
| **Память (RAM)**   | • ECC (для избежания ошибок) <br> • Минимум 16 ГБ (для компиляции Rust) <br> • Поддержка многоканального режима | • DDR4 2400 МГц <br> • 16 ГБ ECC | • DDR4 3200 МГц <br> • 32+ ГБ ECC RDIMM |
| **GPU (CUDA)**     | • Поддержка CUDA 11+ <br> • 8+ GB VRAM <br> • PCIe 4.0 x16 <br> • Документированные регистры | • NVIDIA T4 | • NVIDIA A100 <br> • RTX 4090 |
| **Отладка**        | • UART/Serial-порт (для `println!` в `no_std`) <br> • JTAG/SWD (аппаратная отладка) <br> • Логический анализатор (Saleae) | • USB-UART адаптер <br> • Segger J-Link EDU | • PCIe-адаптер с COM-портом <br> • Логический анализатор 500 МГц+ |

---

### **Пояснения:**
1. **Процессор**  
   - **MMIO/MSI** — доступ к регистрам устройств без ОС.  
   - **PCIe** — чем больше линий, тем больше устройств можно подключить (NVMe, FPGA, GPU).  

2. **NVMe**  
   - **U.2** — надёжнее M.2 (лучшее охлаждение, поддержка hot-swap).  
   - **PLP (Power Loss Protection)** — защита данных при отключении питания.  

3. **Сеть**  
   - **DPDK** — ускоряет обработку пакетов в userspace (ключевое для Rust-стека `smoltcp`).  
   - **Документация** — Intel публикует datasheets для своих NIC, Mellanox — только API.  

4. **Отладка**  
   - **UART** — минимально необходим для вывода логов в `no_std`.  
   - **JTAG** — для отладки на уровне процессора (например, при падениях в `unsafe`-коде).
     
5. **GPU**:
   - Обязательно: PCIe 4.0 x16 для уменьшения bottleneck
   - Крейты: `rust-cuda`, `tch-rs` (Torch bindings)
   - Для `no_std`: доступ к PCIe config space через `x86_64` крейты
   - Tesla-серия лучше подходит для серверных решений (ECC память)
---

### **Идеальный набор для `no_std` Rust:**
- **CPU:** AMD EPYC 7302P (128 PCIe 3.0, документация AMD PPR).  
- **Материнка:** Supermicro H12SSL-NT (SlimSAS, UEFI, 8x PCIe x16).  
- **NVMe:** Intel SSD DC P4510 2TB U.2 (PCIe 3.0 x4, PLP).  
- **Сеть:** Intel X550-T2 (10G, DPDK).  
- **Память:** 32 GB DDR4 ECC RDIMM.
- **GPU** NVIDIA A100
- **Отладка:** FTDI USB-UART + Saleae Logic 16.  

Этот набор покрывает 99% задач: от embedded до гипервизоров. 

## 💻 Пример железа для домашних экспериментов

| Компонент | Модель |
|-----------|--------|
| CPU       | AMD Ryzen 9 5950X (16 ядер) |
| RAM       | 32+ GB DDR4 |
| SSD       | NVMe SSD (например, Samsung 970 EVO Plus) |
| GPU       | NVIDIA Tesla T4 (опционально) |
| Сеть      | Intel X550-T2 (10G) |

---

## 📦 Структура проекта

```
aiSOS/
├── Cargo.toml                       # Общий workspace
├── boot/                            # UEFI-загрузчик
│   └── Cargo.toml
├── kernel/                          # Ядро ОС
│   ├── Cargo.toml
│   ├── x86_64-aisos.json            # Файл таргета для сборки ядра
│   └── src/
│       ├── main.rs                  # Точка входа
│       ├── memory.rs                # Работа с памятью
│       ├── logger.rs                # VGA / Serial логгирование
│       ├── pci.rs                   # PCI-сканирование (включая Tesla)
│       ├── nvme.rs                  # NVMe-драйвер
|       ├── nvme_cache.rs
|       ├── fs.rs                    # in-memory файловая система
|       ├── pkg.rs                   # менеджер RPK
|       ├── acpi.rs / smp.rs         # для многоядерности
|       ├── time.rs                  # таймеры
│       ├── gpu.rs                   # BAR / DMA инициализация
│       └── net/
|           ├── mod.rs 
│           ├── dpdk.rs              # Драйвер Intel X550-T2 через DPDK
│           └── http.rs              # HTTP / WebSocket API
├── rpk/
│   └── gemma_gpu/
│       ├── Cargo.toml
│       ├── README.md                # Документация модуля
│       └── src/
│           ├── main.rs
│           ├── cpu.rs
│           ├── gpu.rs
│           ├── fs.rs
│           └── detect.rs
├── static/
│   └── ai_chat_ws.html              # WebSocket UI для AI
└── README.md                        # Главная документация

```

---



## 🛠 Сборка операционной системы

> Убедись, что установлен nightly Rust и `cargo-xbuild`:

```bash
rustup install nightly
rustup component add rust-src --toolchain nightly
cargo install cargo-xbuild
```

1. Склонируй репозиторий:

```bash
git clone https://github.com/cthvlab/aisos.git
cd aisos
```

2. Собери загрузчик и ядро:

```bash
cd boot
cargo build --release --target x86_64-unknown-uefi

cd ../kernel
cargo xbuild --release --target x86_64-aisos.json
```

3. Создай EFI-диск или ISO:

```bash
mkdir -p iso/EFI/BOOT
cp ../boot/target/x86_64-unknown-uefi/release/bootx64.efi iso/EFI/BOOT/
cp target/x86_64-aisos/release/kernel iso/kernel.elf
grub-mkrescue -o aisos.iso iso/
```

---

## 💽 Установка

1. Запиши `aisos.iso` на флешку или виртуальный диск:

```bash
sudo dd if=aisos.iso of=/dev/sdX bs=4M status=progress
```

2. Запусти на реальном сервере или в QEMU:

```bash
qemu-system-x86_64 -m 4096 -cdrom aisos.iso -enable-kvm
```

---

## 🌐 Использование / Инференс

### Загрузка модели:

```bash
curl -X PUT http://<ip>/fs/models/gemma-2b.safetensors --data-binary @gemma-2b.safetensors
curl -X PUT http://<ip>/fs/models/tokenizer.json --data-binary @tokenizer.json
```

### Установка AI-модуля:

```bash
curl -X POST http://<ip>/pkg/install --data-binary @gemma_gpu.rpk
```

### Запрос инференса:

```bash
curl -X POST http://<ip>/ai -d "Что такое Rust?"
```

ОС сама выберет: использовать CPU (через candle) или GPU (через Tesla, если доступна).

---

## 🖥 Взаимодействие через браузер

Операционная система aiSOS поднимает встроенный HTTP-сервер, доступный по IP.

---

## 🧩 Создание `.rpk` модулей

```bash
cd rpk/gemma_gpu
cargo build --release
cp target/release/gemma_gpu gemma_gpu.rpk
```

Установить модуль:

```bash
curl -X POST http://<ip>/pkg/install --data-binary @gemma_gpu.rpk
```

---

## 🔄 Какие еще бывают Rust-ОС

| Проект       | Описание                                                                                   | Ссылка                                                                 |
|--------------|--------------------------------------------------------------------------------------------|------------------------------------------------------------------------|
| **Redox OS** | Unix-подобная операционная система на основе микроядра, написанная на Rust.                | [https://www.redox-os.org](https://www.redox-os.org)                   |
| **Tock OS**  | ОС для микроконтроллеров, ориентированная на безопасность и изоляцию.                      | [https://www.tockos.org](https://www.tockos.org)                       |
| **Theseus**  | Исследовательская ОС, написанная на Rust, с акцентом на безопасность и live-upgrade.       | [https://theseus-os.github.io](https://theseus-os.github.io)          |

aiSOS отличается тем, что:

- работает как **серверная платформа** для AI-инференса
- использует **.rpk-модули** для расширения функциональности
- предоставляет **интерфейс через HTTP, WebSocket, файловую систему**
- совместима с GPU (NVIDIA Tesla), без GUI

---

## 📜 Лицензия

Все что попадает в интернет, становится достоянием общественности!
