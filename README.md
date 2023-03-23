# rp2040-display

My first ever Rust embedded project. Nothing serious, just a fun project. 🤠

## Hardware I'm using
- [Raspberry Pi Pico H](https://www.raspberrypi.com/products/raspberry-pi-pico/)
- [Raspberry Pi Debug Probe](https://www.raspberrypi.com/products/debug-probe/)
- [Pico Display Pack by Pimoroni](https://shop.pimoroni.com/en-us/products/pico-display-pack)

## What it does
My plan is to have multiple screens with basic info, like the current time, the current temperature, & maybe some basic interactive screens that you can control through the Pimoroni Display buttons.

## Running
First you'll need these development dependencies:
```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link probe-run
```

Once you have the Pico with the Debug Probe connected to your computer, run:
```sh
cargo run

# Or for release mode...
cargo run --release
```
