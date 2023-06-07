use clap::ValueEnum;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use strfmt::strfmt;
use toml;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: std::path::PathBuf,

    /// Choose desired output format
    #[arg(value_enum, default_value_t = PrintFormat::BASIC)]
    format: PrintFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PrintFormat {
    /// Prints output with the format: {name}:{version}
    BASIC,
    /// Prints output with the format: {icon} {name}:{version}
    FORMAL,
    /// Prints output with the format: {alt_icon} {name}:{version}
    PLAYFUL,
}

#[derive(Serialize)]
struct ProjectDetails {
    icon: String,
    alt_icon: String,
    name: String,
    version: String,
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
            Some(details) => println!("{}", format_output(details, args.format)),
            None => (),
        }
    }
}

fn format_output(details: ProjectDetails, print_format: PrintFormat) -> String {
    let format = match print_format {
        PrintFormat::BASIC => "{name}:{version}",
        PrintFormat::FORMAL => "{icon} {name}:{version}",
        PrintFormat::PLAYFUL => "{alt_icon} {name}:{version}",
    };
    let serialized = serde_json::to_value(details).unwrap();
    let hashmap: HashMap<String, String> = serde_json::from_value(serialized).unwrap();

    return strfmt(format, &hashmap).unwrap();
}

fn get_project_name(path: PathBuf) -> Option<ProjectDetails> {
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

fn read_cargo_toml(path: &PathBuf) -> Option<ProjectDetails> {
    let contents = unwrap_or_return_none!(fs::read_to_string(&path));
    let data: CargoData = unwrap_or_return_none!(toml::from_str(&contents));
    return Some(ProjectDetails {
        icon: "\u{e7a8}".to_owned(),
        alt_icon: "🦀".to_owned(),
        name: data.package.name,
        version: data.package.version,
    });
}

fn read_poetry_toml(path: &PathBuf) -> Option<ProjectDetails> {
    let contents = unwrap_or_return_none!(fs::read_to_string(&path));
    let data: PoetryToolData = unwrap_or_return_none!(toml::from_str(&contents));
    return Some(ProjectDetails {
        icon: "\u{e73c}".to_owned(),
        alt_icon: "🐍".to_owned(),
        name: data.tool.poetry.name,
        version: data.tool.poetry.version,
    });
}
