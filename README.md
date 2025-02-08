# Walt

Walt is a TUI for setting you wallpaper, it uses `feh` for X11 and
`swww` for Wayland.

By default `walt` looks in `~/Pictures/Wallpapers` with the possiblity
to change the default directory via a config file.

# Usage

By default if you run `walt` you will run the TUI, but you can also you `walt`
as a CLI to set a random wallpaper via `walt --no-tui`.

For example you could add `exec_always walt --no-tui` into your `i3` config
to set a random wallpaper when you start your computer.

# Preview

TODO

# Install

## Binary

TODO

## Cargo

TODO

## Build from source

```bash
git clone git@github.com:R3ZV/walt.git

cd walt

cargo build -r

cp target/release/walt ~/bin
```
