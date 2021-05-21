use crate::{find_workspace_dir, Error, RawArguments};
use cargo_toml::Manifest;
use ligen_core::proc_macro::{Arguments, BuildType};
use std::convert::TryFrom;
use std::path::PathBuf;

impl TryFrom<RawArguments> for Arguments {
    type Error = Error;
    fn try_from(raw_arguments: RawArguments) -> Result<Self, Self::Error> {
        let current_dir = std::env::current_dir()?;
        let manifest_path = raw_arguments
            .find_pair("--manifest-path")
            .map(|path| PathBuf::from(path))
            .unwrap_or(current_dir.join("Cargo.toml"));

        let workspace_dir = find_workspace_dir(&manifest_path);

        let target_dir = raw_arguments
            .find_pair("--target-dir")
            .map(|path| PathBuf::from(path))
            .unwrap_or_else(|| {
                workspace_dir
                    .clone()
                    .unwrap_or(
                        manifest_path
                            .clone()
                            .parent()
                            .expect("Failed to parse path of the manifest file")
                            .into(),
                    )
                    .join("target")
            });

        let workspace_path = workspace_dir.map(|path| path.join("Cargo.toml"));

        let build_type = raw_arguments
            .find("--release")
            .map(|_| BuildType::Release)
            .unwrap_or(BuildType::Debug);

        let name = Manifest::from_path(&manifest_path)
            .expect(&format!("Failed to parse {}", manifest_path.display()))
            .package
            .expect("Failed to parse package section from Cargo.toml")
            .name;

        Ok(Self {
            name,
            build_type,
            target_dir,
            manifest_path,
            workspace_path,
        })
    }
}
