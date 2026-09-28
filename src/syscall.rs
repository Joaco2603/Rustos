//! Etapa 8 — llamadas al sistema.
//!
//! El código de usuario no puede hablar con el hardware.
//! Cruza a anillo 0 con `int 0x80` o la instrucción `syscall`, y el kernel
//! despacha `write`, `exit`, `yield` según el número en un registro.
