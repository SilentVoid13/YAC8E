# YAC8E - Yet Another CHIP-8 Emulator

[YAC8E](https://github.com/SilentVoid13/YAC8E) is Yet Another CHIP-8 Emulator written in Rust. 

This emulator implements the original CHIP-8 specifications and its 35 instructions (no SUPER-CHIP).

I tried to make the code as clean as possible, and added some documentation. This project aims to be a good reference for people wanting to implement their own CHIP-8 emulator in Rust. 

I invite anyone who wants to start working with emulators to start by creating a CHIP-8 emulator. The creation process is really fun, progressive and not boring at any moment.

## Usage

Because i felt like so, i decided to use 2 different libraries in order to handle the display, the keyboard and the sound :

- [SDL](https://en.wikipedia.org/wiki/Simple_DirectMedia_Layer)
- [minifb](https://github.com/emoon/rust_minifb) (doesn't support sound unfortunately)

You can pick the library of your choice when starting the emulator.

You can also set a custom `Hertz` value for the CPU clock cycle per second speed. The best `Hertz` value may vary with games and may require some tuning. `500` is considered a good value in average.

```bash
USAGE:
    yac8e [FLAGS] [OPTIONS] <ROM_FILE>

FLAGS:
    -d, --debug      Sets debugging output (default: false)
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --height <HEIGHT>           Sets the window height (default: 320)
    -H, --hertz <HERTZ>             Sets the Hertz value for the CPU clock cycle per second speed (default: 500 Hz)
    -l, --library <sdl> <minifb>    Sets the handling library to use (default: sdl) (minifb doesn't support sounds)
        --width <WIDTH>             Sets the window width (default: 640)

ARGS:
    <ROM_FILE>    The ROM file to run
```

## Building

In order to build this project, you will need to have `sdl2` installed.

For Debian-based distributions :

```bash
sudo apt-get install libsdl2-dev libsdl2-gfx-dev
```

For Arch-based distributions :

```bash
sudo pacman -S sdl2 sdl2_gfx
```

To build this project, you will need to use `cargo`:

```bash
cargo build --release
```

Your binaries will be available in the `target/release` directory.

This emulator has only been tested on Linux 64-bit, but it should work on most platforms.

## Resources

These are some good resources i found for the creation of this emulator :

- https://en.wikipedia.org/wiki/CHIP-8
- http://mattmik.com/files/chip8/mastering/chip8.html
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

I also peeked at a few already existing projects to help me on a few things (project structure, ...), check them out too :

- https://github.com/starrhorne/chip8-rust
- https://github.com/AlexEne/rust-chip8

## Known bugs

Feel free to report any bugs, so i can fix them.

I noticed the sound sometimes gets skipped on games like [PONG](https://en.wikipedia.org/wiki/Pong). I didn't find where the issue was coming from, and i noticed this bug and a few other emulators as well. The bug may come from the ROM itself.

## Contributing

Feel free to contribute. You can make a [pull request](https://github.com/SilentVoid13/YAC8E/pulls) to suggest any change you'd like to make (for example a code optimization, ...).

## License

[YAC8E](https://github.com/SilentVoid13/YAC8E) is licensed under the GNU AGPLv3 license. Refer to [LICENSE](https://github.com/SilentVoid13/YAC8E/blob/master/LICENSE.txt) for more informations.