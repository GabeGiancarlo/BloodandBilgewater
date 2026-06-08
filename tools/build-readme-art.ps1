#requires -Version 5.1
<#
.SYNOPSIS
    Build README preview art from existing, animated game frames:
      1. Ship (sails up) -- 8-direction rotation sprite-sheet strip + looping spin GIF.
      2. Shipwright (example animated class) -- 8-direction strip + looping idle GIF.

    Frames are composited onto a solid dark backdrop so output has no GIF
    transparency artifacts and matches the game's gothic tone. Source art is
    never modified. Pure .NET (System.Drawing + a small GIF89a assembler), so it
    runs on stock Windows PowerShell without ImageMagick/ffmpeg/Python.
#>

[CmdletBinding()]
param(
    [int]$SpinDelayCs = 12,   # ship spin per-frame delay (1/100 s)
    [int]$IdleDelayCs = 10    # shipwright idle per-frame delay (1/100 s)
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir
Set-Location $RepoRoot

$OutDir = 'docs/art/preview'
New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
$BackColor = [System.Drawing.Color]::FromArgb(255, 11, 14, 26)  # #0B0E1A gothic near-black blue

# ---------------------------------------------------------------------------
# Frame helpers
# ---------------------------------------------------------------------------
function New-Composite {
    # Center $src on a $canvasW x $canvasH bitmap filled with the backdrop.
    param([System.Drawing.Image]$Src, [int]$CanvasW, [int]$CanvasH)
    $bmp = New-Object System.Drawing.Bitmap($CanvasW, $CanvasH)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.Clear($BackColor)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::Half
    $x = [int](($CanvasW - $Src.Width) / 2)
    $y = [int](($CanvasH - $Src.Height) / 2)
    $g.DrawImage($Src, $x, $y, $Src.Width, $Src.Height)
    $g.Dispose()
    return $bmp
}

function Get-FramesFromFiles {
    param([string[]]$Paths)
    $imgs = @()
    foreach ($p in $Paths) {
        if (-not (Test-Path $p)) { throw "Missing frame: $p" }
        $imgs += [System.Drawing.Image]::FromFile((Resolve-Path $p).Path)
    }
    $cw = ($imgs | Measure-Object Width  -Maximum).Maximum
    $ch = ($imgs | Measure-Object Height -Maximum).Maximum
    $frames = @()
    foreach ($img in $imgs) { $frames += (New-Composite -Src $img -CanvasW $cw -CanvasH $ch); $img.Dispose() }
    return ,$frames
}

function Get-FramesFromSheet {
    # Slice a horizontal sprite sheet into $Count cells of $FrameW x $FrameH.
    param([string]$SheetPath, [int]$FrameW, [int]$FrameH, [int]$Count)
    if (-not (Test-Path $SheetPath)) { throw "Missing sheet: $SheetPath" }
    $sheet = [System.Drawing.Image]::FromFile((Resolve-Path $SheetPath).Path)
    $frames = @()
    for ($i = 0; $i -lt $Count; $i++) {
        $cell = New-Object System.Drawing.Bitmap($FrameW, $FrameH)
        $g = [System.Drawing.Graphics]::FromImage($cell)
        $g.Clear($BackColor)
        $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
        $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::Half
        $destRect = New-Object System.Drawing.Rectangle(0, 0, $FrameW, $FrameH)
        $g.DrawImage($sheet, $destRect, ($i * $FrameW), 0, $FrameW, $FrameH, [System.Drawing.GraphicsUnit]::Pixel)
        $g.Dispose()
        $frames += $cell
    }
    $sheet.Dispose()
    return ,$frames
}

function Save-Strip {
    param([System.Drawing.Bitmap[]]$Frames, [string]$OutPath)
    $w = $Frames[0].Width; $h = $Frames[0].Height
    $strip = New-Object System.Drawing.Bitmap(($w * $Frames.Count), $h)
    $g = [System.Drawing.Graphics]::FromImage($strip)
    $g.Clear($BackColor)
    for ($i = 0; $i -lt $Frames.Count; $i++) { $g.DrawImage($Frames[$i], ($i * $w), 0, $w, $h) }
    $g.Dispose()
    $strip.Save((Join-Path $RepoRoot $OutPath), [System.Drawing.Imaging.ImageFormat]::Png)
    $strip.Dispose()
    Write-Host "Wrote $OutPath"
}

# ---------------------------------------------------------------------------
# GIF assembler
# ---------------------------------------------------------------------------
function Get-FrameGifParts {
    param([System.Drawing.Bitmap]$Bitmap)
    $ms = New-Object System.IO.MemoryStream
    $Bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Gif)
    $b = $ms.ToArray(); $ms.Dispose()

    $packed = $b[10]
    $gctFlag = ($packed -band 0x80) -ne 0
    $gctSizeBits = $packed -band 0x07
    $gctEntries = [int][Math]::Pow(2, $gctSizeBits + 1)
    $gctLen = if ($gctFlag) { $gctEntries * 3 } else { 0 }
    $globalTable = New-Object byte[] $gctLen
    if ($gctLen -gt 0) { [Array]::Copy($b, 13, $globalTable, 0, $gctLen) }
    $pos = 13 + $gctLen

    $colorTable = $globalTable; $sizeBits = $gctSizeBits; $lzwMin = 0; $imageData = $null
    while ($pos -lt $b.Length) {
        $marker = $b[$pos]
        if ($marker -eq 0x21) {
            $pos += 2
            while ($b[$pos] -ne 0) { $pos += ($b[$pos] + 1) }
            $pos += 1
        }
        elseif ($marker -eq 0x2C) {
            $idPacked = $b[$pos + 9]
            $lctFlag = ($idPacked -band 0x80) -ne 0
            $lctSizeBits = $idPacked -band 0x07
            $p2 = $pos + 10
            if ($lctFlag) {
                $lctEntries = [int][Math]::Pow(2, $lctSizeBits + 1)
                $lctLen = $lctEntries * 3
                $localTable = New-Object byte[] $lctLen
                [Array]::Copy($b, $p2, $localTable, 0, $lctLen)
                $colorTable = $localTable; $sizeBits = $lctSizeBits; $p2 += $lctLen
            }
            $lzwMin = $b[$p2]; $p2 += 1
            $start = $p2
            while ($b[$p2] -ne 0) { $p2 += ($b[$p2] + 1) }
            $p2 += 1
            $len = $p2 - $start
            $imageData = New-Object byte[] $len
            [Array]::Copy($b, $start, $imageData, 0, $len)
            break
        }
        else { break }
    }
    return [pscustomobject]@{ ColorTable = $colorTable; SizeBits = $sizeBits; LzwMin = $lzwMin; ImageData = $imageData }
}

function Add-UInt16LE { param([System.Collections.Generic.List[byte]]$List, [int]$Value)
    $List.Add([byte]($Value -band 0xFF)); $List.Add([byte](($Value -shr 8) -band 0xFF))
}

function Save-Gif {
    param([System.Drawing.Bitmap[]]$Frames, [string]$OutPath, [int]$DelayCs)
    $w = $Frames[0].Width; $h = $Frames[0].Height
    $out = New-Object 'System.Collections.Generic.List[byte]'
    [void]($out.AddRange([byte[]][System.Text.Encoding]::ASCII.GetBytes('GIF89a')))
    Add-UInt16LE $out $w; Add-UInt16LE $out $h
    $out.Add([byte]0x70); $out.Add([byte]0x00); $out.Add([byte]0x00)
    [void]($out.AddRange([byte[]](0x21,0xFF,0x0B)))
    [void]($out.AddRange([byte[]][System.Text.Encoding]::ASCII.GetBytes('NETSCAPE2.0')))
    [void]($out.AddRange([byte[]](0x03,0x01,0x00,0x00,0x00)))
    foreach ($bmp in $Frames) {
        $parts = Get-FrameGifParts -Bitmap $bmp
        [void]($out.AddRange([byte[]](0x21,0xF9,0x04,0x04)))
        Add-UInt16LE $out $DelayCs
        $out.Add([byte]0x00); $out.Add([byte]0x00)
        $out.Add([byte]0x2C)
        Add-UInt16LE $out 0; Add-UInt16LE $out 0
        Add-UInt16LE $out $w; Add-UInt16LE $out $h
        $out.Add([byte](0x80 -bor ($parts.SizeBits -band 0x07)))
        [void]($out.AddRange($parts.ColorTable))
        $out.Add([byte]$parts.LzwMin)
        [void]($out.AddRange($parts.ImageData))
    }
    $out.Add([byte]0x3B)
    [System.IO.File]::WriteAllBytes((Join-Path $RepoRoot $OutPath), $out.ToArray())
    Write-Host ("Wrote {0} ({1:N0} bytes)" -f $OutPath, $out.Count)
}

# ===========================================================================
# 1) Ship (sails up) -- rotation sheet + spin GIF
# ===========================================================================
$shipDir = 'Archive/LegacyRuntimePngExports/assets/sprites/ships/default/sails_up/rotations'
$shipOrder = 'north','north-east','east','south-east','south','south-west','west','north-west'
$shipPaths = $shipOrder | ForEach-Object { Join-Path $shipDir "$_.png" }
$shipFrames = Get-FramesFromFiles -Paths $shipPaths
Write-Host ("Ship: {0} frames at {1}x{2}" -f $shipFrames.Count, $shipFrames[0].Width, $shipFrames[0].Height)
Save-Strip -Frames $shipFrames -OutPath (Join-Path $OutDir 'ship_rotation_sheet.png')
Save-Gif   -Frames $shipFrames -OutPath (Join-Path $OutDir 'ship_rotation_spin.gif') -DelayCs $SpinDelayCs
foreach ($f in $shipFrames) { $f.Dispose() }

# ===========================================================================
# 2) Shipwright (example animated class)
# ===========================================================================
$swDir = 'unity/Assets/_Project/Art/Aseprite/Characters/PlayerDefault/shipwright'

# 2a) 8-direction strip (from the 768x96 day sheet -> 8 cells of 96x96)
$swDir8 = Get-FramesFromSheet -SheetPath (Join-Path $swDir '8_direction_day-Sheet.png') -FrameW 96 -FrameH 96 -Count 8
Save-Strip -Frames $swDir8 -OutPath (Join-Path $OutDir 'shipwright_8dir_sheet.png')
Save-Gif   -Frames $swDir8 -OutPath (Join-Path $OutDir 'shipwright_8dir_turn.gif') -DelayCs $SpinDelayCs
foreach ($f in $swDir8) { $f.Dispose() }

# 2b) Idle loop (7 individual frames, south-facing)
$idlePaths = 1..7 | ForEach-Object { Join-Path $swDir "idle_south-Sheet$_.png" }
$swIdle = Get-FramesFromFiles -Paths $idlePaths
Save-Gif -Frames $swIdle -OutPath (Join-Path $OutDir 'shipwright_idle.gif') -DelayCs $IdleDelayCs
foreach ($f in $swIdle) { $f.Dispose() }

Write-Host "Done."
