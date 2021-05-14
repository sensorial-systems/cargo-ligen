use cargo_toml::Manifest;
use std::{env, fs::copy, path::Path, process::Command};

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    if Path::new("./Cargo.toml").exists() {
        env::args().into_iter().for_each(|arg| {
            if let "--clean" = arg.as_str() {
                let pb = ProgressBar::new_spinner();
                pb.enable_steady_tick(120);
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .tick_strings(&[
                            "▹▹▹▹▹",
                            "▸▹▹▹▹",
                            "▹▸▹▹▹",
                            "▹▹▸▹▹",
                            "▹▹▹▸▹",
                            "▹▹▹▹▸",
                            "▪▪▪▪▪",
                        ])
                        .template("{spinner:.blue} {msg}"),
                );
                pb.set_message("Cleaning...");
                
                // FIXME: Why is this needed?
                let out = Command::new("cargo").arg("clean").output();

                if let Ok(_) = out {
                    pb.finish_with_message("Done");
                } else {
                    panic!("Failed to run cargo clean")
                }
            }
        });

        //TODO: reuse the progressbar code

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(120);
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ])
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message("Building...");

        let output = Command::new("cargo").arg("build").output();

        if let Ok(_) = output {
            pb.finish_with_message("Done");
        } else {
            panic!("Failed to run cargo build")
        }

        //TODO: Handle output

        let name = Manifest::from_path("./Cargo.toml")
            .expect("Failed to parse Cargo.toml")
            .package
            .expect("Failed to parse package section from Cargo.toml")
            .name;

        copy(
            format!("./target/debug/lib{}.a", name),
            format!("./target/ligen/{0}/lib/lib{0}.a", name),
        )
        .expect("Failed to copy lib");
    } else {
        panic!("Current directory is not a Cargo project");
    }
}

//TODO: Create tests
