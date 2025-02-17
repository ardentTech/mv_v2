#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m as _;
use cortex_m_rt::entry;
use defmt_rtt as _;
use defmt::info;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use microbit::Board;
use microbit::hal::Timer;
use panic_halt as _;

// WARNING may overflow and wrap-around in long-lived apps
static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", COUNT.fetch_add(1, Ordering::Relaxed));

#[entry]
fn main() -> ! {
    info!("starting up...");
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // switch row1 on
    board.display_pins.row1.set_high().unwrap();

    let mut led1 = board.display_pins.col1;
    let mut led2 = board.display_pins.col2;

    loop {
        if let Ok(true) = board.buttons.button_a.is_high() {
            info!("button a is depressed");
            let _ = led1.set_high();
        } else {
            info!("button a is pressed");
            let _ = led1.set_low();
        }

        if let Ok(true) = board.buttons.button_b.is_high() {
            info!("button b is depressed");
            let _ = led2.set_high();
        } else {
            info!("button b is pressed");
            let _ = led2.set_low();
        }
        timer.delay_ms(1000u32);
    }
}
