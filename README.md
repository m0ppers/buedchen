# buedchen

A work in progress wayland only compositor that implements a fullscreen shell for kiosk systems. Based on smithay. Mostly a fork of anvil.
Supports `wlr_layer_shell` so a virtual keyboard might be attached for tablet use.

## Dependencies

You'll need to install the following dependencies (note, that those package
names may vary depending on your OS and linux distribution):

- `libwayland`
- `libxkbcommon`

#### These are needed for the "Udev/DRM backend"

- `libudev`
- `libinput`
- `libgbm`
- [`libseat`](https://git.sr.ht/~kennylevinsen/seatd)

If you want to enable X11 support (to run X11 applications within anvil),
then you'll need to install the following packages as well:
    - `xwayland`

## Build and run

You can run it with cargo after having cloned this repository:

```
cargo run -- --{backend}
```

The currently available backends are:

- `--winit`: start anvil as a [Winit](https://github.com/tomaka/winit) application. This allows you to run it
  inside of an other X11 or Wayland session.
- `--tty-udev`: start anvil in a tty with udev support. This is the "traditional" launch of a Wayland
  compositor. Note that this requires you to start anvil as root if your system does not have logind
  available.
