use std::env::var_os;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use clap::CommandFactory;
use clap_mangen::Man;
use crate::cli::Args;

#[path = "src/cli.rs"] mod cli;

fn create_assets_folder() -> PathBuf {
    let out_dir = PathBuf::from(var_os("OUT_DIR").unwrap());

    // actually create directory "./target/assets"
    let mut path = out_dir.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    path.push("man1");
    fs::create_dir_all(&path).unwrap();

    path
}

fn build_man() -> std::io::Result<()> {
    let assets_folder = create_assets_folder();

    let file_path = assets_folder.join("git-auto-commit.1");
    let mut file = File::create(&file_path)?;

    Man::new(Args::command()).render(&mut file)
}

fn main() -> std::io::Result<()> {
    build_man()?;

    Ok(())
}
