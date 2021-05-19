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

pub fn to_library_name_convention(name: &String) -> String {
    #[cfg(target_family = "windows")]
    let name = format!("{}.lib", name);

    #[cfg(target_family = "unix")]
    let name = format!("lib{}.a", name);

    name
}