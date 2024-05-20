#![no_std]
#![no_main]

use cortex_m::asm::nop;
use panic_halt as _; 

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");

    loop {
        rprintln!("Looping...");
        for _ in 0..100_000 {
            nop();
        }
    }
}

