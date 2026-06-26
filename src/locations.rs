use std::path::PathBuf;

/// Represents a configured cache location on the local system.
pub struct CacheLocation {
    /// Unique identifier key for CLI selection (e.g. "hf", "pip").
    pub key: &'static str,
    /// Human-readable display name.
    pub name: &'static str,
    /// Absolute path to the cache directory.
    pub path: PathBuf,
}

/// Retrieves all predefined cache locations with OS-specific path configurations.
pub fn get_scan_locations() -> Vec<CacheLocation> {
    // Resolve home directory using the `dirs` crate, falling back to HOME env var.
    let home = dirs::home_dir()
        .unwrap_or_else(|| std::env::var("HOME").map(PathBuf::from).unwrap_or_default());

    // Detect target OS for pip cache path
    let pip_path = if cfg!(target_os = "macos") {
        home.join("Library/Caches/pip")
    } else {
        home.join(".cache/pip")
    };

    // Detect target OS for Cursor editor logs path
    let cursor_path = if cfg!(target_os = "macos") {
        home.join("Library/Application Support/Cursor/logs")
    } else {
        home.join(".config/Cursor/logs")
    };

    vec![
        CacheLocation {
            key: "hf",
            name: "Hugging Face Cache",
            path: home.join(".cache/huggingface/hub"),
        },
        CacheLocation {
            key: "ollama",
            name: "Ollama Models",
            path: home.join(".ollama/models/manifests/registry.ollama.ai/library"),
        },
        CacheLocation {
            key: "pip",
            name: "Pip Package Cache",
            path: pip_path,
        },
        CacheLocation {
            key: "torch",
            name: "PyTorch Cache",
            path: home.join(".cache/torch/hub/checkpoints"),
        },
        CacheLocation {
            key: "cursor",
            name: "Cursor Editor Logs",
            path: cursor_path,
        },
    ]
}
