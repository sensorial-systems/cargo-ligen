use std::path::{Path, PathBuf};
use cargo_toml::Manifest;

pub fn find_workspace_dir(path: &Path) -> Option<PathBuf> {
    path
        .ancestors()
        .find_map(|ancestor| {
            Manifest::from_path(ancestor.join("Cargo.toml"))
                .ok()
                .and_then(|manifest| manifest.workspace)
                .map(|_| ancestor.to_path_buf())
        })
}
