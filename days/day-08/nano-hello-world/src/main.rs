#![no_main]
#![no_std]

use panic_halt as _;

use hal::{
    delay::Delay,
    gpio::Level,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
    temp::Temp,
    uarte,
};
use nrf52840_hal as hal;

use core::fmt::Write;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);

    // Setup UART
    let (uart0, cdc_pins) = {
        (
            p.UARTE0,
            uarte::Pins {
                txd: port1.p1_03.into_push_pull_output(Level::High).degrade(),
                rxd: port1.p1_10.into_floating_input().degrade(),
                cts: None,
                rts: None,
            },
        )
    };

    let mut uarte = uarte::Uarte::new(
        uart0,
        cdc_pins,
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    let mut led = port0.p0_06.into_push_pull_output(Level::High); // blue LED, active low

    // Write on the serial
    write!(uarte, "Hello, World!\r\n").unwrap();

    // Access to the temp sensor
    let mut temp_sensor = Temp::new(p.TEMP);
    let mut temp_value = 0;
    let mut delay = Delay::new(core.SYST);

    loop {
        //set_low turns the LED off
        led.set_low().unwrap();
        delay.delay_ms(500_u32);

        //set_high turns the LED on
        led.set_high().unwrap();
        delay.delay_ms(1000_u32);

        // Read temperature value and print it on UART
        temp_value = temp_sensor.measure().to_num();
        write!(uarte, "Temp: {}\r\n", temp_value).unwrap();
    }
}
