use clap::Parser;

use chatr::*;

use fulcrum;

/// Fulcrum determines if a build is affected by balance changes
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// build chat code
    build: String,

    /// weapons
    weapons: Vec<String>
}

fn main() {
    let args = Args::parse();
}
