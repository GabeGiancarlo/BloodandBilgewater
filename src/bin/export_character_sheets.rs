//! Batch-export character `.aseprite` sources to runtime PNG + JSON hash sheets.
//!
//! Run: `cargo run --bin export_character_sheets --features asset-export`

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use asefile::AsepriteFile;
use image::{imageops, RgbaImage};
use serde_json::json;

const DIRECTIONS: [&str; 8] = [
    "south",
    "southeast",
    "east",
    "northeast",
    "north",
    "northwest",
    "west",
    "southwest",
];

fn main() {
    if let Err(e) = run() {
        eprintln!("export_character_sheets failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let classes = [
        "helmsman",
        "swordsman",
        "marksman",
        "navigator",
        "doctor",
        "shipwright",
        "cook",
        "archaeologist",
        "musician",
    ];
    let mut exported = 0usize;
    let mut skipped = 0usize;

    for class in classes {
        let source_root = root.join(format!(
            "assets/source/characters/player_default/{class}/loadouts"
        ));
        if !source_root.exists() {
            eprintln!("skip missing source: {}", source_root.display());
            continue;
        }
        for entry in walk_aseprites(&source_root) {
            let rel = entry
                .strip_prefix(root.join("assets/source"))
                .map_err(|_| "path prefix")?;
            let stem = entry.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if should_skip_aggregate(stem, &entry) {
                skipped += 1;
                continue;
            }
            let out_dir = root.join("assets/runtime").join(rel.parent().unwrap());
            if stem.ends_with("-8") {
                exported += export_tagged_file(&entry, &out_dir, stem)?;
            } else {
                exported += export_direction_file(&entry, &out_dir, stem)?;
            }
        }
    }

    println!(
        "export_character_sheets: exported {exported} sheets, skipped {skipped} aggregates"
    );
    Ok(())
}

fn walk_aseprites(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.extend(walk_aseprites(&path));
            } else if path.extension().and_then(|e| e.to_str()) == Some("aseprite") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn should_skip_aggregate(stem: &str, path: &Path) -> bool {
    if !stem.ends_with("-8") || stem.starts_with("mid-") {
        return false;
    }
    let prefix = stem.trim_end_matches("-8");
    let parent = path.parent().unwrap();
    for dir in DIRECTIONS {
        if !parent.join(format!("{prefix}-{dir}.aseprite")).exists() {
            return false;
        }
    }
    true
}

fn export_direction_file(path: &Path, out_dir: &Path, stem: &str) -> Result<usize, String> {
    let ase = AsepriteFile::read_file(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let frames: Vec<u32> = (0..ase.num_frames()).collect();
    if frames.is_empty() {
        return Ok(0);
    }
    let out_png = out_dir.join(format!("{stem}-sheet.png"));
    let out_json = out_dir.join(format!("{stem}-sheet.json"));
    write_sheet(&ase, &frames, stem, &out_png, &out_json)?;
    println!("  {}", out_png.display());
    Ok(1)
}

fn export_tagged_file(path: &Path, out_dir: &Path, stem: &str) -> Result<usize, String> {
    let ase = AsepriteFile::read_file(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let prefix = stem.trim_end_matches("-8");
    let tag_count = ase.num_tags();
    if tag_count == 0 {
        let frames: Vec<u32> = (0..ase.num_frames()).collect();
        let out_png = out_dir.join(format!("{prefix}-sheet.png"));
        let out_json = out_dir.join(format!("{prefix}-sheet.json"));
        write_sheet(&ase, &frames, prefix, &out_png, &out_json)?;
        return Ok(1);
    }

    let mut count = 0;
    for tag_id in 0..tag_count {
        let tag = ase.tag(tag_id);
        let tag_name = normalize_direction_tag(tag.name());
        let frames: Vec<u32> = (tag.from_frame()..=tag.to_frame()).collect();
        if frames.is_empty() {
            continue;
        }
        let sheet_stem = format!("{prefix}-{tag_name}");
        let out_png = out_dir.join(format!("{sheet_stem}-sheet.png"));
        let out_json = out_dir.join(format!("{sheet_stem}-sheet.json"));
        write_sheet(&ase, &frames, &sheet_stem, &out_png, &out_json)?;
        println!("  {}", out_png.display());
        count += 1;
    }
    if count == 0 {
        let frames: Vec<u32> = (0..ase.num_frames()).collect();
        if !frames.is_empty() {
            let out_png = out_dir.join(format!("{prefix}-sheet.png"));
            let out_json = out_dir.join(format!("{prefix}-sheet.json"));
            write_sheet(&ase, &frames, prefix, &out_png, &out_json)?;
            println!("  {}", out_png.display());
            count = 1;
        }
    }
    Ok(count)
}

fn normalize_direction_tag(name: &str) -> String {
    let lower = name.to_lowercase().replace('_', "-");
    match lower.as_str() {
        "south-east" | "southe" => "southeast".to_string(),
        "north-east" | "northe" => "northeast".to_string(),
        "south-west" | "southw" => "southwest".to_string(),
        "north-west" | "northw" => "northwest".to_string(),
        other => other.to_string(),
    }
}

fn write_sheet(
    ase: &AsepriteFile,
    frame_indices: &[u32],
    stem: &str,
    png_path: &Path,
    json_path: &Path,
) -> Result<(), String> {
    if let Some(parent) = png_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let first = ase.frame(frame_indices[0]).image();
    let fw = first.width();
    let fh = first.height();
    let sheet_w = fw * frame_indices.len() as u32;
    let mut sheet = RgbaImage::new(sheet_w, fh);

    let mut frames_json = HashMap::new();
    for (i, &frame_idx) in frame_indices.iter().enumerate() {
        let img = ase.frame(frame_idx).image();
        imageops::overlay(&mut sheet, &img, i as i64 * fw as i64, 0);

        let duration = ase.frame(frame_idx).duration();
        let key = format!("{stem} {i}.aseprite");
        let x = i as u32 * fw;
        frames_json.insert(
            key,
            json!({
                "frame": { "x": x, "y": 0, "w": fw, "h": fh },
                "rotated": false,
                "trimmed": false,
                "spriteSourceSize": { "x": 0, "y": 0, "w": fw, "h": fh },
                "sourceSize": { "w": fw, "h": fh },
                "duration": duration
            }),
        );
    }

    sheet
        .save(png_path)
        .map_err(|e| format!("save png {}: {e}", png_path.display()))?;

    let meta = json!({
        "app": "https://www.aseprite.org/",
        "version": "export_character_sheets",
        "format": "RGBA8888",
        "size": { "w": sheet_w, "h": fh },
        "scale": "1",
        "frameTags": [],
        "layers": [{ "name": "Layer", "opacity": 255, "blendMode": "normal" }],
        "slices": []
    });

    let root = json!({ "frames": frames_json, "meta": meta });
    fs::write(json_path, serde_json::to_string_pretty(&root).unwrap())
        .map_err(|e| format!("save json {}: {e}", json_path.display()))?;
    Ok(())
}
