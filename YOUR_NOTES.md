10/22/2026

probe-rs #############################################################################
probe-rs is a cross-platform CLI tool (Rust-based), not a Linux built-in — you run it in your regular shell like any installed program.  
If it's not found, you may need ~/.cargo/bin on your PATH.  

m@m-ThinkStation-P8:~$ probe-rs list
The following debug probes were found:
[0]: STLink V2-1 -- 0483:374b:0668FF303555483043215229 (ST-LINK)
m@m-ThinkStation-P8:~$ 
lsusb #############################################################################

lsusb - list all usb devices. This also works and you can use Grep

m@m-ThinkStation-P8:~$ lsusb | grep STM
Bus 001 Device 003: ID 0483:374b STMicroelectronics ST-LINK/V2.1
m@m-ThinkStation-P8:~$ 

Cargo init#######################################################################
m@m-ThinkStation-P8:~/rust_embedded/hello_board$ cargo init
    Creating binary (application) package
error: `cargo init` cannot be run on existing Cargo packages
help: use `cargo new` to create a package in a new subdirectory
m@m-ThinkStation-P8:~/rust_embedded/hello_board$ 

*** Just scaffolds a normal generic rust package with empty dependencies with standard
fn main() {println!("Hello World");} includes .gitignore and git repo
you then have to add the right crates and flip it to #![no_std]/#![no_main] yourself to make it target the chip instead of your machine.     

#![no_std] is used because bare metal targets such as the stm do not have an os or std library so
and #![no_main] is needed becuase bare metal does not have a starting point for main   
Crates Need#######################################################################

One of the creates https://crates.io/search?q=cortex-M
the command to add this dependency is cargo add cortex-m

then there is this https://crates.io/crates/cortex-m-rt
cargo add cortex-m-rt

https://crates.io/crates/hisi-panic-handler (panic handler)
panic-itm (another panic handler)

cortex-m-semihosting is a real match — Cortex-M specific, correct processor family for your chip.   

full package
cortex-m = "0.7.9"
cortex-m-rt = "0.7.7"
panic-rtt-target = "0.2.0"
rtt-target = "=0.6.2"

memory.x#######################################################################

This a  file is a linker script configuration file used in embedded Rust to define the exact physical memory layout (address ranges and sizes) of the target microcontroller.

IF address layout too small  file is a linker script configuration file used in embedded Rust to define the exact physical memory layout (address ranges and sizes) of the target microcontroller.

fails quiet. The linker is happy (it fits in the declared region), the build reports success, and nothing goes wrong until your  
code or data actually grows into addresses the chip doesn't physically have

cargo run#######################################################################

.cargo/config.toml's runner setting under [target.<triple>] swaps out "just execute this binary" for a custom command 
that gets handed the compiled ELF path instead.                                                                                          
(ST-Link V2.1) speaks SWD (Serial Wire Debug), the 2-wire ARM-specific debug     
protocol. JTAG is the older, more generic standard; SWD is what's actually used here.  

RTT rides the existing SWD connection through the debug probe (ST-Link/J-Link), reading a memory-mapped ring buffer on the     
target. No UART peripheral or extra wiring needed. That's all five items

#######################################################################
Now it's writing it: memory.x, main.rs (#![no_std]/#![no_main], the #[entry] function, RTT init, the panic        
handler), and .cargo/config.toml's runner. 
#######################################################################


hello-world-embedded/
├── .cargo/
│   └── config.toml      # Target architecture and runner configuration
├── src/
│   └── main.rs          # Embedded entry point and panic handler
├── Cargo.toml           # Project metadata and dependencies (e.g., cortex-m, panic-halt)
├── memory.x             # Linker script defining Flash and RAM layout (chip-specific)
└── build.rs             # Build script (optional, depending on target)



COMMAND TO RUN 
m@m-ThinkStation-P8:~/rust_embedded/hello_board$ rustup target add thumbv7em-none-eabihf
info: downloading component rust-std
     rust-std installed                       12.79 MiB                                                                            m@m-ThinkStation-P8:~/rust_embedded/hello_board$ 

