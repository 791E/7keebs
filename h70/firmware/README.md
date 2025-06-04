# Firmware for the h70

## About
This is the firmware for the 7keebs h70 keyboard. It defines a dvorak based layout
and sends keystrokes via HID reports over USB to a computer.
Additionally, the keyboard features up to four custom modifier keys,
which can be programmed to do anything from within this program.
For now I haven't found any use for them, but you can adapt them to your own needs.

## Compatibility
The program should work on ESP32 chips (**Only** ESP32, for the program to work with the ESP32C3
for example, the target would require some slight modifications.)
I'm using an ESP-WROOM-32, but in theory it should work on any ESP32 chip.

## Building
1. Make sure you have cargo installed. If you dont have it, install it via rustup
(see <https://www.rust-lang.org/tools/install>.)
1. Install the esp toolchain using `espup`. If you don't have it, install it like this:
```sh
cargo install espup
espup install
```
1. Install `espflash` to flash the program to the chip
```sh
cargo install espflash
```
1. Flash the program with `cargo run --release`.

## Credits
I took great inspiration from [Brian Schwind](https://github.com/bschwind/key-ripper)'s
[talk at the Tokyo Rust Meetup](https://youtu.be/x7LQevYn7d0?si=_7Q5qb9GhmzrPRaL).

