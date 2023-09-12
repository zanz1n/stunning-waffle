#![no_std]
#![no_main]

mod messaging;
mod utils;

use cortex_m::delay::Delay;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::prelude::_embedded_hal_adc_OneShot;
use messaging::Payload;
use panic_probe as _;
use rp2040_hal::{
    adc::AdcPin, clocks::init_clocks_and_plls, gpio::Pins, pac, usb::UsbBus, Adc, Clock, Sio,
    Watchdog,
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

    let usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut delay = DelayProvider::new(delay, usb_dev);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut buildin_led = pins.gpio25.into_push_pull_output();

    let mut buffer: [u8; 2048] = [0; 2048];

    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);

    let mut sensor1_in = AdcPin::new(pins.gpio26.into_floating_input());

    loop {
        let value: u16 = adc.read(&mut sensor1_in).unwrap();

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

        buildin_led.set_high().unwrap();
        delay.sleep_ms(500, &mut [&mut serial]);
        buildin_led.set_low().unwrap();
        delay.sleep_ms(500, &mut [&mut serial]);

        clear_slice(&mut buffer);
    }
}
