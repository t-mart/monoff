# ![monoff](docs/monoff.png) monoff - Turn off your monitors on Windows

monoff is a Windows program that turns off/sleeps your monitors -- that's it. When you
move your mouse or press a key on the keyboard, they'll turn back on again.

There's no magic to it. It works the same way as if you'd left your computer
inactive for however many minutes.

You can also specify a delay before the monitors turn off, so that you can
safely remove your hand from your mouse, for example.

I wrote this program because I purchased some monitors whose power switches
were hard to reach. Now, I can just click an icon in my taskbar.

## Installation

### From GitHub

Download `monoff.exe` from the [latest
release](https://github.com/t-mart/monoff/releases/latest) and put it somewhere,
such as on your Desktop.

### Scoop

Run:

```shell
scoop bucket add t-mart https://github.com/t-mart/t-mart-scoop-bucket
scoop install t-mart/monoff
```

This will place the executable at
`%UserProfile%\scoop\apps\monoff\current\monoff.exe`. It will also create a
Start Menu item.

### Cargo

Run:

```shell
cargo install --git https://github.com/t-mart/monoff
```

This will place the executable at `%UserProfile%\.cargo\bin\monoff.exe`.

## Making it a taskbar icon

As normal, just drag the application to your taskbar. This will make a shortcut:

![add to taskbar](/docs/add-to-taskbar.gif)

From here, you can just click to icon the turn off your monitors.

If you want to add a delay to this shortcut, edit the `Target` value in its properties:

![edit shortcut arguments](/docs/shortcut-arguments.gif)

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
