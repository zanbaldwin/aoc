#![allow(dead_code)]

use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Error as IoError, Read};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "input.txt")]
    file: String,
}

pub struct Input {
    file: File,
}
impl Input {
    pub fn from_cli() -> Result<Self, IoError> {
        let args = Args::parse();
        let file = File::open(&args.file)?;
        Ok(Self { file })
    }

    pub fn into_buffer(self) -> BufReader<File> {
        BufReader::new(self.file)
    }

    pub fn into_string(mut self) -> String {
        let size = self.file.metadata().map(|m| m.len() as usize).unwrap_or(0);
        let mut buffer = String::with_capacity(size);
        self.file.read_to_string(&mut buffer).expect("Failed to read input");
        buffer
    }
}
