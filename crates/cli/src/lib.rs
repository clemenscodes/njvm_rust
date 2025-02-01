use clap::{command, Parser};

/// Command-line arguments for the NinjaVM
#[derive(Parser, Debug)]
#[command(name = "njvm", version, about)]
struct Args {
    /// Enables debug mode
    #[arg(long)]
    debug: bool,

    /// The file to execute
    file: Option<String>,
}
