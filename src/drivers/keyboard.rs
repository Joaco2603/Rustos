//! Etapa 4 — controlador de teclado PS/2.
//!
//! La IRQ1 entrega un scancode, no un carácter Unicode.
//! Traduce el conjunto 1 a caracteres y distingue make/break (tecla suelta).
