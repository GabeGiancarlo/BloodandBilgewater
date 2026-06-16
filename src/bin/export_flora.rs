//! Export tree + fruit `.aseprite` sources to mirrored `assets/runtime/props/flora/`.
//!
//! Run: `cargo run --bin export_flora --features asset-export`

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use asefile::AsepriteFile;
use image::{imageops, RgbaImage};
use serde_json::json;

fn main() {
    if let Err(e) = run() {
        eprintln!("export_flora failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut exported = 0usize;

    let trees_root = root.join("assets/source/props/flora/trees");
    if trees_root.exists() {
        for entry in walk_aseprites(&trees_root) {
            exported += export_tree_png(&entry, &root)?;
        }
    }

    let fruit_root = root.join("assets/source/props/flora/fruit");
    if fruit_root.exists() {
        for entry in walk_aseprites(&fruit_root) {
            exported += export_fruit(&entry, &root)?;
        }
    }

    println!("export_flora: exported {exported} flora assets");
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

fn export_tree_png(path: &Path, root: &Path) -> Result<usize, String> {
    let rel = path
        .strip_prefix(root.join("assets/source"))
        .map_err(|_| "path prefix")?;
    let out_dir = root.join("assets/runtime").join(rel.parent().unwrap());
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("sprite");
    let out_png = out_dir.join(format!("{stem}.png"));

    let ase = AsepriteFile::read_file(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if ase.num_frames() == 0 {
        return Ok(0);
    }
    let img = ase.frame(0).image();
    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    img.save(&out_png).map_err(|e| format!("save {}: {e}", out_png.display()))?;
    println!("  {}", out_png.display());
    Ok(1)
}

fn export_fruit(path: &Path, root: &Path) -> Result<usize, String> {
    let rel = path
        .strip_prefix(root.join("assets/source"))
        .map_err(|_| "path prefix")?;
    let out_dir = root.join("assets/runtime").join(rel.parent().unwrap());
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("fruit");

    let ase = AsepriteFile::read_file(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let frame_count = ase.num_frames();
    if frame_count == 0 {
        return Ok(0);
    }

    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

    let frames: Vec<u32> = (0..frame_count).collect();
    let sheet_stem = format!("{stem}-sheet");
    let sheet_png = out_dir.join(format!("{sheet_stem}.png"));
    let sheet_json = out_dir.join(format!("{sheet_stem}.json"));
    write_sheet(&ase, &frames, &sheet_stem, &sheet_png, &sheet_json)?;

  // Legacy path used by island spawner: horizontal strip `{name}-8.png`.
    let strip_png = out_dir.join(format!("{stem}.png"));
    write_horizontal_strip(&ase, &frames, &strip_png)?;

    println!("  {}", sheet_png.display());
    println!("  {}", strip_png.display());
    Ok(2)
}

fn write_horizontal_strip(ase: &AsepriteFile, frame_indices: &[u32], png_path: &Path) -> Result<(), String> {
    let first = ase.frame(frame_indices[0]).image();
    let fw = first.width();
    let fh = first.height();
    let sheet_w = fw * frame_indices.len() as u32;
    let mut sheet = RgbaImage::new(sheet_w, fh);
    for (i, &frame_idx) in frame_indices.iter().enumerate() {
        let img = ase.frame(frame_idx).image();
        imageops::overlay(&mut sheet, &img, i as i64 * fw as i64, 0);
    }
    sheet
        .save(png_path)
        .map_err(|e| format!("save png {}: {e}", png_path.display()))?;
    Ok(())
}

fn write_sheet(
    ase: &AsepriteFile,
    frame_indices: &[u32],
    stem: &str,
    png_path: &Path,
    json_path: &Path,
) -> Result<(), String> {
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
        "version": "export_flora",
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
