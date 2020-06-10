# rustbar
A wayland status bar written in pure rust, aimed at tiling window managers.

## Current state of project
Unless you plan to contribute or look at the code, ignore this repository.
The application is not useful yet.

### Aim
`rustbar` aims to provide a status bar that is highly configurable, performant and pretty. Bars like `waybar`, `i3bar`, `polybar` seem to focus on more simplistic components: a date is just a date, and changing it should be done through the configuration.

This project aims to go further than that and provide an experience more like a DE's taskbar, where components offer the functionality to edit them in the GUI itself, etc.

## Compile
Assuming you have `git`, `cargo` and [gtk-layer-shell](https://github.com/wmww/gtk-layer-shell) installed:
```
git clone https://github.com/zeroeightysix/rustbar
cargo build --release
target/build/rustbar
```
(`cargo run` in development)
