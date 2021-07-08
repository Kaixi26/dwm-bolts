# dwm-blocks

## About
A simple rust program to manage the bar for dwm, can output the lines to stdout or
write directly to the bar with `xsetroot`.

## Config
Configuration is all made in the code, the application needs recompiling after changes.

In `src/config.rs` you can select the modules you want to run and how often they should be ran.

Modules are just functions that return a string, they are stored in `src/config/modules.rs` so it's
very easy to add or modify them.

## Installing
To install simply run
```sh
cargo install --path .
```

Make sure you have `$HOME/.cargo/bin` in your `$PATH` variable.

You can simply run.
```sh
~/.cargo/bin/dwm-bolts --dwm
```

## Future
⋅⋅⋅ ~~Make it async.~~ (DONE)

⋅⋅⋅ Reduce friction of changing used modules (maybe macros?)

⋅⋅⋅ Make it able to respond to signals
