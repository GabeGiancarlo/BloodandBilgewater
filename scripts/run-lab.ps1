# Run the Starter Island Animation Lab (not the asset exporter).
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

# Vulkan avoids DXGI black-screen issues on some Windows GPUs.
$env:WGPU_BACKEND = "vulkan"

Write-Host "Starting Blood and Bilgewater Lab (Starter Island)..." -ForegroundColor Cyan
Write-Host "Controls: WASD pan | Space follow | R reset | H help | 1-5 reload" -ForegroundColor DarkGray
cargo run --example lab --features lab
