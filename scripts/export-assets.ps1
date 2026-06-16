# Export runtime assets: characters + flora.
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot\..

Write-Host "Exporting character sheets..." -ForegroundColor Cyan
cargo run --bin export_character_sheets --features asset-export

Write-Host "Exporting trees and fruit..." -ForegroundColor Cyan
cargo run --bin export_flora --features asset-export

Write-Host "Done. Run .\scripts\run-lab.ps1 to preview." -ForegroundColor Green
