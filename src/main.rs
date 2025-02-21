use std::{error::Error, path::PathBuf};

use clap::Parser;
use engine::EngineOptions;
use log::info;
use simple_logger::SimpleLogger;

mod dependency;
mod engine;
mod pyproject;
mod uv;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List of directories to ignore, we ignore .venv and .git by default
    #[arg(long)]
    exclude_dirs: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()
        .unwrap();

    let args = Args::parse();
    let options = EngineOptions {
        exclude_dirs: args.exclude_dirs,
    };
    let pyproject_path = PathBuf::from("./pyproject.toml");
    let pyproject = pyproject::read(&pyproject_path).unwrap();
    let engine = engine::DetectEngine::new(pyproject.clone(), options);
    let deps = engine.detect_dependencies(PathBuf::from(".")).unwrap();
    if deps.is_empty() {
        info!("No dependencies detected, nothing to do");
        return Ok(());
    }

    match pyproject::write(&pyproject_path, pyproject, deps) {
        Ok(_) => (),
        Err(e) => panic!("Failed to write deps to pyproject.toml: {:?}", e),
    };
    match uv::sync() {
        Ok(_) => (),
        Err(e) => panic!("Failed to run uv sync: {:?}", e),
    };
    match uv::lock() {
        Ok(_) => (),
        Err(e) => panic!("Failed to update lockfile: {:?}", e),
    };
    Ok(())
}
