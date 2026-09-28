# RustOS

A minimal operating system written in Rust, designed for learning and experimentation with systems programming.

## Overview

This project is a simple operating system kernel implemented in Rust. It demonstrates core OS concepts including:
- Kernel initialization
- Memory management
- Basic device drivers
- System calls
- Process scheduling

## Features

- Bare-metal Rust development
- Custom bootloader
- Memory protection and virtual memory
- Basic I/O operations
- Interrupt handling
- System call interface

## Getting Started

El orden de trabajo está en [PLAN.md](PLAN.md). Hoy solo existe la etapa 0: un binario freestanding.

```bash
cargo build
```

QEMU entra en la etapa 1, cuando conectes el crate `bootloader`.

## Project Structure

```
src/main.rs            entrada (etapa 0)
src/lib.rs             módulos del kernel
src/drivers/           VGA, serie, teclado
src/interrupts/        IDT y PIC
src/gdt.rs             GDT y TSS
src/memory/            frames, paginación, heap
src/task/              tareas cooperativas
src/process/           anillo 3
src/syscall.rs         llamadas al sistema
bootloader/            notas de arranque
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.# Rustos
