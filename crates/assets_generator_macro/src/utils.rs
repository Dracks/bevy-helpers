use std::fs;
use std::path::Path;

use stringcase::Caser;

pub fn collect_assets(dir: &Path) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut assets = Vec::new();
    collect_assets_recursive(dir, dir.to_str().unwrap_or_default(), &mut assets)?;
    Ok(assets)
}

fn collect_assets_recursive(
    current_dir: &Path,
    base_dir: &str,
    assets: &mut Vec<(String, String)>,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively process subdirectories
            collect_assets_recursive(&path, base_dir, assets)?;
        } else if path.is_file() {
            if let Some(_) = path.file_stem() {
                // Get relative path from base assets directory
                let relative_path = path
                    .strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/"); // Ensure forward slashes for consistency

                // Convert path to a valid Rust enum variant name (includes path to avoid collisions)
                let enum_name = sanitize_enum_name(&relative_path);

                assets.push((enum_name, relative_path.to_string()));
            }
        }
    }
    Ok(())
}

fn sanitize_enum_name(relative_path: &str) -> String {
    // Use the full relative path to avoid collisions
    let path_without_extension = if let Some(dot_pos) = relative_path.rfind('.') {
        &relative_path[..dot_pos]
    } else {
        relative_path
    };


    // Convert path separators and special characters to underscores
    let sanitized = path_without_extension
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let pascal_case = sanitized.to_pascal_case();

    // Ensure the name doesn't start with a number
    if pascal_case.chars().next().map_or(false, |c| c.is_numeric()) {
        format!("Asset{}", pascal_case)
    } else if pascal_case.is_empty() {
        "UnknownAsset".to_string()
    } else {
        pascal_case
    }
}
