# rustbar
A wayland status bar written in pure rust. Aimed at tiling window managers.

Code may not be idiomatic rust. This is my first rust project.

## Current state of project
Unless you plan to contribute or look at the code, ignore this repository.
The application is not useful yet.

### Aim
`rustbar` aims to provide a status bar that is highly configurable, performant and pretty: bars like `waybar`, `i3bar`, `polybar` etc seem to focus on more simplistic components: a date is just a date, and changing it should be done through the configuration.

This project aims to go further than that and provide an experience more like plasma's taskbar, or windows' taskbar, where components offer the functionality to edit them in the GUI itself, etc.

## Compile
Assuming you have `git` and `cargo` installed:
```
git clone https://github.com/zeroeightysix/rustbar
cargo build --release
target/build/rustbar
```
(`cargo run` in development)