use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Represents a specific item found within a cache directory.
#[derive(Clone)]
pub struct SubItem {
    /// Unique identifier key for the category (e.g. "hf", "ollama").
    pub key: String,
    /// Category name of the item (e.g. Hugging Face Cache).
    pub category: String,
    /// Human-readable name of the item.
    pub name: String,
    /// Absolute path to the item on disk.
    pub path: PathBuf,
    /// Size of the item in bytes.
    pub size: u64,
}

/// Recursively calculates the total size of a file or directory.
pub fn get_folder_size(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

/// Helper function to discover direct children of a directory, calculating their sizes.
/// This resolves the DRY violation for generic directory layouts (like pip, cursor).
fn discover_directory_children(key: &str, category_name: &str, base_path: &Path) -> Vec<SubItem> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let size = get_folder_size(&path);
            items.push(SubItem {
                key: key.to_string(),
                category: category_name.to_string(),
                name,
                path,
                size,
            });
        }
    }
    items
}

/// Helper function to read an Ollama manifest JSON file and calculate the combined size
/// of the model configuration and its layers.
fn get_ollama_manifest_size(manifest_path: &Path) -> u64 {
    if let Ok(content) = fs::read_to_string(manifest_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let mut total_size = 0;
            if let Some(config_size) = json
                .get("config")
                .and_then(|c| c.get("size"))
                .and_then(|s| s.as_u64())
            {
                total_size += config_size;
            }
            if let Some(layers) = json.get("layers").and_then(|l| l.as_array()) {
                for layer in layers {
                    if let Some(size) = layer.get("size").and_then(|s| s.as_u64()) {
                        total_size += size;
                    }
                }
            }
            return total_size;
        }
    }
    0
}

/// Discovers specific cleanable items within a target cache location based on its key type.
pub fn discover_sub_items(key: &str, category_name: &str, base_path: &Path) -> Vec<SubItem> {
    let mut items = Vec::new();
    if !base_path.exists() {
        return items;
    }

    match key {
        "hf" => {
            if let Ok(entries) = fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("models--") {
                        let clean_name = name.trim_start_matches("models--").replace("--", "/");
                        let size = get_folder_size(&path);
                        items.push(SubItem {
                            key: key.to_string(),
                            category: category_name.to_string(),
                            name: clean_name,
                            path,
                            size,
                        });
                    }
                }
            }
        }
        "ollama" => {
            for entry in WalkDir::new(base_path).into_iter().flatten() {
                if entry.file_type().is_file() {
                    let path = entry.path().to_path_buf();
                    let rel_path = path.strip_prefix(base_path).unwrap_or(&path);
                    let name = rel_path.to_string_lossy().into_owned();
                    let size = get_ollama_manifest_size(&path);
                    items.push(SubItem {
                        key: key.to_string(),
                        category: category_name.to_string(),
                        name,
                        path,
                        size,
                    });
                }
            }
        }
        "torch" => {
            if let Ok(entries) = fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = entry.file_name().to_string_lossy().into_owned();
                        let size = path.metadata().map(|m| m.len()).unwrap_or(0);
                        items.push(SubItem {
                            key: key.to_string(),
                            category: category_name.to_string(),
                            name,
                            path,
                            size,
                        });
                    }
                }
            }
        }
        "cursor" | "pip" => {
            // Using the shared DRY helper function
            items = discover_directory_children(key, category_name, base_path);
        }
        _ => {}
    }

    items
}
