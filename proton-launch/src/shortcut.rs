use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    name: String,
    exe_path: PathBuf,
    working_dir: PathBuf,
    compat_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcuts {
    shortcuts: Vec<Shortcut>,
}

