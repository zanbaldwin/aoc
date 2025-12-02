#![allow(dead_code)]

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Error as IoError, ErrorKind, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    file: Option<String>,
    #[arg(short, long, default_value_t = true, conflicts_with = "quiet")]
    verbose: bool,
    #[arg(short, long, default_value_t = false, conflicts_with = "verbose")]
    quiet: bool,
}

/// Macro that creates an Input from CLI args, using package name as fallback
#[macro_export]
macro_rules! input {
    () => {{
        let input = match $crate::Input::from_cli() {
            Ok(input) => Ok(input),
            Err(e) if matches!(e.kind(), std::io::ErrorKind::InvalidInput) => {
                // The whole reason for the macro: use the package name of the
                // caller, and not the package name of this common library.
                $crate::Input::from_search(Some(env!("CARGO_PKG_NAME")))
            },
            Err(e) => Err(e),
        };
        if let Ok(input) = &input {
            eprintln!("Using input file: {}", input.filename());
        }
        input
    }};
}

pub struct Input {
    file: File,
    path: PathBuf,
}

impl Input {
    pub fn from_cli() -> Result<Self, IoError> {
        let args = Args::parse();
        match &args.file {
            None => Err(IoError::new(ErrorKind::InvalidInput, "No input file specified")),
            Some(filepath) => match File::open(filepath) {
                Ok(file) => {
                    let path = std::fs::canonicalize(filepath).unwrap_or_else(|_| PathBuf::from(filepath));
                    Ok(Self { file, path })
                },
                Err(e) => {
                    let message = format!("Could not find specified input file `{}`", filepath);
                    Err(IoError::new(e.kind(), message))
                },
            },
        }
    }

    pub fn from_search(package: Option<&str>) -> Result<Self, IoError> {
        let package = package.unwrap_or(env!("CARGO_PKG_NAME"));
        let paths = [
            "input.txt".to_string(),
            format!("inputs/{package}.txt"),
            format!("{package}.txt"),
            concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt").to_string(),
            format!(concat!(env!("CARGO_MANIFEST_DIR"), "/../inputs/{}.txt"), package),
        ];

        for path in &paths {
            if let Ok(file) = File::open(path) {
                let path = std::fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
                return Ok(Self { file, path });
            }
        }
        Err(IoError::new(
            ErrorKind::NotFound,
            "Could not find `input.txt` or `<package-name>.txt` in searched locations",
        ))
    }

    pub fn filename(&self) -> &str {
        self.path.to_str().unwrap_or("<invalid>")
    }

    pub fn into_buffer(self) -> BufReader<File> {
        BufReader::new(self.file)
    }

    pub fn into_string(mut self) -> String {
        let size = self.file.metadata().map(|m| m.len() as usize).unwrap_or(0);
        let mut buffer = String::with_capacity(size);
        self.file.read_to_string(&mut buffer).expect("Failed to read contents of input file");
        buffer
    }
}
