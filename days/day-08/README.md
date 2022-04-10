# Day 08 - Arduino 33 BLE setup - 2

As mentionned yesterday, my goal today will be to get a deeper understanding on how the flashing process work (why do we need a bootloader, how to we communicate with hit, what's the boot sequence that allows use to exectute code on the board).

First things first, let's assume that the bootloader is given (we will see after what's inside and how to change it), how the flashing process works 🤔

## Flash and run new code on the board

Yesterday we saw that the command used to flash the binary was the following

```
/.../bossac -d --port=cu.usbmodem1441301 -U -i -e -w /.../Blink.ino.bin -R 
```

**Let's find out what is `bossac`** 

Looking on the web, I found that `bossac` is

> BOSSA is a flash programming utility for Atmel's SAM family of flash-based ARM microcontrollers. The motivation behind BOSSA is to create a simple, easy-to-use, open source utility to replace Atmel's SAM-BA software. BOSSA is an acronym for Basic Open Source SAM-BA Application to reflect that goal.

Well so that's a tool to interact with the board bootloader, it doesn't give use much informations about what's inside the bootloader but here is what we can do with this command line

```
Usage: bossac [OPTION...] [FILE]
Basic Open Source SAM-BA Application (BOSSA) Version 1.9.1-17-g89f3556
Flash programmer for Atmel SAM devices.
Copyright (c) 2011-2018 ShumaTech (http://www.shumatech.com)

Examples:
  bossac -e -w -v -b image.bin   # Erase flash, write flash with image.bin,
                                 # verify the write, and set boot from flash
  bossac -r0x10000 image.bin     # Read 64KB from flash and store in image.bin

Options:
  -e, --erase           erase the entire flash starting at the offset
  -w, --write           write FILE to the flash; accelerated when
                        combined with erase option
  -r, --read[=SIZE]     read SIZE from flash and store in FILE;
                        read entire flash if SIZE not specified
  -v, --verify          verify FILE matches flash contents
  -o, --offset=OFFSET   start erase/write/read/verify operation at flash OFFSET;
                        OFFSET must be aligned to a flash page boundary
  -p, --port=PORT       use serial PORT to communicate to device;
                        default behavior is to use first serial port
  -b, --boot[=BOOL]     boot from ROM if BOOL is 0;
                        boot from FLASH if BOOL is 1 [default];
                        option is ignored on unsupported devices
  -c, --bod[=BOOL]      no brownout detection if BOOL is 0;
                        brownout detection is on if BOOL is 1 [default]
  -t, --bor[=BOOL]      no brownout reset if BOOL is 0;
                        brownout reset is on if BOOL is 1 [default]
  -l, --lock[=REGION]   lock the flash REGION as a comma-separated list;
                        lock all if not given [default]
  -u, --unlock[=REGION] unlock the flash REGION as a comma-separated list;
                        unlock all if not given [default]
  -s, --security        set the flash security flag
  -i, --info            display device information
  -d, --debug           print debug messages
  -U, --usb-port[=BOOL] force serial port detection to USB if BOOL is 1 [default]
                        or to RS-232 if BOOL is 0
  -R, --reset           reset CPU (if supported)
  -a, --arduino-erase   erase and reset via Arduino 1200 baud hack
  -h, --help            display this help text
  -V, --version         display version info

Report bugs to <bugs@shumatech.com>
```

I thought I would be able to find the source code of the booloader of the Arduino nano 33 BLE but I couldn't and it seems that I'm not the only one as I found in [this issue](https://github.com/arduino/ArduinoCore-nRF528x-mbedos/issues/23).

## ARM flashing process

More on this on another day as I didn't have time to dig into.

Some usefull ressources:

- https://electronics.stackexchange.com/questions/509784/stm32-flash-vs-sram



## Flashing Rust code

Here is the simple example code that I used. It does two things:

- Turn on and off a led
- Wait a given amount of time between each state
- Read the temperature on the temp sensor

```Rust
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
```



Few things to notice here:

- It's a no `#![no_std]` code meaning that the standard library is not included in the binary and can not be use (particularly, there is no dynamic allocation available)
- It's a `#![no_main]` binary. Rust usually needs this keyword to generate the final binary entry, this flag indicates to the compiler that it should not look for it here
- The main function returns nothing `fn main() -> ! {}`.
- The main function is marked as such by the `#[entry]` keyword (we could have named the funtion `hello` it wouldn't change the behavior of the program.



Regarding the cargo configuration I used this one

```
[target.thumbv7em-none-eabihf]

[build]
target = "thumbv7em-none-eabihf"
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

Nothing very new compared to the previous configuration:

- Here our taget is now `thumbv7em-none-eabihf`
- We add specific flags to indicate a linker script file `"link-arg=-Tlink.x"`

For the linker script I used the following configuration

```
MEMORY
{
  FLASH (rx) : ORIGIN = 0x10000, LENGTH = 0xf0000
  RAM_NVIC (rwx) : ORIGIN = 0x20000000, LENGTH = 0x100
  RAM_CRASH_DATA (rwx) : ORIGIN = (0x20000000 + 0x100), LENGTH = 0x100
  RAM (rwx) : ORIGIN = ((0x20000000 + 0x100) + 0x100), LENGTH = (0x40000 - (0x100 + 0x100))
}
```

