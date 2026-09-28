//! Etapa 3 — Global Descriptor Table y Task State Segment.
//!
//! El TSS guarda el stack que la CPU usa si ocurre un double fault.
//! Sin eso, un fallo dentro del handler de page fault triple-fault y resetea.
