//! Etapa 5 — frames físicos y tablas de páginas.
//!
//! El `BootInfo` trae regiones usables. Un frame allocator no devuelve
//! páginas ya ocupadas por el kernel o por el mapa de memoria.

pub mod allocator;
