use clap::{Parser, command};

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(about = "Encode and decode messages into a PNG", long_about = None)]
pub struct Args {
    #[command(subcommand)]
     pub command: Commands
}