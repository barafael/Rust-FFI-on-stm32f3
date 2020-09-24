#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

/* Include the automatically generated bindings */
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
