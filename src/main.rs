#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Etapa 0: entrada freestanding.
/// En la etapa 1 el bootloader reemplaza esta firma por `bootloader::entry_point!`.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        drivers::serial::init();

        serial_println!("=======================================");
        serial_println!("  ¡Hola desde mi propio Kernel Rust!  ");
        serial_println!("=======================================");

        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
