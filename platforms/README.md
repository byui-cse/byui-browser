# Platform shells

Thin native shells that host the browser UI. The engine produces pixels and events; shells stay small.

| Directory | Platform | Owning team   |
|-----------|----------|---------------|
| `macos/`  | AppKit   | Browser UX    |
| `windows/`| Win32 / WinUI | Browser UX |
| `linux/`  | GTK      | Browser UX    |

Create a shell subdirectory when starting platform work. Do not put engine logic here.
