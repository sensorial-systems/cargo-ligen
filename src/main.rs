use cargo_toml::Manifest;
use std::{
    env,
    fs::copy,
    path::Path,
    process::{Command, Stdio},
};

fn main() {
    let mut args = vec![String::from("+nightly"), String::from("build")];
    let env_args: Vec<String> = env::args().collect();
    args.extend_from_slice(env_args.get(2..env_args.len()).unwrap());
    if Path::new("./Cargo.toml").exists() {
        let output = Command::new("cargo")
            .args(args.as_slice())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        if let Ok(_) = output {
            let name = Manifest::from_path("./Cargo.toml")
                .expect("Failed to parse Cargo.toml")
                .package
                .expect("Failed to parse package section from Cargo.toml")
                .name;

            copy(
                format!(
                    "./target/{}/lib{}.a",
                    if env_args
                        .into_iter()
                        .any(|arg| arg == String::from("--release"))
                    {
                        "release"
                    } else {
                        "debug"
                    },
                    name
                ),
                format!("./target/ligen/{0}/lib/lib{0}.a", name),
            )
            .expect("Failed to copy lib");
        } else {
            panic!("Current directory is not a Cargo project");
        }
    } else {
        panic!("Failed to run cargo build");
    }
}
