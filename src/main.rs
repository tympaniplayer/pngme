use std::{ffi::OsString, fs::File, io::Write, str::FromStr};

use args::Args;
use chunk::Chunk;
use chunk_type::ChunkType;
use clap::Parser;
use png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.command {
        commands::Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => {
            let result = encode(file_path, chunk_type, message, output_file);
            match result {
                Ok(_) => (),
                Err(error) => panic!("Unable to encode {}", error),
            }
        }
        commands::Commands::Decode {
            file_path,
            chunk_type,
        } => todo!(),
        commands::Commands::Remove {
            file_path,
            chunk_type,
        } => todo!(),
        commands::Commands::Print { file_path } => todo!(),
    }

    Ok(())
}

fn encode(
    file_path: Option<OsString>,
    chunk_type: String,
    message: String,
    output_file: Option<OsString>,
) -> Result<()> {
    let mut png: Png;
    let matched_path: OsString;
    match file_path {
        Some(path) => {
            png = {
                let parse_result = Png::from_file(&path);
                match parse_result {
                    Ok(parsed) => parsed,
                    Err(error) => panic!("Error parsing png {}", error),
                }
            };
            matched_path = path;
        }
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid filename",
            )));
        }
    }
    let chunk: Chunk;
    let chunk_result = png.chunk_by_type(&chunk_type);
    match chunk_result {
        Some(_) => {
            png.remove_chunk(&chunk_type)?;
        }
        None => (),
    }
    let chunk_type_object = ChunkType::from_str(&chunk_type).unwrap();
    chunk = Chunk::new(chunk_type_object, message.into_bytes());
    png.append_chunk(chunk);

    match output_file {
        Some(path) => {
            write_png(&png, &path);
        }
        None => {
            write_png(&png, &matched_path);
        }
    }

    Ok(())
}

fn write_png(png: &Png, path: &OsString) {
    let mut file = File::create(path).unwrap();
    let bytes = png.as_bytes();
    file.write_all(&bytes).unwrap();
}
