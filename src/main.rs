use clap::Parser;
use std::fs;

use chatr;
use gw2fulcrum::{BuildDependencies, BalanceUpdate};

/// Fulcrum determines if a build is affected by balance changes
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    /// update note file, in the format of: "(SKILL_OR_TRAIT_NAME): (NOTES)"
    update_path: Option<String>,

    /// build markdown filenames
    builds: Vec<String>,
}

fn main() {
    let args = Args::parse();
    
    let mut update = BalanceUpdate::default();
    if let Some(update_filepath) = &args.update_path {
        if let Ok(update_str) = fs::read_to_string(update_filepath.clone()) {
            update = BalanceUpdate::parse_notes(&update_str);
        }
    }

    for file_path in args.builds {
        if let Ok(contents) = fs::read_to_string(file_path.clone()) {
            if args.verbose {
                println!("READING: {}", file_path.clone());
            }

            let build = chatr::BuildTemplate::parse_string(&contents).expect("Error parsing build template");
            let gear = chatr::GearTemplate::parse_string(&contents).expect("Error parsing gear");

            let dep = BuildDependencies::from_templates(&gear, &build);

            if args.verbose {
                println!("{:?} compare with {:?}", dep, update);
            }
        }
    }
}
