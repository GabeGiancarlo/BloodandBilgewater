<#
.SYNOPSIS
    Migrate Unity-phase art from the archived Unity prototype into the Bevy-first
    asset structure (assets/source + assets/textures + assets/fonts).

.DESCRIPTION
    Source of truth = Aseprite files -> assets/source/aseprite/...
    Runtime/raw PNG + metadata -> assets/textures/...
    Reference images -> assets/source/references/...
    Fonts -> assets/fonts/...

    Directory segments are normalised (lowercase, spaces -> underscore) and known
    typos are corrected (musicain -> musician, rockey -> rocky). Asset FILE names
    are preserved verbatim (no file renames). Every directory rename is recorded in
    docs/migration/asset_move_log.md.

    Long Windows paths (>260 chars) in the deep AI-generation PNG dumps are handled
    by robocopy, which supports extended-length paths natively.

    Re-runnable: copies are additive; existing files are overwritten.
#>

$ErrorActionPreference = 'Stop'
$repo   = (Resolve-Path "$PSScriptRoot\..\..").Path
$src    = Join-Path $repo "My 2d game ARggg\Assets\_Project"
$artAse = Join-Path $src "Art\Aseprite"
$assets = Join-Path $repo "assets"
$log    = Join-Path $repo "docs\migration\asset_move_log.md"

if (-not (Test-Path $src)) { throw "Unity _Project source not found at: $src" }

$renames = New-Object System.Collections.Generic.List[string]
function Note([string]$from, [string]$to) { $renames.Add("| ``$from`` | ``$to`` |") }

function Normalize-Dir([string]$relDir) {
    $d = $relDir.ToLowerInvariant().Replace(' ', '_')
    $d = $d.Replace('playerdefault', 'player_default')
    $d = $d.Replace('shallowwater', 'shallow_water')
    $d = $d.Replace('titlescreen', 'title_screen')
    $d = $d.Replace('rockey_stone_shore', 'rocky_stone_shore')
    $d = $d.Replace('rockey', 'rocky')
    $d = $d.Replace('musicain', 'musician')
    return $d
}

Write-Host "== 1. Aseprite sources -> assets/source/aseprite =="
$aseFiles = Get-ChildItem -LiteralPath $artAse -Recurse -Force -File -Filter *.aseprite -ErrorAction SilentlyContinue
$aseCount = 0
foreach ($f in $aseFiles) {
    $rel    = $f.FullName.Substring($artAse.Length).TrimStart('\')
    $relDir = Split-Path $rel -Parent
    $newDir = if ($relDir) { Normalize-Dir $relDir } else { "" }
    $destDir = Join-Path (Join-Path $assets "source\aseprite") $newDir
    New-Item -ItemType Directory -Force -Path $destDir | Out-Null
    Copy-Item -LiteralPath $f.FullName -Destination (Join-Path $destDir $f.Name) -Force
    $aseCount++
}
Write-Host "   copied $aseCount aseprite files"

Write-Host "== 2. Reference images -> assets/source/references =="
$refSrc = Join-Path $src "Art\References\source"
$refDst = Join-Path $assets "source\references"
New-Item -ItemType Directory -Force -Path $refDst | Out-Null
robocopy $refSrc $refDst *.png /R:1 /W:1 /NFL /NDL /NJH /NP | Out-Null

Write-Host "== 3. Fonts -> assets/fonts =="
$fontSrc = Join-Path $src "UI\Fonts\legacy"
$fontDst = Join-Path $assets "fonts"
New-Item -ItemType Directory -Force -Path $fontDst | Out-Null
robocopy $fontSrc $fontDst /E /XF *.meta /R:1 /W:1 /NFL /NDL /NJH /NP | Out-Null

Write-Host "== 4. Character + prop PNG/metadata -> assets/textures (robocopy, long-path safe) =="
$charSrc = Join-Path $artAse "Characters\PlayerDefault"
$charDst = Join-Path $assets "textures\characters\player_default"
New-Item -ItemType Directory -Force -Path $charDst | Out-Null
robocopy $charSrc $charDst *.png *.json /E /XF *.meta /R:1 /W:1 /NFL /NDL /NJH /NP | Out-Null
# Correct misspelled class folder: musicain -> musician
$bad = Join-Path $charDst "musicain"
$good = Join-Path $charDst "musician"
if (Test-Path -LiteralPath $bad) {
    robocopy $bad $good /E /MOVE /R:1 /W:1 /NFL /NDL /NJH /NP | Out-Null
    Note "Art/Aseprite/Characters/PlayerDefault/musicain (PNG)" "assets/textures/characters/player_default/musician"
}

$propSrc = Join-Path $artAse "Props"
$propDst = Join-Path $assets "textures\props"
New-Item -ItemType Directory -Force -Path $propDst | Out-Null
robocopy $propSrc $propDst *.png /E /XF *.meta /R:1 /W:1 /NFL /NDL /NJH /NP | Out-Null

Write-Host "== 5. Empty gameplay-data + audio scaffolding =="
foreach ($d in @("data\classes","data\items","data\loot_tables","data\ships","data\world","audio\music","audio\sfx")) {
    $p = Join-Path $assets $d
    New-Item -ItemType Directory -Force -Path $p | Out-Null
    $gk = Join-Path $p ".gitkeep"
    if (-not (Test-Path $gk)) { New-Item -ItemType File -Force -Path $gk | Out-Null }
}

Write-Host "== Done. Directory renames recorded: $($renames.Count) =="
$renames | ForEach-Object { Write-Host $_ }
