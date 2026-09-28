# Plan: aprender un sistema operativo escribiéndolo en Rust

Objetivo: un kernel propio para `x86_64`, arrancado en QEMU, escrito a mano.
Cada etapa agrega un mecanismo del sistema y deja la anterior funcionando.
No avances de etapa hasta poder explicar en voz alta qué hace el código que acabas de escribir.

Referencia principal (sigue el orden, no copies el resultado):
[Writing an OS in Rust](https://os.phil-opp.com/).
Manuales de hardware: Intel SDM volumen 3 (interrupciones, paginación, GDT)
y la especificación UEFI/Multiboot solo como lectura; el arranque lo hace el crate `bootloader`.

## Cómo trabajar

1. Lee el capítulo de la etapa.
2. Escribe el módulo que corresponde (ver mapa abajo).
3. `cargo run` en QEMU y comprueba el comportamiento esperado.
4. Anota en un comentario corto del módulo: qué invariante mantiene.

Herramientas: Rust nightly, target `x86_64-unknown-none`, QEMU (`qemu-system-x86_64`),
`cargo install bootimage` cuando llegues a la etapa 1.

## Etapas

### 0 — Rust sin sistema operativo

Un binario `no_std` / `no_main` que no enlaza la libc.
Panic handler propio y un bucle infinito.
Aprende: qué aporta el runtime de Rust que un kernel no tiene (allocator global, `std`, hilos).

Directorio: `src/main.rs`.

Listo cuando: `cargo build` produce un binario freestanding.

### 1 — Arranque

El firmware carga un bootloader; el bootloader pone la CPU en modo largo (64 bits),
arma una tabla de páginas mínima y salta a tu `_start` con un `BootInfo`
(mapa de memoria física).

Directorio: `bootloader/README.md`, `.cargo/config.toml`.

Listo cuando: QEMU muestra un mensaje tuyo en la salida serie (`-serial stdio`).

### 2 — Salida en pantalla

El buffer de texto VGA en `0xb8000`: 80×25 celdas de carácter + color.
Un `Writer` con bloqueo (`spin::Mutex`) para poder imprimir desde cualquier parte.

Directorio: `src/drivers/vga.rs`.

Listo cuando: `println!` pinta texto y hace scroll.

### 3 — Excepciones de CPU

La CPU, ante un fallo (división por cero, page fault, breakpoint), consulta la
IDT (Interrupt Descriptor Table) y llama a tu handler.
Antes hace falta una GDT con un TSS para el stack de double fault.

Directorios: `src/gdt.rs`, `src/interrupts/mod.rs`.

Listo cuando: `int3` entra a tu handler y un page fault no resetea la máquina
(double fault controlado).

### 4 — Interrupciones de hardware

El PIC 8259 (luego el APIC) avisa teclado y timer.
Hay que enmascarar, registrar handlers y mandar EOI; si no, la IRQ se repite.

Directorios: `src/interrupts/mod.rs`, `src/drivers/keyboard.rs`.

Listo cuando: el timer cuenta ticks y el teclado imprime scancodes.

### 5 — Memoria física y virtual

El bootloader ya paginó el kernel. Tú recorres el mapa de memoria, marcas frames
usados y construyes un asignador de frames (lista o bitmap).
Después, tablas de páginas propias: mapear, desmapear, traducir.

Directorio: `src/memory/`.

Listo cuando: puedes mapear una página, escribirla y leerla por la dirección virtual.

### 6 — Heap del kernel

`GlobalAlloc` encima del asignador de frames (lista enlazada o bloques fijos).
A partir de aquí existen `Box`, `Vec` y `String` en el kernel.

Directorio: `src/memory/allocator.rs`.

Listo cuando: un `Vec` crece sin pisar el código del kernel.

### 7 — Concurrencia cooperativa

Sin procesos todavía: tareas que ceden la CPU (`yield`) en un ejecutor
asíncrono mínimo, o un round-robin de stacks propios.
El timer de la etapa 4 es la preempción más adelante.

Directorio: `src/task/`.

Listo cuando: dos tareas se alternan y ambas imprimen.

### 8 — Procesos y llamadas al sistema

Anillo 3 (usuario) contra anillo 0 (kernel).
Una syscall es una interrupción (`int 0x80`) o `syscall` que cambia de privilegio,
copia argumentos y vuelve con `sysret` / `iretq`.

Directorios: `src/syscall.rs`, `src/process/`.

Listo cuando: un programa mínimo en usuario pide `write` y el kernel lo imprime.

## Mapa de directorios

```
src/main.rs                 etapa 0–1   entrada
src/lib.rs                  biblioteca del kernel
src/gdt.rs                  etapa 3     segmentos y TSS
src/drivers/vga.rs          etapa 2     texto VGA
src/drivers/keyboard.rs     etapa 4     teclado
src/drivers/serial.rs       etapa 1     puerto COM1 para depurar
src/interrupts/mod.rs       etapa 3–4   IDT, PIC, excepciones
src/memory/mod.rs           etapa 5     frames y paginación
src/memory/allocator.rs     etapa 6     heap
src/task/mod.rs             etapa 7     tareas cooperativas
src/process/mod.rs          etapa 8     espacio de usuario
src/syscall.rs              etapa 8     interfaz de llamadas
bootloader/                 notas del arranque (el binario lo genera bootimage)
```

## Orden de lectura por concepto

| Concepto            | Dónde vive en hardware        | Etapa |
|---------------------|-------------------------------|-------|
| Modo largo, entrada | bootloader → `_start`         | 1     |
| MMIO                | VGA `0xb8000`                 | 2     |
| Tabla de vectores   | IDT + GDT                     | 3     |
| IRQ                 | PIC / IOAPIC                  | 4     |
| Aislamiento         | tablas de páginas, anillos    | 5 y 8 |
| Asignación          | frames → heap                 | 5–6   |
| Multiplexar la CPU  | timer + cambio de contexto    | 7–8   |

## Qué dejar para después

Sistema de archivos, red, varios núcleos (APIC + trampolín de AP),
un loader ELF de verdad y un shell. Cada uno es un proyecto aparte
cuando las ocho etapas imprimen, fallan de forma controlada y tienen tests con `bootimage test`.
