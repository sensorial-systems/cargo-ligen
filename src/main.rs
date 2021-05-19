use cargo_toml::Manifest;
use std::{
    env,
    fs::{copy, read_dir},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn find_workspace(path: &Path) -> Option<PathBuf> {
    let mut iter = path.ancestors();
    iter.next();
    while let Some(parent) = iter.next() {
        if let Ok(manifest) = Manifest::from_path(parent.join("Cargo.toml")) {
            if let Some(_worspace) = manifest.workspace {
                return Some(parent.to_path_buf());
            }
        }
    }
    return None;
}

fn main() {
    let original_path = env::current_dir().expect("Failed to fetch original directory");
    let mut manifest_path = PathBuf::new();
    if let Some(workspace_path) =
        find_workspace(&env::current_dir().expect("Failed to fetch current directory"))
    {
        manifest_path.push(workspace_path);
    } else {
        manifest_path.push("./")
    }

    let mut args = vec![String::from("+nightly"), String::from("build")];
    let env_args: Vec<String> = env::args().collect();
    args.extend_from_slice(env_args.get(2..env_args.len()).unwrap());
    if manifest_path.join("Cargo.toml").exists() {
        env::set_current_dir(&manifest_path)
            .expect("Failed to set current working directory to the manifest path");
        println!("current_dir: {:#?}", &env::current_dir());
        let output = Command::new("cargo")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        if let Ok(_) = output {
            let name = Manifest::from_path(original_path.join("Cargo.toml"))
                .expect("Failed to parse Cargo.toml")
                .package
                .expect("Failed to parse package section from Cargo.toml")
                .name;

            #[cfg(target_family = "windows")]
            let file_name = format!("{}.lib", name);

            #[cfg(target_family = "unix")]
            let file_name = format!("lib{}.a", name);

            let build_dir = manifest_path.join(format!(
                "target/{}/",
                if env_args
                    .into_iter()
                    .any(|arg| arg == String::from("--release"))
                {
                    "release"
                } else {
                    "debug"
                }
            ));

            let lib_paths = read_dir(build_dir).unwrap().filter_map(|entry| {
                if let Ok(dir_entry) = entry {
                    if let Some(extension) = dir_entry.path().extension() {
                        if extension == "a" {
                            Some(dir_entry.path())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            lib_paths.for_each(|path| {
                println!("{:#?}", path);
                copy(
                    path,
                    manifest_path.join(format!("target/ligen/{}/lib/{}", name, file_name)),
                )
                .expect("Failed to copy lib");
            });
        } else {
            panic!("Current directory is not a Cargo project");
        }
    } else {
        panic!("Cargo.toml wasn't found in the current directory.");
    }
}
