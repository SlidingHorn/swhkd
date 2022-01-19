<p align=center>
  <img src="./assets/swhkd.png" alt=SWHKD width=60%>
  
  <p align=center>A next-generation hotkey daemon for Wayland/X11 written in Rust.</p>
  
  <p align="center">
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-BSD-yellow.svg"></a>
  <img src="https://img.shields.io/badge/cargo-v0.1.0-yellow">
  <img src="https://img.shields.io/badge/open%20issues-7-yellow">
  <img src="https://img.shields.io/badge/build-passing-success">
  </p>
</p>

## SWHKD

swhkd is a display protocol-independent hotkey daemon made in Rust. swhkd uses an easy-to-use configuration system inspired by sxhkd so you can easily add or remove hotkeys.

Because swhkd can be used anywhere, the same swhkd config can be used across Xorg or Wayland desktops, and you can even use swhkd in a tty.

# Dependencies:

## Runtime:

-   Policy Kit Daemon ( polkit )

## Compile time:

-   `rustup`
-   `make`

# Compiling:

-   `git clone https://github.com/shinyzenith/swhkd`
-   `make setup`
-   `make clean`
    -   `make` for a musl compile.
    -   `make glibc` for a glibc compile.
-   `sudo make install`

# Running:

`pkexec swhkd`

# Support server:

https://discord.gg/KKZRDYrRYW

# Contributors:

<a href="https://github.com/Shinyzenith/swhkd/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Shinyzenith/swhkd" />
</a>
