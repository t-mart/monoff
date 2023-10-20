# ![monoff](docs/monoff.png) monoff - Turn off your monitors on Windows

![GitHub all releases](https://img.shields.io/github/downloads/t-mart/monoff/total)
![GitHub release (with filter)](https://img.shields.io/github/v/release/t-mart/monoff)

monoff is a Windows program that turns off/sleeps your monitors – that's it. When you
move your mouse or press a key on the keyboard, they'll turn back on again.

- **Simple**: There's no magic to it. It works the same way as if you'd left your computer
inactive for however many minutes. Click it, and the monitors turn off.
- **Portable**: Everything about it is self-contained in the .exe and does not
require an installer. You can even run it from a USB stick.
- **Convenient**: Just click an icon in your taskbar, or press a keyboard shortcut. I made this program because the power button on my monitors is hard to reach.
- **Safe**: This program is open-source, and you can inspect the code and build it yourself.

You can also specify a delay before the monitors turn off, so that you can
safely remove your hand from your mouse, for example. It defaults to 50 milliseconds.

## Installation

### From GitHub

Download `monoff.exe` from the [latest
release](https://github.com/t-mart/monoff/releases/latest) and put it somewhere,
such as on your Desktop.

Running it is then as simple as double-clicking it.

### Scoop

Run:

```shell
scoop bucket add t-mart https://github.com/t-mart/bucket
scoop install t-mart/monoff
```

This will:

- put `monoff` in your `PATH` variable
- place the executable at `%UserProfile%\scoop\apps\monoff\current\monoff.exe`
- create a Start Menu item

### Cargo

Run:

```shell
cargo install --git https://github.com/t-mart/monoff
```

This will place the executable at `%UserProfile%\.cargo\bin\monoff.exe` (and, most likely, put it in your `PATH` variable).

## Making it a taskbar shortcut

As normal, just drag the application to your taskbar:

![add to taskbar](/docs/add-to-taskbar.gif)

From here, you can **just click to icon** the turn off your monitors.

### Adding a hotkey/keyboard shortcut

<details>
  <summary>Expand</summary>

  To run the application when you press a hotkey/keyboard shortcut, edit the `Shortcut Key` value in its properties:

  ![edit shortcut arguments](/docs/hotkey.gif)

  Note that Windows will automatically prefix your choice with `Ctrl + Alt`. You only can only provide the last key.
</details>

### Adding a delay

<details>
  <summary>Expand</summary>

  The default delay (50ms) is fine in most cases. But, if you want to change the
  delay to this shortcut, edit the `Target` value in its properties to add the delay option:

  ![edit shortcut arguments](/docs/shortcut-arguments.gif)
</details>

## Usage examples

- Turn off your monitors immediately:

  ```shell
  monoff.exe
  ```

- Turn them off after a delay of 2 seconds:

  ```shell
  monoff.exe --delay 2000
  ```

- See help:

  ```shell
  monoff.exe --help
  ```

## Thanks

This program is heavily inspired by [lcdoff-rs](https://github.com/Gekkio/lcdoff-rs) (and its partner [article](https://gekkio.fi/blog/2014/calling-win32-api-with-rust-ffi/)), but with a few differences:

- uses the [`windows`](https://crates.io/crates/windows) crate, which is a
  little easier/idiomatic to work with
- accepts a delay parameter, the amount of time to wait before powering the monitors off
- has a representative icon
