# aiSOS: artificial intelligence Server Operating System at heavy metal 👻

**aiSOS** — минималистичная операционная система на Rust (`no_std`), с поддержкой AI-инференса, NVMe-дисков, сетей на DPDK и GPU-инференса через NVIDIA Tesla.

## 💻 Рекомендации к железу

| Компонент | Модель |
|-----------|--------|
| CPU       | AMD Ryzen 9 5950X (16 ядер) |
| RAM       | 32+ GB DDR4 |
| SSD       | NVMe SSD (например, Samsung 970 EVO Plus) |
| GPU       | NVIDIA Tesla K80 (опционально) |
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
│       ├── gpu.rs                   # BAR / DMA инициализация
│       └── net/
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
