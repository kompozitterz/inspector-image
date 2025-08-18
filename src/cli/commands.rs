use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="inspector-image", version="1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Map { image: String },
    Steg { image: String },
}
