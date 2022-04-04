# Day 02 - Cross compile Rust for Raspberry Pi

Today I'm going to cross compile a simple `hello-world` Rust project from MacOS to my Raspberry Pi.

Cross compiling in Rust is quite simple when you know what you are doing but I realised it's not that easy when you just follow random tutorial on the web without really understanding what you do. That's why I'll sum-up here what I understood 📝.

We will go into the details after but we basically need three things:

- A compiler to generate ARM assembly from our rust code.
- A linker to link the generated assembly code with system libraries (dynamically -> the dependencies will just be resolved during linking, so dynamic libs must be present at linking but won't be included in the final binary, statically -> dependencies will be included in the final binary).
- A `sysroot` that contains the target system files hierarchy with the target libraries, includes and so on (needed by the linker to resolve the dependencies ).

### First let's compile

Compiling with Rust is easy as it only require to add the proper rustup compiler. Here we are targeting a Raspberry Pi 3B which as mentionned in my previous notes has a Cortex A53 processor. This processor implements the ARMv8 instruction set which is more broadly speaking an `aarch64` instruction set. In order to add this target to the rust compiler, we just have to run the following command:

````
rustup target add aarch64-unknown-linux-gnu
````

By doing that I was able to compile rust but of course without linker I have an error at linking 🙂

```
cargo build --target aarch64-unknown-linux-gnu

# The error I got
error: linking with `cc` failed: exit status: 1
```



### A linker

So as mentionned earlier, with a proper compiler we can compile our source code to the target architecture but then, we have to link our generated binaries and to do that we need a [linker](https://en.wikipedia.org/wiki/Linker_(computing)) ! Rust doesn't use its own linker so we need to use the one from the host. But the default one on MacOS doesn't understand ARM code so it won't work. 

That's why we need to have a linker for ARM target **BUT** compiled for MacOS. Usually, what we would need to do is to build a toolchain for that given target but thankfully some poeple already done that and shared the appropriate toolchain of the web. I took[ this one](https://thinkski.github.io/osx-arm-linux-toolchains/).

Now, all we need to do is to indicate to cargo that we want to use this specific linker to link our binary files for our target. We can do that by creating a `.cargo/config` file with the following:

```
[target.aarch64-unknown-linux-gnu]
linker = "/Users/sinitame/Downloads/aarch64-unknown-linux-gnu/bin/aarch64-unknown-linux-gnu-ld"
```

Now when I run `cargo build --target aarch64-unknown-linux-gnu --release`

And I get the following 🥳

```
   Compiling hello-world v0.1.0 (/Users/sinitame/Documents/dev/100-days-of-code/days/day-02/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.45s
```

At this step I thought it had worked but when I tried to execute the generated binary located in `target/aarch64-unknown-linux-gnu/release/hello-world` on the Pi, it didn't produced any output 😭



### What happened ? 💀

I've tried to investigate a bit by doing the following.

**First I checked that the binary is really a aarch64 ELF file**

```
pi@raspi3-home:/tmp $ file hello-world
hello-world: ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, with debug_info, not stripped
```

Looks good 🤔

**Then I checked it was properly linked**

Not sure if there is a proper way to do that but I found that by using `ldd` command, I could see the reference to the dynamic libraries that were required by a given binary.

In `man` words:

> ```
> ldd prints the shared objects (shared libraries) required by each program or shared object specified on the command line.
> ```

So I tried that and I had the following output:

```
pi@raspi3-home:/tmp $ ldd hello-world
	linux-vdso.so.1 (0x0000007f8b369000)
	libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x0000007f8b2f2000)
	libpthread.so.0 => /lib/aarch64-linux-gnu/libpthread.so.0 (0x0000007f8b2c3000)
	libdl.so.2 => /lib/aarch64-linux-gnu/libdl.so.2 (0x0000007f8b2af000)
	libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x0000007f8b13d000)
	/lib/ld-linux-aarch64.so.1 (0x0000007f8b33b000)
```



Just to be sure it was great, I also build the same code example directly on the raspberry-pi and I did the same check on the generated binary. And guess what ? I got exacly the same thing:

```
pi@raspi3-home:~/hello-world $ ldd target/debug/hello-world
	linux-vdso.so.1 (0x0000007fa457b000)
	libgcc_s.so.1 => /lib/aarch64-linux-gnu/libgcc_s.so.1 (0x0000007fa44c5000)
	libpthread.so.0 => /lib/aarch64-linux-gnu/libpthread.so.0 (0x0000007fa4496000)
	libdl.so.2 => /lib/aarch64-linux-gnu/libdl.so.2 (0x0000007fa4482000)
	libc.so.6 => /lib/aarch64-linux-gnu/libc.so.6 (0x0000007fa4310000)
	/lib/ld-linux-aarch64.so.1 (0x0000007fa454d000)
```



**The last thing I've tried is to really follow one of the tutorial that I saw 🤓**

I had decided to compile for `aarch64` but for some reason, most of the tutorials I've found are cross compiling for `armv7` so I tried to do the same.

To do so I replace my `.cargo/config` with:

```
[target.armv7-unknown-linux-musleabihf]
linker = "arm-linux-gnueabihf-ld"
```

I installed a nice package from Homebrew containing all the binutils (mainly the linker) we need to cross compile

```
brew install arm-linux-gnueabihf-binutils
```

And a new rust target

```
rustup target add armv7-unknown-linux-musleabihf
```

Here the rust target is called `musleabihf` the `musl` part indicated that we use the `musl` libc instead of the `gnu` one. You can find out more about this [here](https://amritrathie.vercel.app/posts/2020/03/06/cross-compiling-rust-from-macos-to-raspberry-pi/). The advantage of `musl` is that all the dynamic dependencies are statically linked and so they are included in the final binary.

Then I build the new binary with:

```
cargo build --target armv7-unknown-linux-musleabihf --release
```

I tested it on the raspi and it worked 🚀

That will be all for today, tomorrow I'll try to figure out why my first approach didn't worked 😬