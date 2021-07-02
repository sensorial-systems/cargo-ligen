use crate::{find_workspace_dir, Error, RawArguments};
use cargo_toml::Manifest;
use ligen::generator::{Arguments, BuildType};
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
        let workspace_member_package_id = raw_arguments
            .find_pair("--package")
            .or_else(|| raw_arguments.find_pair("-p"));

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

        match Manifest::from_path(&manifest_path) {
            Ok(manifest) => match manifest.package {
                Some(package) => Ok(Self {
                    crate_name: package.name,
                    build_type,
                    target_dir,
                    manifest_path,
                    workspace_path,
                    workspace_member_package_id,
                }),
                None => match manifest.workspace {
                    Some(_) => Ok(Self {
                        crate_name: "workspace".into(),
                        build_type,
                        target_dir,
                        manifest_path,
                        workspace_path,
                        workspace_member_package_id,
                    }),
                    None => Err(Error::String(
                        "Couldn't find package/workspace information on Cargo.toml".to_string(),
                    )),
                },
            },
            Err(error) => Err(Self::Error::from(error)),
        }
    }
}
