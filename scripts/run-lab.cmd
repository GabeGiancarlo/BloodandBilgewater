@echo off
set WGPU_BACKEND=vulkan
if not defined LAB_WORLD set LAB_WORLD=island_gen
echo Blood and Bilgewater Lab via bloodandbilgewater.exe (LAB_WORLD=%LAB_WORLD%)...
cargo run --bin bloodandbilgewater
