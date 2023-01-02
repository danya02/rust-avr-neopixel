#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(trait_alias)]
use panic_halt as _;

mod neopixel;
mod color;

use neopixel::NeoPixelWriter;
use neopixel::Brg;
use arduino_hal::port::{Pin, mode::Output};
use ufmt::uwriteln;

pub type OutputPin = Pin<Output, arduino_hal::hal::port::PB5>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    //let mut led = pins.d13.into_output();
    let mut neopixel: NeoPixelWriter<Brg> = NeoPixelWriter::new(pins.d13.into_output());

    loop {
        uwriteln!(&mut serial, "Loop!").unwrap();
        neopixel.write(color::Rgb24::new(255, 255, 255));
        neopixel.wait();
    }
    loop{
        //led.toggle();
        //arduino_hal::delay_ms(1000);
        neopixel.write(color::Rgb24::new(0, 0, 0));
        neopixel.wait();
        //led.toggle();
        //arduino_hal::delay_ms(1000);
    }
}
