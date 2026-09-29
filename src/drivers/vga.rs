//! Etapa 2 — buffer de texto VGA en `0xb8000`.
//!
//! Cada celda son 2 bytes: carácter ASCII y atributo de color.
//! 80 columnas × 25 filas. Escribe un `Writer` y un `println!` encima.

use core::fmt;
use core::ptr;
use spin::lazy::Lazy;
use spin::Mutex;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
const VGA_BUFFER: usize = 0xb8000;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
}

impl Writer {
    fn cell_ptr(row: usize, col: usize) -> *mut ScreenChar {
        let offset = row * BUFFER_WIDTH + col;
        unsafe { (VGA_BUFFER as *mut ScreenChar).add(offset) }
    }

    fn read_cell(row: usize, col: usize) -> ScreenChar {
        unsafe { ptr::read_volatile(Self::cell_ptr(row, col)) }
    }

    fn write_cell(row: usize, col: usize, character: ScreenChar) {
        unsafe { ptr::write_volatile(Self::cell_ptr(row, col), character) }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                Self::write_cell(
                    row,
                    col,
                    ScreenChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    },
                );
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = Self::read_cell(row, col);
                Self::write_cell(row - 1, col, character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            Self::write_cell(row, col, blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

static WRITER: Lazy<Mutex<Writer>> = Lazy::new(|| {
    Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
    })
});

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("Error al escribir en el buffer VGA");
    });
}

/// Macro global `print!` para la pantalla VGA.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::drivers::vga::_print(format_args!($($arg)*));
    };
}

/// Macro global `println!` para la pantalla VGA.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
