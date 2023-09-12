use cortex_m::delay::Delay;
use rp2040_hal::usb::UsbBus;
use usb_device::{class_prelude::UsbClass, prelude::UsbDevice};

pub struct DelayProvider<'a> {
    delay: Delay,
    device: UsbDevice<'a, UsbBus>,
}

impl<'a> DelayProvider<'a> {
    pub fn new(delay: Delay, device: UsbDevice<'a, UsbBus>) -> Self {
        Self { delay, device }
    }

    pub fn sleep_ms(&mut self, mut ms: i32, pool_list: &mut [&mut dyn UsbClass<UsbBus>]) {
        while ms > 0 {
            ms -= 8;
            self.delay.delay_ms(8);
            self.device.poll(pool_list);
        }
    }
}

pub fn clear_slice(s: &mut [u8]) {
    for ele in s {
        *ele = 0;
    }
}
