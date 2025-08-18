use clap::{Parser, Subcommand};

#[derive(Debug)]
#[command(name="inspector-image", version="1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Map { image: String},
    Steg { image: String},
}