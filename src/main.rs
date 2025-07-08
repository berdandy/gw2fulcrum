use clap::Parser;
use std::fs;

use chatr;
use gw2fulcrum;

/// Fulcrum determines if a build is affected by balance changes
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// build markdown filenames
    builds: Vec<String>
}

fn main() {
    let args = Args::parse();

    for file_path in args.builds {
        if let Ok(contents) = fs::read_to_string(file_path.clone()) {
            println!("READING: {}", file_path.clone());
            let build = chatr::BuildTemplate::parse_string(&contents).expect("Error parsing build template");
            let gear = chatr::GearTemplate::parse_string(&contents).expect("Error parsing gear");

            let dep = gw2fulcrum::BuildDependencies::from_templates(&gear, &build);
            println!("{:?}", dep);
        }
    }
}
