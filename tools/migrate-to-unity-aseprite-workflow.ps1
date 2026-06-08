#requires -Version 5.1
<#
.SYNOPSIS
    Migrate Blood and Bilgewater from the legacy Rust/Bevy layout to a clean Unity 2D
    project where .aseprite files are the source of truth (imported by Unity's 2D
    Aseprite Importer).

.DESCRIPTION
    DRY-RUN BY DEFAULT. Prints every proposed create/move/delete/update and changes
    nothing. Pass -Apply to actually perform the operations.

    Safety guarantees:
      * Never deletes source art (.ase/.aseprite), documents, fonts, audio, or PNGs.
      * Never deletes unity/Assets, unity/Packages, unity/ProjectSettings.
      * Only deletes clearly generated caches: target/, unity/Library, unity/Temp,
        unity/Obj, unity/Logs, unity/UserSettings.
      * Archives instead of deleting for everything else.
      * Prefers `git mv` for tracked files, falls back to Move-Item for untracked.
      * Does NOT enter the nested git repo at unity/BloodandBildgewater/.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\tools\migrate-to-unity-aseprite-workflow.ps1
.EXAMPLE
    powershell -ExecutionPolicy Bypass -File .\tools\migrate-to-unity-aseprite-workflow.ps1 -Apply
#>

[CmdletBinding()]
param(
    [switch]$Apply
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# ---------------------------------------------------------------------------
# Resolve repo root (parent of this script's tools/ folder) and move into it.
# ---------------------------------------------------------------------------
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RepoRoot  = Split-Path -Parent $ScriptDir
Set-Location $RepoRoot

$Mode = if ($Apply) { 'APPLY' } else { 'DRY-RUN' }

# Aseprite + Archive roots
$AseRoot     = 'unity/Assets/_Project/Art/Aseprite'
$ReviewRoot  = 'unity/Assets/_Project/Art/LegacyPngOnlyReview'
$ArchiveRoot = 'Archive'

# Collected log lines (written to docs/MIGRATION_LOG_UNITY_ASEPRITE.md)
$script:LogLines = New-Object System.Collections.Generic.List[string]
$script:Warnings = New-Object System.Collections.Generic.List[string]
$script:GitAvailable = $null
# Tracks directories already created/announced so dry-run does not log duplicates.
$script:EnsuredDirs = New-Object 'System.Collections.Generic.HashSet[string]' ([System.StringComparer]::OrdinalIgnoreCase)

# ===========================================================================
# Helper functions
# ===========================================================================

function Write-Step {
    param(
        [Parameter(Mandatory)][string]$Message,
        [ValidateSet('info','mkdir','move','delete','skip','warn','update','header')]
        [string]$Kind = 'info'
    )
    $color = switch ($Kind) {
        'mkdir'  { 'Cyan' }
        'move'   { 'Green' }
        'delete' { 'Red' }
        'skip'   { 'DarkGray' }
        'warn'   { 'Yellow' }
        'update' { 'Magenta' }
        'header' { 'White' }
        default  { 'Gray' }
    }
    $prefix = "[$Mode]"
    Write-Host "$prefix $Message" -ForegroundColor $color
    $script:LogLines.Add("- **$Kind** -- $Message")
    if ($Kind -eq 'warn') { $script:Warnings.Add($Message) }
}

function Invoke-Git {
    <#
        Runs git with all output captured (stderr merged into stdout and discarded) so a
        non-zero exit + stderr text never trips $ErrorActionPreference='Stop'. Returns the
        captured output; the git exit code is available to the caller via $LASTEXITCODE.
    #>
    param([Parameter(Mandatory, ValueFromRemainingArguments)][string[]]$GitArgs)
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $out = & git @GitArgs 2>&1
    } catch {
        $out = $_.Exception.Message
    } finally {
        $ErrorActionPreference = $prev
    }
    return $out
}

function Test-GitAvailable {
    if ($null -ne $script:GitAvailable) { return $script:GitAvailable }
    try {
        $null = Invoke-Git rev-parse --is-inside-work-tree
        $script:GitAvailable = ($LASTEXITCODE -eq 0)
    } catch {
        $script:GitAvailable = $false
    }
    return $script:GitAvailable
}

function Ensure-Directory {
    param([Parameter(Mandatory)][string]$Path)
    if ([string]::IsNullOrWhiteSpace($Path)) { return }
    $norm = ($Path -replace '\\','/').TrimEnd('/')
    if (Test-Path -LiteralPath $Path) { [void]$script:EnsuredDirs.Add($norm); return }
    if ($script:EnsuredDirs.Contains($norm)) { return }
    [void]$script:EnsuredDirs.Add($norm)
    Write-Step "MKDIR $Path" 'mkdir'
    if ($Apply) { New-Item -ItemType Directory -Force -Path $Path | Out-Null }
}

function Test-IsGitTracked {
    <# Output-based check (no --error-unmatch) so untracked paths produce no stderr.
       Works for files (returns the path) and directories (lists tracked children). #>
    param([Parameter(Mandatory)][string]$Path)
    if (-not (Test-GitAvailable)) { return $false }
    $out = Invoke-Git ls-files "$Path"
    return -not [string]::IsNullOrWhiteSpace(($out | Out-String))
}

function Move-RepoItem {
    <#
        Safe move for a single file OR directory.
        - Idempotent: skips if source missing (and notes if dest already exists).
        - Refuses to overwrite an existing destination.
        - Uses `git mv` when the item is tracked; falls back to Move-Item otherwise.
    #>
    param(
        [Parameter(Mandatory)][string]$Source,
        [Parameter(Mandatory)][string]$Destination
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        if (Test-Path -LiteralPath $Destination) {
            Write-Step "SKIP (already moved): $Source -> $Destination" 'skip'
        } else {
            Write-Step "SKIP (source absent): $Source" 'skip'
        }
        return
    }

    if (Test-Path -LiteralPath $Destination) {
        Write-Step "CONFLICT (destination exists) -- leaving in place: $Source -> $Destination" 'warn'
        return
    }

    $destParent = Split-Path -Parent $Destination
    Ensure-Directory $destParent

    $tracked = Test-IsGitTracked $Source
    $how = if ($tracked) { 'git' } else { 'fs' }
    Write-Step ("MOVE [{0}] {1} -> {2}" -f $how, $Source, $Destination) 'move'

    if (-not $Apply) { return }

    if ($tracked) {
        $null = Invoke-Git mv "$Source" "$Destination"
        if ($LASTEXITCODE -ne 0) {
            Write-Step "git mv failed, falling back to Move-Item: $Source" 'warn'
            Ensure-Directory $destParent
            Move-Item -LiteralPath $Source -Destination $Destination -Force
        }
    } else {
        Move-Item -LiteralPath $Source -Destination $Destination -Force
    }
}

function Remove-GeneratedItem {
    <# Deletes a clearly-generated cache path. Refuses obviously protected paths. #>
    param([Parameter(Mandatory)][string]$Path)

    $protected = @('unity/Assets','unity/Packages','unity/ProjectSettings')
    $norm = ($Path -replace '\\','/').TrimEnd('/')
    foreach ($p in $protected) {
        if ($norm -ieq $p) {
            Write-Step "REFUSED to delete protected path: $Path" 'warn'
            return
        }
    }

    if (-not (Test-Path -LiteralPath $Path)) {
        Write-Step "SKIP delete (absent): $Path" 'skip'
        return
    }
    Write-Step "DELETE (generated) $Path" 'delete'
    if ($Apply) { Remove-Item -LiteralPath $Path -Recurse -Force }
}

function Append-IfMissing {
    <#
        Ensures $File contains $Block. Uses a unique $Marker line to detect presence so
        the operation is idempotent. Creates the file if absent.
    #>
    param(
        [Parameter(Mandatory)][string]$File,
        [Parameter(Mandatory)][string]$Marker,
        [Parameter(Mandatory)][string]$Block
    )

    $exists  = Test-Path -LiteralPath $File
    $content = if ($exists) { Get-Content -LiteralPath $File -Raw } else { '' }

    if ($content -like "*$Marker*") {
        Write-Step "SKIP (already present): block '$Marker' in $File" 'skip'
        return
    }

    Write-Step "UPDATE append block '$Marker' -> $File" 'update'
    if (-not $Apply) { return }

    if (-not $exists) {
        Ensure-Directory (Split-Path -Parent $File)
        Set-Content -LiteralPath $File -Value $Block -Encoding UTF8 -NoNewline
    } else {
        $sep = if ($content.Length -gt 0 -and -not $content.EndsWith("`n")) { "`r`n" } else { '' }
        Add-Content -LiteralPath $File -Value ($sep + $Block) -Encoding UTF8
    }
}

function Copy-OrMoveWithReport {
    <#
        Generic wrapper: moves $Source -> $Destination via Move-RepoItem and records a
        review note in the log (used for PNG-only / needs-review assets).
    #>
    param(
        [Parameter(Mandatory)][string]$Source,
        [Parameter(Mandatory)][string]$Destination,
        [string]$Reason = ''
    )
    if ($Reason) { Write-Step "REVIEW: $Reason ($Source)" 'warn' }
    Move-RepoItem -Source $Source -Destination $Destination
}

# ===========================================================================
# Aseprite destination mapping (ordered; first match wins)
# Operates on forward-slash relative paths.
# ===========================================================================
$AsepriteRules = @(
    @{ p = '^assets/source/aseprite/characters/player_default/(.*)$'; r = 'Characters/PlayerDefault/$1' }
    @{ p = '^assets/source/aseprite/characters/(.*)$';                r = 'Characters/$1' }
    @{ p = '^assets/source/aseprite/tilesets/ocean/shallow/(.*)$';    r = 'Tilesets/ShallowWater/$1' }
    @{ p = '^assets/source/aseprite/tilesets/ocean/(.*)$';            r = 'Tilesets/Ocean/$1' }
    @{ p = '^assets/source/aseprite/tilesets/beach/(.*)$';            r = 'Tilesets/Beach/$1' }
    @{ p = '^assets/source/aseprite/tilesets/cove/(.*)$';             r = 'Tilesets/Cove/$1' }
    @{ p = '^assets/source/aseprite/tilesets/ships/(.*)$';            r = 'Tilesets/Ships/$1' }
    @{ p = '^assets/source/aseprite/tilesets/structures/(.*)$';       r = 'Tilesets/Structures/$1' }
    @{ p = '^assets/source/aseprite/tilesets/(.*)$';                  r = 'Tilesets/$1' }
    @{ p = '^assets/source/aseprite/ui/menus/(.*)$';                  r = 'UI/Menus/$1' }
    @{ p = '^assets/source/aseprite/ui/hud/(.*)$';                    r = 'UI/HUD/$1' }
    @{ p = '^assets/source/aseprite/ui/icons/(.*)$';                  r = 'UI/Icons/$1' }
    @{ p = '^assets/source/aseprite/ui/titlescreen/(.*)$';            r = 'UI/TitleScreen/$1' }
    @{ p = '^assets/source/aseprite/ui/(.*)$';                        r = 'UI/Menus/$1' }
    @{ p = '^assets/source/aseprite/props/(.*)$';                     r = 'Props/$1' }
    @{ p = '^assets/source/aseprite/sprites/ships/(.*)$';             r = 'Ships/$1' }
    @{ p = '^assets/source/aseprite/sprites/(.*)$';                   r = '$1' }
    @{ p = '^assets/source/aseprite/(.*)$';                           r = '$1' }
    # Strays in runtime export folders -> mapped to their categories
    @{ p = '^assets/sprites/characters/player_default/(.*)$';         r = 'Characters/PlayerDefault/$1' }
    @{ p = '^assets/sprites/characters/(.*)$';                        r = 'Characters/$1' }
    @{ p = '^assets/sprites/ships/(.*)$';                             r = 'Ships/$1' }
    @{ p = '^assets/sprites/props/(.*)$';                             r = 'Props/$1' }
    @{ p = '^assets/sprites/creatures/(.*)$';                         r = 'Characters/Enemies/$1' }
    @{ p = '^assets/sprites/effects/(.*)$';                           r = 'FX/$1' }
    @{ p = '^assets/sprites/(.*)$';                                   r = '$1' }
    @{ p = '^assets/ui/titlescreen/(.*)$';                            r = 'UI/TitleScreen/$1' }
    @{ p = '^assets/ui/menus/(.*)$';                                  r = 'UI/Menus/$1' }
    @{ p = '^assets/ui/hud/(.*)$';                                    r = 'UI/HUD/$1' }
    @{ p = '^assets/ui/icons/(.*)$';                                  r = 'UI/Icons/$1' }
    @{ p = '^assets/ui/(.*)$';                                        r = 'UI/Menus/$1' }
    @{ p = '^assets/tilesets/ocean/shallow/(.*)$';                    r = 'Tilesets/ShallowWater/$1' }
    @{ p = '^assets/tilesets/ocean/(.*)$';                            r = 'Tilesets/Ocean/$1' }
    @{ p = '^assets/tilesets/beach/(.*)$';                            r = 'Tilesets/Beach/$1' }
    @{ p = '^assets/tilesets/cove/(.*)$';                             r = 'Tilesets/Cove/$1' }
    @{ p = '^assets/tilesets/ships/(.*)$';                            r = 'Tilesets/Ships/$1' }
    @{ p = '^assets/tilesets/structures/(.*)$';                       r = 'Tilesets/Structures/$1' }
    @{ p = '^assets/tilesets/(.*)$';                                  r = 'Tilesets/$1' }
)

function Get-AsepriteDestination {
    param([Parameter(Mandatory)][string]$RelPath)
    $rel = $RelPath -replace '\\','/'
    foreach ($rule in $AsepriteRules) {
        if ($rel -match $rule.p) {
            $sub = $rel -replace $rule.p, $rule.r
            return (Join-Path $AseRoot $sub) -replace '\\','/'
        }
    }
    # Fallback: drop into Aseprite root by filename
    return (Join-Path $AseRoot (Split-Path -Leaf $rel)) -replace '\\','/'
}

function Get-RelativePath {
    param([Parameter(Mandatory)]$Item)
    return $Item.FullName.Substring($RepoRoot.Length).TrimStart('\','/') -replace '\\','/'
}

# ===========================================================================
# MIGRATION PHASES
# ===========================================================================

Write-Host ""
Write-Step "Blood and Bilgewater -> Unity 2D + Aseprite migration ($Mode)" 'header'
Write-Step "Repo root: $RepoRoot" 'info'
if (-not (Test-GitAvailable)) {
    Write-Step "git not available -- all moves will use the filesystem (Move-Item)." 'warn'
}

# --- PHASE 0: nested-repo guard ------------------------------------------------
Write-Host ""; Write-Step "PHASE 0 -- Safety guards" 'header'
if (Test-Path 'unity/.git') {
    Write-Step "unity/.git exists -- STOPPING. Resolve the nested repo first." 'warn'
    return
}
$nested = 'unity/BloodandBildgewater'
if (Test-Path (Join-Path $nested '.git')) {
    Write-Step "Nested git repo detected at $nested/.git -- it will NOT be modified by this script. Resolve it manually (see audit)." 'warn'
}

# --- PHASE 3: create target structure -----------------------------------------
Write-Host ""; Write-Step "PHASE 3 -- Create Unity + Archive structure" 'header'
$dirs = @(
    "$AseRoot/Characters/PlayerDefault",
    "$AseRoot/Characters/NPCs",
    "$AseRoot/Characters/Enemies",
    "$AseRoot/Tilesets/Ocean",
    "$AseRoot/Tilesets/ShallowWater",
    "$AseRoot/Tilesets/Beach",
    "$AseRoot/Tilesets/Cove",
    "$AseRoot/Tilesets/Ships",
    "$AseRoot/Tilesets/Structures",
    "$AseRoot/UI/Menus",
    "$AseRoot/UI/HUD",
    "$AseRoot/UI/Icons",
    "$AseRoot/UI/TitleScreen",
    "$AseRoot/Props",
    "$AseRoot/Ships",
    "$AseRoot/FX",
    'unity/Assets/_Project/Art/References',
    $ReviewRoot,
    'unity/Assets/_Project/Animations',
    'unity/Assets/_Project/Audio/Music',
    'unity/Assets/_Project/Audio/SFX',
    'unity/Assets/_Project/Audio/Ambience',
    'unity/Assets/_Project/Data/Characters',
    'unity/Assets/_Project/Data/Roles',
    'unity/Assets/_Project/Data/Items',
    'unity/Assets/_Project/Data/Ships',
    'unity/Assets/_Project/Data/World',
    'unity/Assets/_Project/Data/LegacyJson',
    'unity/Assets/_Project/Materials',
    'unity/Assets/_Project/Prefabs/Player',
    'unity/Assets/_Project/Prefabs/UI',
    'unity/Assets/_Project/Prefabs/Tiles',
    'unity/Assets/_Project/Prefabs/Ships',
    'unity/Assets/_Project/Scenes',
    'unity/Assets/_Project/Scripts/Core',
    'unity/Assets/_Project/Scripts/Player',
    'unity/Assets/_Project/Scripts/Camera',
    'unity/Assets/_Project/Scripts/UI',
    'unity/Assets/_Project/Scripts/Scenes',
    'unity/Assets/_Project/Scripts/Data',
    'unity/Assets/_Project/Scripts/Gameplay/Combat',
    'unity/Assets/_Project/Scripts/Gameplay/Inventory',
    'unity/Assets/_Project/Scripts/Gameplay/Roles',
    'unity/Assets/_Project/Scripts/Gameplay/Ship',
    'unity/Assets/_Project/Scripts/Gameplay/Home',
    'unity/Assets/_Project/ScriptableObjects/Characters',
    'unity/Assets/_Project/ScriptableObjects/Roles',
    'unity/Assets/_Project/ScriptableObjects/Items',
    'unity/Assets/_Project/Settings',
    'unity/Assets/_Project/Tilemaps',
    'unity/Assets/_Project/Editor',
    'unity/Assets/_Project/UI/Fonts',
    "$ArchiveRoot/BevyReference",
    "$ArchiveRoot/LegacyRuntimePngExports",
    "$ArchiveRoot/UnityTemplateArtifacts",
    "$ArchiveRoot/MigrationReports"
)
foreach ($d in $dirs) { Ensure-Directory $d }

# --- PHASE 4: move all .aseprite/.ase into Unity ------------------------------
Write-Host ""; Write-Step "PHASE 4 -- Move Aseprite source files into Unity" 'header'
$aseFiles = Get-ChildItem -Path 'assets' -Recurse -Force -Include *.ase,*.aseprite -ErrorAction SilentlyContinue
$aseCount = 0
foreach ($f in $aseFiles) {
    $rel  = Get-RelativePath $f
    $dest = Get-AsepriteDestination $rel
    Move-RepoItem -Source $rel -Destination $dest
    $aseCount++
}
Write-Step "Aseprite files processed: $aseCount" 'info'

# --- PHASE 5: references / fonts / audio / data -------------------------------
Write-Host ""; Write-Step "PHASE 5 -- Move references, fonts, audio, data" 'header'
Move-RepoItem -Source 'assets/source/references' -Destination 'unity/Assets/_Project/Art/References/source'
Move-RepoItem -Source 'assets/fonts'             -Destination 'unity/Assets/_Project/UI/Fonts/legacy'
Move-RepoItem -Source 'assets/audio'             -Destination 'unity/Assets/_Project/Audio/legacy'
Move-RepoItem -Source 'assets/data'              -Destination 'unity/Assets/_Project/Data/LegacyJson/data'

# --- PHASE 6: archive leftover PNG/runtime folders ----------------------------
Write-Host ""; Write-Step "PHASE 6 -- Archive legacy PNG / runtime exports" 'header'
Write-Step "Note: .aseprite files were already extracted in PHASE 4." 'info'
Move-RepoItem -Source 'assets/sprites'  -Destination "$ArchiveRoot/LegacyRuntimePngExports/assets/sprites"
Move-RepoItem -Source 'assets/tilesets' -Destination "$ArchiveRoot/LegacyRuntimePngExports/assets/tilesets"
Move-RepoItem -Source 'assets/ui'       -Destination "$ArchiveRoot/LegacyRuntimePngExports/assets/ui"

# --- PHASE 7: archive Bevy/Rust, delete target/ -------------------------------
Write-Host ""; Write-Step "PHASE 7 -- Archive Rust/Bevy reference, delete target/" 'header'
Move-RepoItem -Source 'src'        -Destination "$ArchiveRoot/BevyReference/src"
Move-RepoItem -Source 'examples'   -Destination "$ArchiveRoot/BevyReference/examples"
Move-RepoItem -Source 'Cargo.toml' -Destination "$ArchiveRoot/BevyReference/Cargo.toml"
Move-RepoItem -Source 'Cargo.lock' -Destination "$ArchiveRoot/BevyReference/Cargo.lock"
Remove-GeneratedItem 'target'

# --- PHASE 8: archive Unity template artifacts --------------------------------
Write-Host ""; Write-Step "PHASE 8 -- Archive Unity template artifacts" 'header'
Move-RepoItem -Source 'unity/Assets/TutorialInfo'      -Destination "$ArchiveRoot/UnityTemplateArtifacts/TutorialInfo"
Move-RepoItem -Source 'unity/Assets/TutorialInfo.meta' -Destination "$ArchiveRoot/UnityTemplateArtifacts/TutorialInfo.meta"
Move-RepoItem -Source 'unity/Assets/Readme.asset'      -Destination "$ArchiveRoot/UnityTemplateArtifacts/Readme.asset"
Move-RepoItem -Source 'unity/Assets/Readme.asset.meta' -Destination "$ArchiveRoot/UnityTemplateArtifacts/Readme.asset.meta"
Move-RepoItem -Source 'unity/Assets/_Recovery'         -Destination "$ArchiveRoot/UnityTemplateArtifacts/_Recovery"
Move-RepoItem -Source 'unity/Assets/_Recovery.meta'    -Destination "$ArchiveRoot/UnityTemplateArtifacts/_Recovery.meta"
Write-Step "Leaving unity/Assets/Scenes/SampleScene.unity in place (rename after Unity opens)." 'info'

# --- PHASE 9: fix root .gitignore ---------------------------------------------
Write-Host ""; Write-Step "PHASE 9 -- Update root .gitignore" 'header'
$gitignoreBlock = @'
# === Unity / Rust migration (managed block) ===
# Unity generated files
unity/Library/
unity/Temp/
unity/Obj/
unity/Logs/
unity/UserSettings/
unity/MemoryCaptures/
unity/Recordings/
unity/Build/
unity/Builds/

# Unity generated IDE/project files
unity/*.csproj
unity/*.sln
unity/*.suo
unity/*.user
unity/*.userprefs
unity/*.pidb
unity/*.booproj
unity/*.svd
unity/*.pdb
unity/*.mdb
unity/*.opendb
unity/*.VC.db
*.csproj
*.sln
*.suo
*.user
*.userprefs

# Rust build output
target/

# OS / IDE
.vs/
.idea/
.DS_Store
Thumbs.db
desktop.ini
'@
Append-IfMissing -File '.gitignore' -Marker 'Unity / Rust migration (managed block)' -Block $gitignoreBlock

# --- PHASE 10: root .gitattributes + LFS --------------------------------------
Write-Host ""; Write-Step "PHASE 10 -- Root .gitattributes + Git LFS" 'header'
$gitattributes = @'
# === Blood and Bilgewater root .gitattributes (single top-level file) ===

# Aseprite / source art
*.ase filter=lfs diff=lfs merge=lfs -text
*.aseprite filter=lfs diff=lfs merge=lfs -text

# Raster art and references
*.png filter=lfs diff=lfs merge=lfs -text
*.jpg filter=lfs diff=lfs merge=lfs -text
*.jpeg filter=lfs diff=lfs merge=lfs -text
*.webp filter=lfs diff=lfs merge=lfs -text
*.psd filter=lfs diff=lfs merge=lfs -text

# Fonts
*.ttf filter=lfs diff=lfs merge=lfs -text
*.otf filter=lfs diff=lfs merge=lfs -text

# Audio
*.wav filter=lfs diff=lfs merge=lfs -text
*.mp3 filter=lfs diff=lfs merge=lfs -text
*.ogg filter=lfs diff=lfs merge=lfs -text

# Unity YAML assets
*.unity merge=unityyamlmerge eol=lf
*.prefab merge=unityyamlmerge eol=lf
*.asset merge=unityyamlmerge eol=lf
*.mat merge=unityyamlmerge eol=lf
*.controller merge=unityyamlmerge eol=lf
*.anim merge=unityyamlmerge eol=lf
*.meta eol=lf

# Code and docs
*.cs eol=lf
*.json eol=lf
*.md eol=lf
'@
$gaExists = Test-Path '.gitattributes'
if ($gaExists) {
    Write-Step "Root .gitattributes already exists -- leaving as-is (review manually)." 'skip'
} else {
    Write-Step "CREATE root .gitattributes" 'update'
    if ($Apply) { Set-Content -LiteralPath '.gitattributes' -Value $gitattributes -Encoding UTF8 }
}

if (Test-GitAvailable) {
    $null = Invoke-Git lfs version
    if ($LASTEXITCODE -eq 0) {
        Write-Step "Git LFS detected." 'info'
        if ($Apply) { $null = Invoke-Git lfs install }
    } else {
        Write-Step "Git LFS NOT installed -- install it before pushing large binary assets." 'warn'
    }
}

# Warn about nested .gitattributes with [attr] macros (do not edit inside nested repo).
$nestedGa = Join-Path $nested '.gitattributes'
if (Test-Path $nestedGa) {
    Write-Step "Nested $nestedGa uses [attr] macros and lives inside a nested repo -- resolve with the nested project, not here." 'warn'
}

# ===========================================================================
# Write migration log
# ===========================================================================
Write-Host ""; Write-Step "Writing migration log" 'header'
$logHeader = @"
# Migration Log -- Unity 2D + Aseprite Workflow

**Mode:** $Mode
**Run at:** $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
**Repo:** $RepoRoot

> $(if ($Apply) { 'Changes WERE applied.' } else { 'DRY-RUN only. No files were changed.' })

## Operations
"@
$logBody = ($script:LogLines -join "`r`n")
$warnSection = if ($script:Warnings.Count -gt 0) {
    "`r`n`r`n## Warnings`r`n" + (($script:Warnings | ForEach-Object { "- $_" }) -join "`r`n")
} else { "`r`n`r`n## Warnings`r`n- none" }

$logPath = 'docs/MIGRATION_LOG_UNITY_ASEPRITE.md'
Ensure-Directory 'docs'
if ($Apply) {
    Set-Content -LiteralPath $logPath -Value ($logHeader + "`r`n" + $logBody + $warnSection) -Encoding UTF8
    Write-Host "[$Mode] Log written to $logPath" -ForegroundColor White
} else {
    Write-Host "[$Mode] (log would be written to $logPath on -Apply)" -ForegroundColor DarkGray
}

# ===========================================================================
# Summary
# ===========================================================================
Write-Host ""
Write-Host "==================== SUMMARY ($Mode) ====================" -ForegroundColor White
Write-Host ("Aseprite files processed : {0}" -f $aseCount)
Write-Host ("Warnings                 : {0}" -f $script:Warnings.Count)
if ($script:Warnings.Count -gt 0) {
    $script:Warnings | ForEach-Object { Write-Host "  ! $_" -ForegroundColor Yellow }
}
if (-not $Apply) {
    Write-Host ""
    Write-Host "DRY-RUN complete. Nothing changed." -ForegroundColor Cyan
    Write-Host "To apply:" -ForegroundColor Cyan
    Write-Host "  powershell -ExecutionPolicy Bypass -File .\tools\migrate-to-unity-aseprite-workflow.ps1 -Apply" -ForegroundColor Cyan
} else {
    Write-Host ""
    Write-Host "APPLY complete. Review with: git status --short" -ForegroundColor Green
}
