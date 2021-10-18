# WGPU Cube
Demonstrates how to render an interactive static scene (a cube with an orbit camera) with [wgpu](https://github.com/gfx-rs/wgpu) while using only a small amount of system resources (e.g. render only when the camera orientation or the scene changes).

It is heavily based on the excellent [learn-wgpu](https://github.com/sotrh/learn-wgpu) tutorial. In contrast to the official examples for wgpu, which use a fairly involved [_framework_](https://github.com/gfx-rs/wgpu/blob/v0.10/wgpu/examples/framework.rs) this aims to be completly standalone and will hopefully become as modular as possible later in time.

## Desktop
You should be able to run the project on **Linux**, **MacOS** and **Windows** using `cargo run --release`.

## WebGL2
In order to build for `wasm32-unknown-unknown`:
1. Run `wasm-pack build --target web` ( you need a recent version of [`wasm-pack`](https://rustwasm.github.io/wasm-pack/))
2. Serve the root directory that contains the `Cargo.toml` with a static web server (for example with [`live-server`](https://www.npmjs.com/package/live-server))
3. Open `index.html` in your browser with WebGL2.0 support

The latest build is live on [https://frankenapps.github.io/wgpu_cube/](https://frankenapps.github.io/wgpu_cube/).

## Controls
#### Mouse
* Rotate: Hold left mouse button and drag.
* Zoom: Scroll mouse wheel.
