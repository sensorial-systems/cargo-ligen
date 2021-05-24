mod environment;
mod error;
mod utils;
pub use environment::*;
pub use error::Error;
pub use utils::*;

use cargo_toml::Manifest;
use std::path::PathBuf;
use std::{env, fs::create_dir_all};
use std::{
    fs::copy,
    process::{Command, Stdio},
};

fn main() {
    let mut environment = Environment::parse().expect("Couldn't parse environment variables.");
    if let Some(workspace_dir) =
        find_workspace_dir(&env::current_dir().expect("Couldn't get current dir"))
    {
        std::env::set_current_dir(&workspace_dir).expect("Couldn't change directory");
        environment = Environment::parse().expect("Couldn't parse environment variables.");

        let manifest = Manifest::from_path(workspace_dir.join("Cargo.toml"))
            .expect("Couldn't parse the workspace Cargo.toml manifest.");
        let workspace = manifest
            .workspace
            .expect("Couldn't get the workspace members.");

        for member in workspace.members {
            std::env::set_current_dir(workspace_dir.join(&member))
                .expect("Couldn't change directory");
            let mut member_env =
                Environment::parse().expect("Couldn't parse environment variables.");
            member_env.arguments.target_dir = workspace_dir.join("target/");
            build(&member_env).expect("Failed to build.");
        }
    } else {
        build(&environment).expect("Failed to build.");
    }

    if environment.arguments.workspace_path.as_ref() == Some(&environment.arguments.manifest_path) {
        let manifest = Manifest::from_path(&environment.arguments.manifest_path)
            .expect("Couldn't parse the workspace Cargo.toml manifest.");
        let workspace = manifest
            .workspace
            .expect("Couldn't get the workspace members.");
        let manifest_dir = environment
            .arguments
            .manifest_path
            .parent()
            .expect("Couldn't get manifest dir.");
        for member in workspace.members {
            let member_toml = manifest_dir.join(member).join("Cargo.toml");
            copy_crate_libraries(&environment, &member_toml).expect("Couldn't copy libraries.");
        }
    } else {
        copy_crate_libraries(&environment, &environment.arguments.manifest_path)
            .expect("Couldn't copy libraries.");
    }
}

pub fn build(environment: &Environment) -> Result<(), Error> {
    environment.arguments.to_env();

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

        let to_dir = to_path
            .parent()
            .expect(&format!("Couldn't get directory of {}", to_path.display()));

        create_dir_all(to_dir).expect(&format!("Couldn't create {}.", to_dir.display()));

        // TODO: Add a better mechanism to detect if the crate implements ligen and only copy the
        //  static library if so.
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
