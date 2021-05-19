use crate::{Error, RawArguments, find_workspace_dir};
use std::path::PathBuf;
use std::convert::TryFrom;
use ligen_core::proc_macro::{Arguments, BuildType};

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
                    .unwrap_or(manifest_path.clone())
                    .join("target")
            });

        let workspace_path = workspace_dir.map(|path| path.join("Cargo.toml"));

        let build_type = raw_arguments
            .find("--release")
            .map(|_| BuildType::Release)
            .unwrap_or(BuildType::Debug);

        Ok(Self { build_type, target_dir, manifest_path, workspace_path })
    }
}