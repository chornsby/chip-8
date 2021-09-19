# Chip 8

![Test](https://github.com/chornsby/chip-8/workflows/Test/badge.svg)

This is a working Chip-8 emulator built to learn more about emulation and Rust
without following an explicit step-by-step tutorial.

## Getting started

You will need the [SDL2 library][0] installed in order to compile the project.

For example, on Fedora run the following command to install the development
library:

```bash
sudo dnf install SDL2-devel
```

Then run the project passing a path to the Chip-8 rom as follows:

```bash
cargo run --release -- <path_to_rom>
```

If you do not have a Chip-8 rom you can download an archive from [here][1].

## Acknowledgements

I implemented the emulator without referring to other similar projects directly,
so it probably has a few questionable design decisions. I worked from [Cowgod's
Technical Reference][2] which describes how the interpreter should work and
lists the CPU instruction set. I also used the [Wikipedia article][3] as another
reference to fill in the gaps. When the project was complete I checked my
approach in comparison to the one described in [this blog][4].

I got started using SDL2 thanks to helpful blog posts like [this one][5] about
opening a window.

[0]: https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries
[1]: https://www.zophar.net/pdroms/chip8.html
[2]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
[3]: https://en.wikipedia.org/wiki/CHIP-8
[4]: https://wjdevschool.com/blog/video-game-console-emulator/
[5]: https://sunjay.dev/learn-game-dev/opening-a-window.html
