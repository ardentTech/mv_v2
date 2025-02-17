#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m as _;
use cortex_m_rt::entry;
use defmt_rtt as _;
use defmt::{info, warn};
use panic_halt as _;

// WARNING may overflow and wrap-around in long-lived apps
static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", COUNT.fetch_add(1, Ordering::Relaxed));

#[entry]
fn main() -> ! {
    info!("starting up...");
    loop {}
}
