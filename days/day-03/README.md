# Day 03 - Cross compile Rust for Raspberry Pi (follow up)

Yesterday, I have managed to cross compile a simple hello world Rust projet for my Raspberry Pi but I had a bug when I tried to do it for `aarch64` instead of `armv7` instruction set. I've tried to find the root cause but it doesn't seem so obvious 😕

Today I wanted to try again but this time using musl in order to generate a statically linked binary and see if this time it runs on my Raspberry Pi.

## Generating an `aarch64` binary with musl

In order to target `aarch64` using musl I have added the proper target to `rustup`

```
rustup target add aarch64-unknown-linux-musl
```

Then I updated my `.cargo/config` to use the linker from the `aarch64`  toolchain that I downloaded yesterday

```
[target.aarch64-unknown-linux-musl]
linker = "/Users/sinitame/Downloads/aarch64-unknown-linux-gnu/bin/aarch64-unknown-linux-gnu-ld"
```

Finaly I build my new binary

```
cargo build --target aarch64-unknown-linux-musl --release
```

And when I've run it on the Raspberry Pi, it worked !!! 🥳

```
pi@raspi3-home:/tmp $ file hello-world
hello-world: ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked, with debug_info, not stripped
pi@raspi3-home:/tmp $ ./hello-world
Hello, world!
```

