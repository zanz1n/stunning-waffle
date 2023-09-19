#![no_std]
#![no_main]

mod max6675;
mod messaging;
mod utils;

use cortex_m::delay::Delay;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use max6675::{TemperatureSensor, MAX6675};
use messaging::Payload;
use panic_probe as _;
use rp2040_hal::{
    clocks::init_clocks_and_plls, gpio::Pins, pac, usb::UsbBus, Clock, Sio, Watchdog,
};
use serde_json_core::to_slice;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::SerialPort;
use utils::{clear_slice, DelayProvider};

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let usb_bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let usb_bus = UsbBusAllocator::new(usb_bus);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let delay = DelayProvider::new(delay);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio25.into_push_pull_output();

    let mut sensor1 = MAX6675::new(
        pins.gpio2.into_push_pull_output(),
        pins.gpio3.into_push_pull_output(),
        pins.gpio4.into_pull_up_input(),
        &delay,
    );

    let mut buffer: [u8; 1024] = [0; 1024];

    loop {
        let value = sensor1.read_celsius();

        let written_pos = to_slice(
            &Payload {
                temperature_1: value,
            },
            &mut buffer,
        )
        .unwrap();

        buffer[written_pos] = 13; // \r char
        buffer[written_pos + 1] = 10; // \n char

        _ = serial.write(&buffer);

        {
            let mut ms = 500;
            while ms > 0 {
                ms -= 8;
                delay.delay_ms(8);
                usb_dev.poll(&mut [&mut serial]);
            }
        }

        led_pin.set_high().unwrap();

        {
            let mut ms = 500;
            while ms > 0 {
                ms -= 8;
                delay.delay_ms(8);
                usb_dev.poll(&mut [&mut serial]);
            }
        }

        led_pin.set_low().unwrap();

        clear_slice(&mut buffer);
    }
}
