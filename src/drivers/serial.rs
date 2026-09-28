//! Etapa 1 — puerto serie COM1 (`0x3f8`) para ver logs en la terminal de QEMU.
//!
//! Más fiable que VGA mientras el manejador de pantalla no existe.

// src/drivers/serial.rs
//! Invariante de serial.rs:
//! Protege el puerto I/O serie COM1 mediante un Mutex sin spin-yield del SO,
//! proporcionando una vía segura de depuración por hardware desde cualquier núcleo/contexto.

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static; // O usando spin::Lazy / core::sync::LazyLock en Rust moderno

use core::fmt;

/// Acceso estático global y seguro al primer puerto serie (COM1).
pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe {
    // 0x3F8 es la dirección I/O estándar de puerto de hardware para COM1 en x86
    SerialPort::new(0x3F8)
});

/// Inicializa el puerto serie con los baudios e interfaces por defecto.
pub fn init() {
    SERIAL1.lock().init();
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    // Desactivamos interrupciones temporalmente para evitar Deadlocks si un handler de
    // interrupción intenta imprimir por el puerto serie mientras está bloqueado.
    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Error al escribir en el puerto serie COM1");
    });
}

/// Imprime texto por la salida serie COM1
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(format_args!($($arg)*));
    };
}

/// Imprime texto con salto de línea por la salida serie COM1
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}