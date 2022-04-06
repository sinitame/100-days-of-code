# Day 04 - Using cargo dinghy

Today I didn't have much time to work on the challenge so I just worked on the setup of a tool that is really useful when doing cross-compilation in Rust which is called [`dinghy`](https://github.com/sonos/dinghy). It's a `cargo`extension that ease up cross compilation and code deplayment on "os-friendly" embedded devices.

## Dinghy installation

I choosed the easy way since I didn't have much time. I directly downloaded the binary from the release page and installed it in cargo binaries

```
wget -q https://github.com/sonos/dinghy/releases/download/0.4.71/cargo-dinghy-linux-0.4.71.tgz
tar vzxf cargo-dinghy-macos-0.4.71.tar --strip-components 1
mv cargo-dinghy $HOME/.cargo/bin
```

And that's it ! We are ready to go !



## Cross compile and run on target with dinghy

In order to setup the project for dinghy, I just had to copy my `hello-world` project and to add the following dinghy config file `.dinghy.toml`.

```
[platforms.raspi-ubuntu]
rustc_triple="aarch64-unknown-linux-musl"
toolchain="/Users/sinitame/Downloads/aarch64-unknown-linux-gnu"

[ssh_devices]
raspi = { hostname = "raspi", username="pi", platform="raspi-ubuntu" }
```

In this file we specify the platform specificities:

- `rustc_triple` that is equivalent to the `--target` argument we had previously
- `toolchain` which indicates the path where dinghy will look for the binutils it needs to cross-compile (particularly the linker).

An also the devices we want to use to run the code. The arguments speak for themselves.

**Note:** Here the `hostname` refers to a given `Host` defined in my `.ssh/config` . I could have directly set the IP address but doing so allows me to change this value just in one place when I need to update it.

Now let's compile and run our code with dinghy ! 🚀

```
cargo dinghy -d raspi run
[2022-04-06T06:07:37Z INFO  dinghy_lib::ios] Failed while looking for ios simulators. It this is not expected, you need to make sure `xcrun simctl list --json` works.
[2022-04-06T06:07:37Z INFO  cargo_dinghy] Targeting platform 'raspi-ubuntu' and device 'raspi'
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
[2022-04-06T06:07:37Z INFO  dinghy_lib::ssh::device] Install "hello-world-dinghy"
[2022-04-06T06:07:38Z INFO  dinghy_lib::ssh::device] Install hello-world-dinghy to raspi
[2022-04-06T06:07:40Z INFO  dinghy_lib::ssh::device] Run hello-world-dinghy on raspi (Build)
Hello, world!
```

Great ! It worked 🥳

