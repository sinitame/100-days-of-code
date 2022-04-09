# Day 7 - Arduino 33 BLE setup - 1

Today I've tried to flash Rust code on the Arduino Nano 33 BLE



## Flash code using Arduino IDE

Let's start with the basics ! I decided to first try to flash the board with the default example from Arduino (the blink example) using the Arduino IDE.

Doing this is very simple, you just need to follow the documentation from the [Arduino website](https://www.arduino.cc/en/Guide/NANO33BLESense), basically you have to:

- Install the IDE
- Download the board package (here Arduino Mbed OS Core)
- Select the example you want to use in the menu (here Blink)
- Configure the IDE by specifying the board type (Arduino Nano 33 BLE) and the port (/dev/cu.usbmodelXXX)
- Click on Upload 

And tadaaaa, you should see the led blink ! 💡



It was quite easy but it didn't tell me much about what was happening in background and from what I know in embedded system programing things are never magical 🪄. So I decided to search a bit on what was happening and to do so I enabled the logs 🤓

<details>
<summary><b>Here is what I got for compilation</b></summary>



```
/private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/arduino-builder -dump-prefs -logger=machine -hardware /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/hardware -hardware /Users/sinitame/Library/Arduino15/packages -tools /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/tools-builder -tools /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/hardware/tools/avr -tools /Users/sinitame/Library/Arduino15/packages -built-in-libraries /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/libraries -libraries /Users/sinitame/Documents/Arduino/libraries -fqbn=arduino:mbed_nano:nano33ble -vid-pid=2341_805A -ide-version=10819 -build-path /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018 -warnings=all -build-cache /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_cache_236445 -prefs=build.warn_data_percentage=75 -prefs=runtime.tools.dfu-util.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/dfu-util/0.10.0-arduino1 -prefs=runtime.tools.dfu-util-0.10.0-arduino1.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/dfu-util/0.10.0-arduino1 -prefs=runtime.tools.arm-none-eabi-gcc.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4 -prefs=runtime.tools.arm-none-eabi-gcc-7-2017q4.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4 -prefs=runtime.tools.openocd.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/openocd/0.11.0-arduino2 -prefs=runtime.tools.openocd-0.11.0-arduino2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/openocd/0.11.0-arduino2 -prefs=runtime.tools.bossac.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2 -prefs=runtime.tools.bossac-1.9.1-arduino2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2 -prefs=runtime.tools.imgtool.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/imgtool/1.8.0-arduino -prefs=runtime.tools.imgtool-1.8.0-arduino.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/imgtool/1.8.0-arduino -prefs=runtime.tools.rp2040tools.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/rp2040tools/1.0.2 -prefs=runtime.tools.rp2040tools-1.0.2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/rp2040tools/1.0.2 -verbose /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/examples/01.Basics/Blink/Blink.ino
/private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/arduino-builder -compile -logger=machine -hardware /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/hardware -hardware /Users/sinitame/Library/Arduino15/packages -tools /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/tools-builder -tools /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/hardware/tools/avr -tools /Users/sinitame/Library/Arduino15/packages -built-in-libraries /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/libraries -libraries /Users/sinitame/Documents/Arduino/libraries -fqbn=arduino:mbed_nano:nano33ble -vid-pid=2341_805A -ide-version=10819 -build-path /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018 -warnings=all -build-cache /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_cache_236445 -prefs=build.warn_data_percentage=75 -prefs=runtime.tools.dfu-util.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/dfu-util/0.10.0-arduino1 -prefs=runtime.tools.dfu-util-0.10.0-arduino1.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/dfu-util/0.10.0-arduino1 -prefs=runtime.tools.arm-none-eabi-gcc.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4 -prefs=runtime.tools.arm-none-eabi-gcc-7-2017q4.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4 -prefs=runtime.tools.openocd.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/openocd/0.11.0-arduino2 -prefs=runtime.tools.openocd-0.11.0-arduino2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/openocd/0.11.0-arduino2 -prefs=runtime.tools.bossac.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2 -prefs=runtime.tools.bossac-1.9.1-arduino2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2 -prefs=runtime.tools.imgtool.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/imgtool/1.8.0-arduino -prefs=runtime.tools.imgtool-1.8.0-arduino.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/imgtool/1.8.0-arduino -prefs=runtime.tools.rp2040tools.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/rp2040tools/1.0.2 -prefs=runtime.tools.rp2040tools-1.0.2.path=/Users/sinitame/Library/Arduino15/packages/arduino/tools/rp2040tools/1.0.2 -verbose /private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/examples/01.Basics/Blink/Blink.ino
Using board 'nano33ble' from platform in folder: /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1
Using core 'arduino' from platform in folder: /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1
Detecting libraries used...
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++ -c -w -g3 -nostdlib @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/defines.txt @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/cxxflags.txt -DARDUINO_ARCH_NRF52840 -mcpu=cortex-m4 -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -w -x c++ -E -CC -DARDUINO=10819 -DARDUINO_ARDUINO_NANO33BLE -DARDUINO_ARCH_MBED_NANO -DARDUINO_ARCH_MBED -DARDUINO_LIBRARY_DISCOVERY_PHASE=1 -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated-avr-comp -iprefix/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/includes.txt /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/sketch/Blink.ino.cpp -o /dev/null
Generating function prototypes...
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++ -c -w -g3 -nostdlib @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/defines.txt @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/cxxflags.txt -DARDUINO_ARCH_NRF52840 -mcpu=cortex-m4 -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -w -x c++ -E -CC -DARDUINO=10819 -DARDUINO_ARDUINO_NANO33BLE -DARDUINO_ARCH_MBED_NANO -DARDUINO_ARCH_MBED -DARDUINO_LIBRARY_DISCOVERY_PHASE=1 -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated-avr-comp -iprefix/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/includes.txt /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/sketch/Blink.ino.cpp -o /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/preproc/ctags_target_for_gcc_minus_e.cpp
/private/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/AppTranslocation/FD901E4C-4F00-4F7F-84B7-841F5F8B55FF/d/Arduino.app/Contents/Java/tools-builder/ctags/5.8-arduino11/ctags -u --language-force=c++ -f - --c++-kinds=svpf --fields=KSTtzns --line-directives /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/preproc/ctags_target_for_gcc_minus_e.cpp
Compilation du croquis...
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++ -c -Wall -Wextra -g3 -nostdlib @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/defines.txt @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/cxxflags.txt -DARDUINO_ARCH_NRF52840 -MMD -mcpu=cortex-m4 -mfloat-abi=softfp -mfpu=fpv4-sp-d16 -DARDUINO=10819 -DARDUINO_ARDUINO_NANO33BLE -DARDUINO_ARCH_MBED_NANO -DARDUINO_ARCH_MBED -DARDUINO_LIBRARY_DISCOVERY_PHASE=0 -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated -I/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino/api/deprecated-avr-comp -iprefix/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/cores/arduino @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/includes.txt /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/sketch/Blink.ino.cpp -o /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/sketch/Blink.ino.cpp.o
Compiling libraries...
Compiling core...
Utilisation du fichier déjà compilé : /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/core/variant.cpp.o
Using precompiled core: /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_cache_236445/core/core_arduino_mbed_nano_nano33ble_a8211ef55d5cb723d9b59e4bc7c22fcb.a
Linking everything together...
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++ -E -P -x c /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/linker_script.ld -o /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/linker_script.ld
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++ -L/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018 -Wl,--gc-sections -Wall -Wextra -Wl,--as-needed @/Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/ldflags.txt -T/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/linker_script.ld -Wl,-Map,/var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.map --specs=nosys.specs -o /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.elf /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/sketch/Blink.ino.cpp.o /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/core/variant.cpp.o -Wl,--whole-archive /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/../arduino_cache_236445/core/core_arduino_mbed_nano_nano33ble_a8211ef55d5cb723d9b59e4bc7c22fcb.a /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/libs/libmbed.a /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/libs/libcc_310_core.a /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/libs/libcc_310_ext.a /Users/sinitame/Library/Arduino15/packages/arduino/hardware/mbed_nano/3.0.1/variants/ARDUINO_NANO33BLE/libs/libcc_310_trng.a -Wl,--no-whole-archive -Wl,--start-group -lstdc++ -lsupc++ -lm -lc -lgcc -lnosys -Wl,--end-group
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-objcopy -O binary /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.elf /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.bin
/Users/sinitame/Library/Arduino15/packages/arduino/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-objcopy -O ihex -R .eeprom /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.elf /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.hex
/Users/sinitame/Library/Arduino15/packages/arduino/tools/imgtool/1.8.0-arduino/imgtool exit
```

</details>

<details>
<summary><b> And here is what I got for flashing </b></summary>

```
Uploading using selected port: /dev/cu.usbmodem1441301
/Users/sinitame/Library/Arduino15/packages/arduino/tools/bossac/1.9.1-arduino2/bossac -d --port=cu.usbmodem1441301 -U -i -e -w /var/folders/wj/44g0x4sx2gqfwlndqmf0df700000gn/T/arduino_build_882018/Blink.ino.bin -R
```

</details>



#### That's a lot of lines but few of them are very informative

**Linker file preparation**

```
/.../bin/arm-none-eabi-g++ -E -P -x c /.../ARDUINO_NANO33BLE/linker_script.ld -o /.../linker_script.ld
```

Here is what the different options mean:

- **E**: Stop after the preprocessing stage; do not run the compiler proper. The output is in the form of preprocessed source code, which is sent to the standard output. Input files which don't require preprocessing are ignored.
- **P**: I didn't find this one 🤷🏻‍♂️
- **x**: Specify explicitly the language for the following input files (so here it's `c`)
- **o**: Output file name

I don't really understant the logic behind this command but from what I understant it just copies the linker script to the relevant project build directory.

What's inside is also very verbose, but it basically maps the symbols (which are relative addresses) from the objects files during linking to actual memory addresses on the device memory. The important thing here is this first block. I tells us the address of the RAM and the RAM

```
 MEMORY
{
   FLASH (rx) : ORIGIN = 0x10000, LENGTH = 0xf0000
   RAM_NVIC (rwx) : ORIGIN = 0x20000000, LENGTH = 0x100
   RAM_CRASH_DATA (rwx) : ORIGIN = (0x20000000 + 0x100), LENGTH = 0x100
   RAM (rwx) : ORIGIN = ((0x20000000 + 0x100) + 0x100), LENGTH = (0x40000 - (0x100 + 0x100))
}
```



**Source compilation**

```
/.../arm-none-eabi-g++ 
	-L/.../T/arduino_build_882018 -Wl
	--gc-sections -Wall -Wextra -Wl
	--as-need  @/.../ARDUINO_NANO33BLE/ldflags.txt -T/.../linker_script.ld -Wl
	-Map /.../Blink.ino.map
	--specs=nosys.specs 
	-o /.../Blink.ino.elf 
	/.../sketch/Blink.ino.cpp.o
	/.../variant.cpp.o -Wl
	--whole-archive /.../core_arduino_mbed_nano_nano33ble_a8211ef55d5cb723d9b59e4bc7c22fcb.a /.../libs/libmbed.a /.../libs/libcc_310_core.a /.../libs/libcc_310_ext.a /.../libs/libcc_310_trng.a -Wl
	--no-whole-archive -Wl
	--start-group -lstdc++ -lsupc++ -lm -lc -lgcc -lnosys -Wl
	--end-group
```

I won't go into too much details here but we basically use the compiled object file `Blink.ino.cpp.o` (which is the one containing our project code) and all the libraries and objects files that we need to build the final binary into `Blink.ino.elf`.

**Binary file generation**

```
/.../arm-none-eabi-objcopy -O binary /.../Blink.ino.elf /.../Blink.ino.bin
```

`objcopy` can be used to generate a raw binary file by using an output target of ‘binary’ (e.g., use -O binary). When `objcopy` generates a raw binary file, it will essentially produce a memory dump of the contents of the input object file. All symbols and relocation information will be discarded. The memory dump will start at the load address of the lowest section copied into the output file.

**Convert binary to intel hex format**

```
/.../arm-none-eabi-objcopy -O ihex -R .eeprom /.../Blink.ino.elf /.../Blink.ino.hex
```

**Get the size of the binary**

```
/.../arm-none-eabi-size -A /.../Blink.ino.elf
```

**Flash the board with it**

```
/.../bossac -d --port=cu.usbmodem1441301 -U -i -e -w /.../Blink.ino.bin -R 
```



I have managed to flash some Rust code on the board but it took me some time to figure out how to do it. I decided to look a bit deeper on how a flashing process works before going into the details of how to run Rust code on the board. I'll try to cover that tomorrow

