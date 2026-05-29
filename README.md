# RSZoomer

A simple screen magnifier for Windows inspired by Tsoding's Boomer tool. 

It takes a screenshot instantly and lets you zoom in on pixels.

## Controls
* `Ctrl + F1` — Activate / Freeze screen
* `Mouse Wheel` — Zoom in / out
* `Hold LMB` — Drag / Pan around
* `Press RMB` — Toggle flashlight
* `Left Ctrl + Mouse Wheel` — Adjust flashlight radius
* `ESC` — Close

## Quick Start

You need to have **Rust** and a **C compiler** (like Visual Studio Build Tools) installed on your system. This is required to compile `raylib`.

```powershell
cargo build --release
```

## Config

You can customize the settings using the configuration file. It will be created automatically on the first launch.

File location: `C:\Users\<Your-Username>\.config\RSZoomer\config.toml`

### Parameters
* `fps` — Sets the maximum framerate for the zoom window.
* `zoom_speed` — Controls how smoothly/fast the camera zooms in and out.
* `flashlight_speed` — Controls how smoothly the flashlight radius adjusts.
* `default_flashlight_radius` — Sets the default flashlight radius.

### Example
```toml
[zoomer]
fps = 240
zoom_speed = 0.1
flashlight_speed = 0.05
default_flashlight_radius = 120.0
```
