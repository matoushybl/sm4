#![no_main]
#![no_std]

use sm4_firmware as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    sm4_firmware::exit()
}
