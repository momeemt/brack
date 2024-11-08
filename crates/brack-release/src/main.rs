use std::fs::read_to_string;
use std::path::Path;
use toml_edit::{value, DocumentMut};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

mod semver;
use crate::semver::SemVer;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    sub_commands: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    Update { semver_kind: SemVerKind },
    DebugUpdate { version: String },
}

#[derive(Debug, Clone, ValueEnum)]
enum SemVerKind {
    Major,
    Minor,
    Patch,
}

fn get_current_version<P: AsRef<Path>>(path: P) -> Result<SemVer> {
    let file = read_to_string(path)?;
    let mut lines = file.lines();
    let version = SemVer::new_with_string(
        lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("No version found"))?,
    )?;
    Ok(version)
}

fn rewrite_version<P: AsRef<Path> + Copy>(path: P, version: &SemVer) -> Result<()> {
    let version = version.to_string();
    let toml_str = read_to_string(path)?;
    let mut toml = toml_str.parse::<DocumentMut>()?;
    toml["package"]["version"] = value(version);
    let toml_str = toml.to_string();
    std::fs::write(path, toml_str)?;
    Ok(())
}

fn rewrite_all_cargo_toml(next_version: &SemVer) -> Result<()> {
    let cargo_toml_paths = [
        "Cargo.toml",
        "crates/brack-codegen/Cargo.toml",
        "crates/brack-expander/Cargo.toml",
        "crates/brack-language-server/Cargo.toml",
        "crates/brack-parser/Cargo.toml",
        "crates/brack-plugin/Cargo.toml",
        "crates/brack-plugin/Cargo.toml",
        "crates/brack-project-manager/Cargo.toml",
        "crates/brack-tokenizer/Cargo.toml",
        "crates/brack-transformer/Cargo.toml",
    ];
    for path in cargo_toml_paths.iter() {
        rewrite_version(path, &next_version)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.sub_commands {
        SubCommands::Update { semver_kind } => {
            let current_version = get_current_version("VERSION")?;
            println!("Current version: {}", current_version);
            let next_version = match semver_kind {
                SemVerKind::Major => current_version.next_major(),
                SemVerKind::Minor => current_version.next_minor(),
                SemVerKind::Patch => current_version.next_patch(),
            };
            println!("Next version: {}", next_version);
            rewrite_all_cargo_toml(&next_version)?;
        },
        SubCommands::DebugUpdate { version } => {
            let next_version = SemVer::new_with_string(&version)?;
            rewrite_all_cargo_toml(&next_version)?;
        },
    }
    Ok(())
}
