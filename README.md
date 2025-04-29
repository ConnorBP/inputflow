# Input Flow [![Crates.io Version](https://img.shields.io/crates/v/inputflow)](https://crates.io/crates/inputflow)

User Input Device abstract plugin API. Enables users to access one set of apis for sending mouse and keyboard input, then allowing the actual method of input to be swapped out at runtime. One such method may be [WinAPI](https://learn.microsoft.com/en-us/windows/win32/learnwin32/mouse-movement) for example. Now supports the popular KMBox device as well for controlling user input. Heavily inspired by my favourite memory introspection crate [memflow-rs](https://github.com/memflow/memflow).

# Connectors
Currently inputflow supports three methods of input control: `native`, `kmbox`, and `qmp` (QEMU).
## Native
Controls the mouse and keyboard of your local computer. [... read more](./inputflow-native/README.md)

## KMBOX
Sends commands over serial to a KMBOX or arduino device to control the mouse and keyboard of an external computer.[... read more](./inputflow-kmbox/README.md)

## QMP
Utilizes the QMP Protocol of QEMU to control the mouse and keyboard of a virtual machine. [... read more](./inputflow-qmp/README.md)


# Documentation
- [Docs.rs](https://docs.rs/inputflow/latest/inputflow/)
- [DeepWiki](https://deepwiki.com/ConnorBP/inputflow)
- [Loader code example](./inputflow-example-loader/src/main.rs)
- [Connector code example](./inputflow-native/src/lib.rs)

# Running Examples
You can run the example plugin loader to test any inputflow plugin like so:
```bash
cargo b -r
cargo r -r --bin inputflow-example-loader
```