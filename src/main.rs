#![no_std]
#![no_main]

use rp_pico::entry;
use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    info!("Program started...");

    loop {}
}
