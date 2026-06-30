# Run Island Gen lab via the main game binary (works when lab.exe is blocked by Windows).
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

$env:WGPU_BACKEND = "vulkan"
$env:LAB_WORLD = "island_gen"

Write-Host "Island Gen Lab (via bloodandbilgewater.exe)" -ForegroundColor Cyan
Write-Host "Esc = menus | 1-7 stages | N seed | R regen | WASD pan | wheel zoom" -ForegroundColor DarkGray

cargo run --bin bloodandbilgewater
