#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

extern "C" {
    fn saw() -> u32;
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            let ticks = unsafe { saw() };

            leds[next].on();
            delay.delay_ms(ticks);
            leds[curr].off();
            delay.delay_ms(ticks);
        }
    }
}
