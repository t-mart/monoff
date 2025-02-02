# <img src="./docs/monoff.png" width="64px" alt="monoff logo of a computer screen displaying crescent moon"> monoff - Turn off your monitors on Windows

![GitHub download count](https://img.shields.io/github/downloads/t-mart/monoff/total)
![Latest GitHub release](https://img.shields.io/github/v/release/t-mart/monoff)
![Crates.io](https://img.shields.io/crates/v/monoff)

monoff is a Windows program that turns off/sleeps your monitors – that's it.

- **Simple**: There's no magic to it. It works the same way as if you'd left
  your computer inactive for however many minutes. Run it, and the monitors turn
  off. Then, move your mouse, they'll turn back on again. No GUI, no
  administrator privileges, no background processes.
- **Portable**: Everything about it is self-contained in the .exe and does not
  require an installer. You can even run it from a USB stick.
- **Convenient**: Just click an icon in your taskbar, or press a keyboard
  shortcut. I made this program because the power button on my monitors is hard
  to reach.
- **Safe**: This program is open-source. You can inspect the code and build it
  yourself.

You can also specify a delay before the monitors turn off, so that you can
safely remove your hand from your mouse, for example. It defaults to 100
milliseconds.

## Installation

### From GitHub (easiest)

Download `monoff.exe` from the
[latest release](https://github.com/t-mart/monoff/releases/latest) and put it
somewhere, such as on your Desktop.

Running it is then as simple as double-clicking it.

### Scoop (recommended)

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
cargo install monoff
```

This will build the executable and place it at
`%UserProfile%\.cargo\bin\monoff.exe` (and, most likely, put it in your `PATH`
variable).

#### Binstall

You can also use [binstall](https://github.com/cargo-bins/cargo-binstall) to
install the latest GitHub release without needing to build. Run:

```shell
cargo binstall monoff
```

## Making it a taskbar shortcut

As normal, just drag the application to your taskbar:

![add to taskbar](https://raw.githubusercontent.com/t-mart/monoff/master/docs/add-to-taskbar.gif)

From here, you can **just click to icon** the turn off your monitors.

### Adding a hotkey/keyboard shortcut

<details>
  <summary>Expand</summary>

To run the application when you press a hotkey/keyboard shortcut, edit the
`Shortcut Key` value in its properties:

![edit shortcut arguments](https://raw.githubusercontent.com/t-mart/monoff/master/docs/hotkey.gif)

Note that Windows will automatically prefix your choice with `Ctrl + Alt`. You
only can only provide the last key.

</details>

### Adding a delay

<details>
  <summary>Expand</summary>

The default delay (50ms) is fine in most cases. But, if you want to change the
delay to this shortcut, edit the `Target` value in its properties to add the
delay option:

![edit shortcut arguments](https://raw.githubusercontent.com/t-mart/monoff/master/docs/shortcut-arguments.gif)

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

This program is heavily inspired by
[lcdoff-rs](https://github.com/Gekkio/lcdoff-rs) (and its partner
[article](https://gekkio.fi/blog/2014/calling-win32-api-with-rust-ffi/)). Thank
you, @Gekkio!

## Other Projects Named monoff

After creating this `monoff` project, I found that there are several others with
the same name. To keep things clear, these are the only pages directly connected
to this project:

- [monoff on GitHub](https://github.com/t-mart/monoff)
- [monoff on Crates.io](https://crates.io/crates/monoff)

## DevOps

To:

- create and push a new git tag,
- create a new GitHub release with binaries attached, and
- publish a new version to <https://crates.io> (and update docs on
  <https://docs.rs>)

simply push a commit to the `master` branch with an updated version number in
`Cargo.toml`. The workflow file at `.github/workflows/release-build-publish.yml`
will take care of the rest. Make sure to pull afterwards.
