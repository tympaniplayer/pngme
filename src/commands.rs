use std::ffi::OsString;

use clap::{Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encode{
        #[arg(required(true))]
        file_path: Option<OsString>,
        #[arg(required(true))]
        chunk_type: String,
        #[arg(required(true))]
        message: String,
        #[arg(required(false))]
        output_file: Option<OsString>
    },
    Decode {
        #[arg(required(true))]
        file_path: Option<OsString>,
        #[arg(required(true))]
        chunk_type: String,
    },
    Remove {
        #[arg(required(true))]
        file_path: Option<OsString>,
        #[arg(required(true))]
        chunk_type: String,
    },
    Print {
        #[arg(required(true))]
        file_path: Option<OsString>,
    }
}