


use crate::color::Rgb24;
use crate::OutputPin;

/// Module for working with NeoPixel LEDs.

/// NeoPixels is a struct that represents a strip of NeoPixel LEDs.
/// It is generic over the type of the data pin, which must implement the
/// `OutputPin` trait.
pub struct NeoPixelWriter<ORDER> {
    pin: OutputPin,
    order: core::marker::PhantomData<ORDER>,
}

/// Determines the order of the RGB components in the NeoPixel data stream.
pub trait PixelOrder {
    /// Send the RGB components in the correct order.
    unsafe fn send_color(writer: &mut NeoPixelWriter<Self>, color: Rgb24) where Self: Sized;
}

/// BRG order. Used for most WS2812Bs.
pub struct Brg;

impl PixelOrder for Brg {
    unsafe fn send_color(writer: &mut NeoPixelWriter<Self>, color: Rgb24) {
        let mut bytes = [false; 24];
        for i in 0..8 {
            bytes[i] = color.b & (1 << i) != 0;
            bytes[i + 8] = color.r & (1 << i) != 0;
            bytes[i + 16] = color.g & (1 << i) != 0;
        }
        writer.write_bytes(bytes);
    }
}

impl<ORDER> NeoPixelWriter<ORDER>
where
    ORDER: PixelOrder,
{
    /// Create a new `NeoPixels` instance.
    ///
    /// # Arguments
    ///
    /// * `pin` - The data pin to use for the NeoPixels.
    /// * `num_leds` - The number of LEDs in the strip.
    pub fn new(pin: OutputPin) -> Self {
        Self {
            pin,
            order: core::marker::PhantomData,
        }
    }

    /// Internal: Wait for 0.8us (800ns) using busy-waiting.
    #[inline(always)]
    unsafe fn wait_800ns(&self) {
        // 800ns at 16MHz is 12.8 cycles.
        // Wait for 12 cycles: the other code will make up the rest.
        core::arch::asm!(
            "nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop",
            options(nomem, nostack)
        );
    }

    /// Internal: Wait for 0.4us (400ns) using busy-waiting.
    #[inline(always)]
    unsafe fn wait_400ns(&self) {
        // 400ns at 16MHz is 6.4 cycles.
        // Wait for 6 cycles: the other code will make up the rest.
        core::arch::asm!(
            "nop
            nop
            nop
            nop
            nop
            nop
            nop",
            options(nomem, nostack)
        );
    }

    /// Internal: Wait for 0.05us (50ns) using busy-waiting.
    #[inline(always)]
    unsafe fn wait_50ns(&self) {
        // 50ns at 16MHz is 0.8 cycles.
        // Wait for 1 cycle.
        core::arch::asm!("nop", options(nomem, nostack));
    }

    /// Internal: Write a `one` bit to the NeoPixel data pin.
    /// This is 0.8us high, 0.45us low.
    #[inline(always)]
    unsafe fn write_one(&mut self) {
        self.pin.set_high();
        self.wait_800ns();
        self.pin.set_low();
        self.wait_400ns();
        self.wait_50ns();
    }

    /// Internal: Write a `zero` bit to the NeoPixel data pin.
    /// This is 0.4us high, 0.85us low.
    #[inline(always)]
    unsafe fn write_zero(&mut self) {
        self.pin.set_high();
        self.wait_400ns();
        self.pin.set_low();
        self.wait_800ns();
        self.wait_50ns();
    }

    /// Internal: Write three bytes to the NeoPixel data pin.
    /// The bytes are laid out as bools for efficiency.
    #[inline(always)]
    unsafe fn write_bytes(&mut self, bytes: [bool; 24]) {
        for byte in bytes.iter() {
            if *byte {
                self.write_one();
            } else {
                self.write_zero();
            }
        }
    }

    /// Write a color to the NeoPixel data pin.
    pub fn write(&mut self, color: Rgb24) {
        unsafe {
            ORDER::send_color(self, color);
        }
    }

    /// Signal the end of data to send to the NeoPixels.
    /// This is 300us of the data pin being low.
    pub fn wait(&mut self) {
        self.pin.set_low();
        arduino_hal::delay_us(300);
    }

}