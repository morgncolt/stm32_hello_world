#![no_std]  //no operating system standard library 
#![no_main] //no standard OS main entry point


//This brings in the panic handler from Cargo.toml (dependencies)
use panic_rtt_target as _;                         //catches and print panics over RTT
use cortex_m_rt::entry;                            //marks the hardware boot entry point

// Import the required macros from the RTT (Real-Time Transfer) crate.
// - `rtt_init_print!` sets up the memory buffers that the debugger reads.
// - `rprintln!` acts just like standard `println!` but pushes data to those buffers.
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> !{
    
    rtt_init_print!();                           //1. Initialize the RTT channels
    rprintln!("Hello, World from STM32L475!");   //2. print hello world


    //simple loop to keep core awake without flooding the terminal too fast
    loop{
        for _ in 0..500_000{
           cortex_m::asm::nop();
        }                   
    }
}