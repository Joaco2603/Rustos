# Arranque (etapa 1)

Este directorio no contiene el bootloader: lo genera el crate `bootloader` 0.9
vía `bootimage`. Aquí quedan notas de lo que esa etapa tiene que dejarte claro.

## Qué ocurre antes de tu código

1. El firmware (BIOS/UEFI) inicializa la máquina y carga el bootloader.
2. El bootloader entra en modo largo (64 bits), identidad-mapea el kernel
   y construye un mapa de memoria física.
3. Salta a `_start` con un puntero a `BootInfo`.

## Qué escribes tú

En `src/main.rs`, sustituye el `_start` actual por `bootloader::entry_point!`
y registra el target runner de `.cargo/config.toml`.

Comprueba con:

```bash
rustup component add llvm-tools-preview
cargo install bootimage
cargo run
```

QEMU debe abrir una ventana. La salida serie (`-serial stdio`) es el primer
canal de depuración; va en `src/drivers/serial.rs`.
