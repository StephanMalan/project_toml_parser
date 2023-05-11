use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Deserialize)]
struct CargoData {
    package: Config,
}

#[derive(Deserialize)]
struct PoetryToolData {
    tool: PoetryData,
}

#[derive(Deserialize)]
struct PoetryData {
    poetry: Config,
}

#[derive(Deserialize)]
struct Config {
    name: String,
    version: String,
}

macro_rules! unwrap_or_return_none {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return None,
        }
    };
}

fn main() {
    let args = Cli::parse();
    let full_path = fs::canonicalize(&args.path);
    if full_path.is_ok() {
        match get_project_name(full_path.unwrap()) {
            Some(name) => println!("{}", name),
            None => (),
        }
    }
}

fn get_project_name(path: PathBuf) -> Option<String> {
    let content = unwrap_or_return_none!(fs::read_dir(path.clone()));
    for entry in content {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = path.file_name()?;
            match file_name.to_str()? {
                "Cargo.toml" => return read_cargo_toml(&path),
                "pyproject.toml" => return read_poetry_toml(&path),
                _ => (),
            };
        }
    }
    let parent = path.parent();
    if parent.is_none() {
        return None;
    }
    return get_project_name(path.parent().unwrap().to_path_buf());
}

fn read_cargo_toml(path: &PathBuf) -> Option<String> {
    let contents = unwrap_or_return_none!(fs::read_to_string(&path));
    let data: CargoData = unwrap_or_return_none!(toml::from_str(&contents));
    return Some(data.package.name + &":".to_owned() + &data.package.version.to_owned());
}

fn read_poetry_toml(path: &PathBuf) -> Option<String> {
    let contents = unwrap_or_return_none!(fs::read_to_string(&path));
    let data: PoetryToolData = unwrap_or_return_none!(toml::from_str(&contents));
    return Some(data.tool.poetry.name + &":".to_owned() + &data.tool.poetry.version.to_owned());
}
