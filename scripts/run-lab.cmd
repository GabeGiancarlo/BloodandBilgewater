@echo off
set WGPU_BACKEND=vulkan
echo Starting Blood and Bilgewater Lab...
cargo run --example lab --features lab
