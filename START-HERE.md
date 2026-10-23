# Hello world on the STM32L475VGT — you write it

Onu wrote this project for you on 2026-10-21 and then removed it, because you had said an hour
earlier that you wanted AI out of the loop. The setup *is* the lesson: linker scripts, the memory
map, the entry point. Having it handed over teaches nothing.

**The working version is sealed in `../hello_board_solution/`. Do not open it until you either
have output on screen or have been genuinely stuck for 30 minutes.** Then diff it.

## The goal

Print a line from the board, over the ST-Link, with no operating system and no heap.

## Facts worth being given — these are lookups, not understanding

Hunting these costs an evening and teaches nothing about Rust.

- **Chip name for probe-rs: `STM32L475VG`.** Not `STM32L475VGTx` — that name is rejected.
  Confirm yourself with `probe-rs chip list | grep L475`.
- **Memory map** (ST datasheet DS10969): flash **1024 KB at `0x08000000`**; SRAM1 **96 KB at
  `0x20000000`**; SRAM2 32 KB at `0x10000000`, parity-checked. Leave SRAM2 out for now.
- **Target triple: `thumbv7em-none-eabi`** — Cortex-M4F. Already installed.
- Installed and ready: `rustc 1.98.1`, `probe-rs 0.32.0`, `cargo-embed`, `cargo-flash`.

## What you have to work out

1. `cargo init`, then which crates you need for a bare-metal Cortex-M binary. Four of them:
   a runtime, a core support crate, something to carry output off-chip, and a panic handler.
2. Why `#![no_std]` and `#![no_main]` are both needed, and what replaces `fn main`.
3. What `memory.x` is for, and what breaks if the lengths are wrong.
4. How `cargo run` ends up flashing a board — the runner setting.
5. Why your output arrives on the host when there is no UART wired up.

## How to know you are done

```
readelf -h target/thumbv7em-none-eabi/release/<name> | grep Entry
```

**If the entry point is `0x0`, the image is broken even if the build said "Finished".** It will
hard-fault the moment it runs. There is a linker script that places the vector table, and
`rustflags` has to ask for it. You will probably hit this. Read the warnings `rust-lld` prints,
not just the last line of cargo output.

A correct image has `.vector_table` at `0x08000000` and an entry point just above `0x08000400`.

## The rule

Compiling is not working, and a build that reports success is not an artifact. Check the ELF.
