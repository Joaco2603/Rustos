//! Etapas 3 y 4 — IDT, excepciones de CPU e IRQs del PIC 8259.
//!
//! La IDT es un arreglo de puertas. La CPU indexa con el vector
//! (0 división, 3 breakpoint, 14 page fault, 32.. timer y teclado).
