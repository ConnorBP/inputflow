# Input Flow [![Crates.io Version](https://img.shields.io/crates/v/inputflow)](https://crates.io/crates/inputflow)

User Input Device abstract plugin API. Enables users to access one set of apis for sending mouse and keyboard input, then allowing the actual method of input to be swapped out at runtime. One such method may be [WinAPI](https://learn.microsoft.com/en-us/windows/win32/learnwin32/mouse-movement) for example. Now supports the popular KMBox device as well for controlling user input. Heavily inspired by my favourite memory introspection crate [memflow-rs](https://github.com/memflow/memflow).

# Documentation
[docs.rs](https://docs.rs/inputflow/latest/inputflow/)

# Running Examples
You can run the example plugin loader to test any inputflow plugin like so:
```bash
cargo b -r
cargo r -r --bin inputflow-example-loader
```