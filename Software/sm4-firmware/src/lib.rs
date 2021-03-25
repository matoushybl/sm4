#![no_std]

mod board;
mod can;
mod current_reference;
mod leds;
mod monitoring;
mod object_dictionary;
mod sm4;
mod step_counter;
mod step_timer;
mod usb;

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _; // global logger
use panic_probe as _;

pub use sm4::SM4;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}
