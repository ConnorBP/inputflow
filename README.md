# Input Flow
User Input Device abstract plugin API. Enables users to access one set of apis for sending mouse and keyboard input, then allowing the actual method of input to be swapped out at runtime. One such method may be [WinAPI](https://learn.microsoft.com/en-us/windows/win32/learnwin32/mouse-movement) for example. Heavily inspired by my favourite memory introspection crate [memflow-rs](https://github.com/memflow/memflow).

## WORK IN PROGRESS
This is a brand new project, I would not recommend you use it just yet,