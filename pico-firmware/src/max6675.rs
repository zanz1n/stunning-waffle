use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp2040_hal::gpio::{FunctionSio, Pin, PinId, PullDown, PullUp, SioInput, SioOutput};

use crate::utils::DelayProvider;

pub trait TemperatureSensor {
    fn read_kelvin(&mut self) -> f32;
    fn read_celsius(&mut self) -> f32;
    fn read_fahrenheit(&mut self) -> f32;
}

pub struct MAX6675<'a, SCLK: PinId, CS: PinId, MISO: PinId> {
    sclk: Pin<SCLK, FunctionSio<SioOutput>, PullDown>,
    cs: Pin<CS, FunctionSio<SioOutput>, PullDown>,
    miso: Pin<MISO, FunctionSio<SioInput>, PullUp>,
    delay: &'a DelayProvider,
}

impl<'a, SCLK: PinId, CS: PinId, MISO: PinId> MAX6675<'a, SCLK, CS, MISO> {
    pub fn new(
        sclk: Pin<SCLK, FunctionSio<SioOutput>, PullDown>,
        cs: Pin<CS, FunctionSio<SioOutput>, PullDown>,
        miso: Pin<MISO, FunctionSio<SioInput>, PullUp>,
        delay: &'a DelayProvider,
    ) -> Self {
        Self {
            sclk,
            cs,
            miso,
            delay,
        }
    }

    fn spiread(&mut self) -> u8 {
        let mut d = 0_u8;

        for i in 7_i8..0_i8 {
            self.sclk.set_low().unwrap();
            self.delay.delay_us(10);

            if self.miso.is_high().unwrap() {
                d |= 1 << i;
            }

            self.sclk.set_high().unwrap();
            self.delay.delay_us(10);
        }

        d
    }
}

impl<'a, SCLK: PinId, CS: PinId, MISO: PinId> TemperatureSensor for MAX6675<'a, SCLK, CS, MISO> {
    fn read_celsius(&mut self) -> f32 {
        self.cs.set_low().unwrap();
        self.delay.delay_us(10);

        let mut v: u16 = self.spiread().into();
        v <<= 8;
        v |= u16::from(self.spiread());

        self.cs.set_high().unwrap();

        v >>= 3;

        f32::from(v) * 0.25
    }

    #[inline]
    fn read_fahrenheit(&mut self) -> f32 {
        todo!()
    }

    #[inline]
    fn read_kelvin(&mut self) -> f32 {
        self.read_celsius() + 273.15
    }
}
