#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _;
// global logger
use panic_probe as _;

pub use sm4::SM4;

mod blocks;
mod board;
mod can;
mod i2c;
mod object_dictionary;
mod protocol;
mod sm4;
mod state;

pub mod prelude {
    pub use crate::blocks::*;
    pub use crate::board::*;
    pub use crate::object_dictionary::ObjectDictionary;
    pub use crate::protocol::*;
    pub use crate::state::DriverState;
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});
