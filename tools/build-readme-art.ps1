#requires -Version 5.1
<#
.SYNOPSIS
    Build README preview art from existing game frames:
      1. A horizontal sprite-sheet strip PNG of the 8 ship rotation frames.
      2. A looping animated GIF that "spins" the ship through those 8 directions.

    Frames are composited onto a solid dark backdrop so the output has no GIF
    transparency artifacts and matches the game's gothic tone. Source PNGs are
    never modified.

.NOTES
    Pure .NET (System.Drawing + a small hand-rolled GIF89a assembler), so it runs
    on stock Windows PowerShell without ImageMagick/ffmpeg/Python.
#>

[CmdletBinding()]
param(
    [int]$FrameDelayCs = 12  # per-frame delay in 1/100 second
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir
Set-Location $RepoRoot

$RotDir = 'Archive/LegacyRuntimePngExports/assets/sprites/ships/default/Dark_blue_glowing_sp/rotations'
# Clockwise spin order.
$Order  = 'north','north-east','east','south-east','south','south-west','west','north-west'
$OutDir = 'docs/art/preview'
$SheetPath = Join-Path $OutDir 'ship_rotation_sheet.png'
$GifPath   = Join-Path $OutDir 'ship_rotation_spin.gif'

$BackColor = [System.Drawing.Color]::FromArgb(255, 11, 14, 26)  # #0B0E1A gothic near-black blue

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

# --- Load + composite frames onto the dark backdrop ---------------------------
$frames = @()
$w = 0; $h = 0
foreach ($name in $Order) {
    $p = Join-Path $RotDir "$name.png"
    if (-not (Test-Path $p)) { throw "Missing frame: $p" }
    $src = [System.Drawing.Image]::FromFile((Resolve-Path $p).Path)
    if ($w -eq 0) { $w = $src.Width; $h = $src.Height }
    $bmp = New-Object System.Drawing.Bitmap($w, $h)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.Clear($BackColor)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::NearestNeighbor
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::Half
    $g.DrawImage($src, 0, 0, $w, $h)
    $g.Dispose(); $src.Dispose()
    $frames += $bmp
}
Write-Host ("Loaded {0} frames at {1}x{2}" -f $frames.Count, $w, $h)

# --- 1) Sprite-sheet strip PNG ------------------------------------------------
$sheet = New-Object System.Drawing.Bitmap(($w * $frames.Count), $h)
$sg = [System.Drawing.Graphics]::FromImage($sheet)
$sg.Clear($BackColor)
for ($i = 0; $i -lt $frames.Count; $i++) { $sg.DrawImage($frames[$i], ($i * $w), 0, $w, $h) }
$sg.Dispose()
$sheet.Save((Join-Path $RepoRoot $SheetPath), [System.Drawing.Imaging.ImageFormat]::Png)
$sheet.Dispose()
Write-Host "Wrote $SheetPath"

# --- GIF helpers --------------------------------------------------------------
function Get-FrameGifParts {
    # Encode a bitmap to a single-frame GIF via GDI+, then parse out its color
    # table + LZW image data so we can re-emit it as one frame of an animation.
    param([System.Drawing.Bitmap]$Bitmap)

    $ms = New-Object System.IO.MemoryStream
    $Bitmap.Save($ms, [System.Drawing.Imaging.ImageFormat]::Gif)
    $b = $ms.ToArray()
    $ms.Dispose()

    $packed = $b[10]
    $gctFlag = ($packed -band 0x80) -ne 0
    $gctSizeBits = $packed -band 0x07
    $gctEntries = [int][Math]::Pow(2, $gctSizeBits + 1)
    $gctLen = if ($gctFlag) { $gctEntries * 3 } else { 0 }

    $globalTable = New-Object byte[] $gctLen
    if ($gctLen -gt 0) { [Array]::Copy($b, 13, $globalTable, 0, $gctLen) }
    $pos = 13 + $gctLen

    $colorTable = $globalTable
    $sizeBits = $gctSizeBits
    $lzwMin = 0
    $imageData = $null

    while ($pos -lt $b.Length) {
        $marker = $b[$pos]
        if ($marker -eq 0x21) {
            # Extension: skip label + sub-blocks.
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
                $colorTable = $localTable
                $sizeBits = $lctSizeBits
                $p2 += $lctLen
            }
            $lzwMin = $b[$p2]; $p2 += 1
            $start = $p2
            while ($b[$p2] -ne 0) { $p2 += ($b[$p2] + 1) }
            $p2 += 1  # include terminating 0x00
            $len = $p2 - $start
            $imageData = New-Object byte[] $len
            [Array]::Copy($b, $start, $imageData, 0, $len)
            break
        }
        else { break }
    }

    return [pscustomobject]@{
        ColorTable = $colorTable
        SizeBits   = $sizeBits
        LzwMin     = $lzwMin
        ImageData  = $imageData
    }
}

function Add-UInt16LE { param([System.Collections.Generic.List[byte]]$List, [int]$Value)
    $List.Add([byte]($Value -band 0xFF)); $List.Add([byte](($Value -shr 8) -band 0xFF))
}

# --- 2) Animated, looping GIF -------------------------------------------------
$out = New-Object 'System.Collections.Generic.List[byte]'
# Header
[void]($out.AddRange([byte[]][System.Text.Encoding]::ASCII.GetBytes('GIF89a')))
# Logical Screen Descriptor (no global color table; per-frame local tables)
Add-UInt16LE $out $w
Add-UInt16LE $out $h
$out.Add([byte]0x70)  # color resolution = 7, no GCT
$out.Add([byte]0x00)  # background color index
$out.Add([byte]0x00)  # pixel aspect ratio
# Netscape 2.0 looping extension (loop forever)
[void]($out.AddRange([byte[]](0x21,0xFF,0x0B)))
[void]($out.AddRange([byte[]][System.Text.Encoding]::ASCII.GetBytes('NETSCAPE2.0')))
[void]($out.AddRange([byte[]](0x03,0x01,0x00,0x00,0x00)))

foreach ($bmp in $frames) {
    $parts = Get-FrameGifParts -Bitmap $bmp
    # Graphic Control Extension (delay, no transparency, disposal = do not dispose)
    [void]($out.AddRange([byte[]](0x21,0xF9,0x04,0x04)))
    Add-UInt16LE $out $FrameDelayCs
    $out.Add([byte]0x00)  # transparent color index (unused)
    $out.Add([byte]0x00)  # block terminator
    # Image Descriptor
    $out.Add([byte]0x2C)
    Add-UInt16LE $out 0
    Add-UInt16LE $out 0
    Add-UInt16LE $out $w
    Add-UInt16LE $out $h
    $out.Add([byte](0x80 -bor ($parts.SizeBits -band 0x07)))  # local color table flag + size
    [void]($out.AddRange($parts.ColorTable))
    $out.Add([byte]$parts.LzwMin)
    [void]($out.AddRange($parts.ImageData))
}
$out.Add([byte]0x3B)  # trailer

[System.IO.File]::WriteAllBytes((Join-Path $RepoRoot $GifPath), $out.ToArray())
Write-Host ("Wrote {0} ({1:N0} bytes)" -f $GifPath, $out.Count)

foreach ($bmp in $frames) { $bmp.Dispose() }
Write-Host "Done."
