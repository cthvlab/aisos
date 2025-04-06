// logger.rs
#![allow(dead_code)] // Для подавления предупреждений о неиспользуемом коде на этапе разработки

use uart_16550::SerialPort;
use spin::Mutex;
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use x86_64::instructions::port::Port; // Для работы с портами VGA

// Адреса для VGA-буфера
const VGA_BUFFER: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Перечисление уровней логирования
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// Структура для управления VGA-выводом
struct VgaWriter {
    column: usize,
    row: usize,
    buffer: &'static mut [u16],
}

impl VgaWriter {
    fn new() -> Self {
        VgaWriter {
            column: 0,
            row: 0,
            buffer: unsafe { core::slice::from_raw_parts_mut(VGA_BUFFER as *mut u16, VGA_WIDTH * VGA_HEIGHT) },
        }
    }

    fn write_char(&mut self, c: char, color: u8) {
        if c == '\n' {
            self.new_line();
            return;
        }

        if self.column >= VGA_WIDTH {
            self.new_line();
        }

        let offset = self.row * VGA_WIDTH + self.column;
        self.buffer[offset] = (color as u16) << 8 | (c as u16);
        self.column += 1;
    }

    fn new_line(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= VGA_HEIGHT {
            self.scroll();
        }
    }

    fn scroll(&mut self) {
        for i in 0..(VGA_HEIGHT - 1) * VGA_WIDTH {
            self.buffer[i] = self.buffer[i + VGA_WIDTH];
        }
        let last_row = (VGA_HEIGHT - 1) * VGA_WIDTH;
        for i in last_row..last_row + VGA_WIDTH {
            self.buffer[i] = 0x0700; // Очистка последней строки (белый текст на чёрном фоне)
        }
        self.row = VGA_HEIGHT - 1;
    }
}

impl Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c, 0x07); // Белый текст на чёрном фоне
        }
        Ok(())
    }
}

// Глобальные объекты
lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        Mutex::new(serial)
    };

    static ref VGA_WRITER: Mutex<VgaWriter> = {
        Mutex::new(VgaWriter::new())
    };
}

// Основная функция вывода
#[doc(hidden)]
pub fn _print(args: fmt::Arguments, level: LogLevel) {
    let mut serial = SERIAL1.lock();
    let mut vga = VGA_WRITER.lock();

    // Префикс с уровнем логирования
    match level {
        LogLevel::Info => serial.write_str("[INFO] ").unwrap(),
        LogLevel::Warning => serial.write_str("[WARN] ").unwrap(),
        LogLevel::Error => serial.write_str("[ERROR] ").unwrap(),
    }
    serial.write_fmt(args).unwrap();

    // То же самое для VGA
    match level {
        LogLevel::Info => vga.write_str("[INFO] ").unwrap(),
        LogLevel::Warning => vga.write_str("[WARN] ").unwrap(),
        LogLevel::Error => vga.write_str("[ERROR] ").unwrap(),
    }
    vga.write_fmt(args).unwrap();
}

// Макросы для логирования
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::logger::_print(format_args!($($arg)*), $level);
    };
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::log!($crate::logger::LogLevel::Info, concat!($($arg)*, "\n"));
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log!($crate::logger::LogLevel::Info, $($arg)*);
    };
}

// Инициализация логгера
pub fn init() {
    // Тестовый вывод на оба вывода
    log!(LogLevel::Info, "Logger initialized (serial @ 0x3F8, VGA @ 0xb8000)\n");
}

// Публичная функция для логирования с уровнем
pub fn log(message: &str, level: LogLevel) {
    log!(level, "{}\n", message);
}
