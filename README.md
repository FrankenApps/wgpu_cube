# WGPU Cube
Demonstrates how to render an interactive static scene (a cube with an orbit camera) with [wgpu](https://github.com/gfx-rs/wgpu) while using only a small amount of system resources.

It is heavily based on the excellent [learn-wgpu](https://github.com/sotrh/learn-wgpu) tutorial. In contrast to the official examples for wgpu, which use a fairly involved [_framework_](https://github.com/gfx-rs/wgpu/blob/v0.10/wgpu/examples/framework.rs) this aims to be completly standalone and will hopefully become as modular as possible, later in time.

## WebGL
The goal is to also compile this sample to wasm using WebGL, but this is not done yet.