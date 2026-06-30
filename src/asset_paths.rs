//! Resolve files under the repo `assets/` directory regardless of process cwd.

use std::path::PathBuf;

/// Absolute path to the repository `assets/` folder.
pub fn assets_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets")
}

/// Absolute path to `assets/<rel>`.
pub fn assets_relative_path(rel: &str) -> PathBuf {
    assets_root().join(rel)
}

/// Whether `assets/<rel>` exists on disk.
pub fn asset_exists(rel: &str) -> bool {
    assets_relative_path(rel).exists()
}

/// Bevy `AssetPlugin` `file_path` pointing at the repo `assets/` folder.
pub fn assets_file_path() -> String {
    assets_root().to_string_lossy().into_owned()
}

/// First existing PNG among lowercase / capital `sheet` suffix variants.
pub fn resolve_sheet_png(base_without_ext: &str) -> Option<String> {
    let trimmed = base_without_ext.trim_end_matches("-sheet");
    for candidate in [
        format!("{base_without_ext}.png"),
        format!("{trimmed}-sheet.png"),
        format!("{trimmed}-Sheet.png"),
    ] {
        if asset_exists(&candidate) {
            return Some(candidate);
        }
    }
    None
}

/// Pair of (`png`, `json`) paths for `.../idle-south-sheet` (no extension).
pub fn resolve_sheet_pair(sheet_base: &str) -> Option<(String, String)> {
    let png = resolve_sheet_png(sheet_base)?;
    let json = format!("{sheet_base}.json");
    Some((png, json))
}
