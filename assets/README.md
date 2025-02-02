# Assets for monoff

## `icon.ico`

![monoff icon](../docs/monoff.png)

This is the icon (or icon group) used for monoff and is bundled inside it when
the project is built (see [build.rs](../build.rs)).

It comes from the official Windows icons. Its largest resolution is 256x256
pixels at a 32-bit color depth, and contains several other smaller sizes.

The process for producing this file is as follows:

1. Locate an `imageres.dll` (or `imageres.dll.mun`) file on a Windows system.
   Several were available on my system, and I just chose the largest one.

2. Using [Greenfish Icon Editor Pro 4.2](https://greenfishsoftware.org/gfie.php),
   open the DLL file.

3. Locate this icon from the list displayed, icon #101. Right-click, "Extract
   and save...", and save the file in this directory as `icon.ico`.
