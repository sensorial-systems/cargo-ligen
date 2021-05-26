mod environment;
mod error;
mod utils;
pub use environment::*;
pub use error::Error;
pub use utils::*;

use cargo_toml::Manifest;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{
    fs::copy,
    process::{Command, Stdio},
};

fn main() {
    let environment = Environment::parse().expect("Couldn't parse environment variables.");
    let arguments = &environment.arguments;
    let manifest =
        Manifest::from_path(&arguments.manifest_path).expect("Couldn't parse Cargo.toml manifest.");
    if let Some(workspace) = manifest.workspace {
        let member_package_ids = collect_members_package_ids(&environment, workspace.members);
        let members = arguments
            .workspace_member_package_id
            .clone()
            .map(|package_id| {
                vec![member_package_ids
                    .clone()
                    .into_iter()
                    .find(|(_, package)| package == &package_id)
                    .expect("Package not found")]
            }) // We only build the selected workspace member.
            .unwrap_or_else(|| member_package_ids); // We build all the workspace members.
        for member in members {
            build_workspace_member(&environment, member).expect("Couldn't build workspace member.")
        }
    } else {
        build(&environment).expect("Failed to build.");
        copy_crate_libraries(&environment, &environment.arguments.manifest_path)
            .expect("Couldn't copy libraries.");
    }
}

pub fn collect_members_package_ids(
    environment: &Environment,
    members: Vec<String>,
) -> Vec<(String, String)> {
    let manifest_dir = environment
        .arguments
        .manifest_path
        .parent()
        .expect("Couldn't get manifest dir.");

    members
        .clone()
        .into_iter()
        .zip(
            members
                .iter()
                .filter_map(|member| {
                    let cargo_path = manifest_dir.join(member).join("Cargo.toml");
                    Manifest::from_path(cargo_path)
                        .ok()
                        .and_then(|manifest| manifest.package)
                })
                .map(|package| package.name),
        )
        .collect()
}

pub fn build_workspace_member(
    environment: &Environment,
    member: (String, String),
) -> Result<(), Error> {
    let manifest_dir = environment
        .arguments
        .manifest_path
        .parent()
        .expect("Couldn't get manifest dir.");
    let member_toml = manifest_dir.join(member.clone().0).join("Cargo.toml");
    let mut member_env = environment.clone();
    member_env
        .raw_arguments
        .values
        .append(&mut vec!["--package".to_string(), member.clone().1]);
    member_env.arguments.crate_name = member.clone().1;
    build(&member_env)?;
    copy_crate_libraries(&member_env, &member_toml)
}

pub fn build(environment: &Environment) -> Result<(), Error> {
    environment.arguments.to_env();

    std::env::set_var("RUSTFLAGS", "--cfg cargo_ligen");
    let output = Command::new("cargo")
        .arg("+nightly")
        .arg("build")
        .args(&environment.raw_arguments.values)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Couldn't run cargo build.");

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::ExecutionFailure(
            output
                .status
                .code()
                .expect("Couldn't get execution status code."),
        ))
    }
}

fn copy_crate_libraries(environment: &Environment, cargo_toml: &PathBuf) -> Result<(), Error> {
    if cargo_toml.exists() {
        let manifest = Manifest::from_path(cargo_toml)
            .expect(&format!("Failed to parse {}", cargo_toml.display()));

        let name = manifest
            .package
            .expect("Failed to parse package section from Cargo.toml")
            .name;

        let file_name = to_library_name_convention(&name);

        let from_path = environment
            .arguments
            .target_dir
            .join(environment.arguments.build_type.to_string().to_lowercase())
            .join(&file_name);

        let to_path = environment
            .arguments
            .target_dir
            .join("ligen")
            .join(&name)
            .join("lib")
            .join(file_name);

        if from_path.exists() {
            copy(&from_path, &to_path).expect(&format!(
                "Failed to copy file from {:?} to {:?}",
                from_path, to_path
            ));
        }

        Ok(())
    } else {
        Err(Error::ExecutionFailure(-1))
    }
}
