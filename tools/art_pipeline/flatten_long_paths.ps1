<#
.SYNOPSIS
    Flatten over-long Windows paths under assets/textures created by the deep
    AI-generation frame dumps (doctor / helmsman / marksman / shipwright).

.DESCRIPTION
    Some Unity-phase character art was generated into directories named after the
    full English generation prompt (e.g. "The_character_stands_firmly_bracing_their_shoulder"),
    nested several levels deep. These blow past the Windows 260-char path limit.

    This script repeatedly shortens the longest offending directory LEAF names
    (segments below the class folder) to "<8chars>~<4hexhash>" until every path is
    comfortably under the limit. File names are never changed. Every directory rename
    is appended to docs/migration/asset_move_log.md so the operation is reversible.

    Uses .NET Directory.Move with the \\?\ extended-length prefix and cmd `dir`
    enumeration so long paths can be read and moved reliably.
#>

$ErrorActionPreference = 'Stop'
$repo = (Resolve-Path "$PSScriptRoot\..\..").Path
$root = Join-Path $repo "assets\textures\characters\player_default"
$log  = Join-Path $repo "docs\migration\asset_move_log.md"
$limit = 230   # rename any directory whose full path is at least this long

if (-not (Test-Path $root)) { throw "textures root not found: $root" }

$md5 = [System.Security.Cryptography.MD5]::Create()
function Hash4([string]$s) {
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($s)
    ($md5.ComputeHash($bytes) | ForEach-Object { $_.ToString('x2') }) -join '' | ForEach-Object { $_.Substring(0,4) }
}
function LP([string]$p) { "\\?\" + $p }

$renames = New-Object System.Collections.Generic.List[object]
$pass = 0
while ($true) {
    $pass++
    # cmd `dir` enumerates long paths safely; /ad = dirs only, /b = bare, /s = recurse
    $dirs = @(& cmd /c "dir `"$root`" /s /b /ad" 2>$null) |
        ForEach-Object { "$_".Trim() } |
        Where-Object { $_.Length -ge $limit } |
        Sort-Object { $_.Length } -Descending
    if (@($dirs).Count -eq 0) { break }
    if ($pass -gt 40) { Write-Warning "stopping after 40 passes"; break }

    $changed = $false
    foreach ($full in $dirs) {
        $full = "$full"
        if ($full.Length -lt 1) { continue }
        $lpFull = '\\?\' + $full
        if (-not (Test-Path -LiteralPath $lpFull)) { continue }  # parent already renamed this pass
        $parent = Split-Path $full -Parent
        $leaf   = Split-Path $full -Leaf
        if ($leaf.Length -le 14) { continue }
        $short = ($leaf.Substring(0, [Math]::Min(8, $leaf.Length))) + '~' + (Hash4 $leaf)
        $dest  = Join-Path $parent $short
        if (Test-Path -LiteralPath ('\\?\' + $dest)) { $short = $short + '_' + (Hash4 $full); $dest = Join-Path $parent $short }
        [System.IO.Directory]::Move($lpFull, ('\\?\' + $dest))
        $relOld = $full.Substring($repo.Length).TrimStart('\')
        $relNew = $dest.Substring($repo.Length).TrimStart('\')
        $renames.Add([pscustomobject]@{ Old = $relOld; New = $relNew })
        $changed = $true
    }
    if (-not $changed) { break }
}

Write-Host "Flatten complete. Directory renames: $($renames.Count)"

# Append the rename table to the move log
if ($renames.Count -gt 0) {
    $lines = @()
    $lines += ""
    $lines += "### Long-path flatten renames ($(Get-Date -Format yyyy-MM-dd))"
    $lines += ""
    $lines += "Deep AI-generation frame directories shortened to keep all paths under the Windows 260-char limit. File names unchanged."
    $lines += ""
    $lines += "| Old directory | New directory |"
    $lines += "| --- | --- |"
    foreach ($r in $renames) { $lines += "| ``$($r.Old)`` | ``$($r.New)`` |" }
    Add-Content -LiteralPath $log -Value $lines -Encoding utf8
    Write-Host "Appended $($renames.Count) renames to $log"
}
