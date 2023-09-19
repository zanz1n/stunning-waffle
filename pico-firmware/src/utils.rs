use core::cell::RefCell;
use cortex_m::delay::Delay;

pub struct DelayProvider {
    delay: RefCell<Delay>,
}

impl DelayProvider {
    pub fn new(delay: Delay) -> Self {
        Self {
            delay: RefCell::new(delay),
        }
    }

    pub fn delay_ms(&self, ms: u32) {
        self.delay.borrow_mut().delay_ms(ms)
    }

    pub fn delay_us(&self, us: u32) {
        self.delay.borrow_mut().delay_ms(us)
    }
}

pub fn clear_slice(s: &mut [u8]) {
    for ele in s {
        *ele = 0;
    }
}
