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
    invert: bool,

    /// update note file, in the format of: "(SKILL_OR_TRAIT_NAME): (NOTES)"
    update_path: String,

    /// build markdown filenames
    builds: Vec<String>,
}

fn main() {
    let args = Args::parse();
    
    if args.verbose {
        println!("UPDATE: {}", args.update_path);
    }
    let update = match fs::read_to_string(args.update_path.clone()) {
        Ok(update_str) => BalanceUpdate::parse_notes(&update_str),
        _ => BalanceUpdate::default(),
    };

    for file_path in &args.builds {
        if let Ok(contents) = fs::read_to_string(file_path.clone()) {
            let build = chatr::BuildTemplate::parse_string(&contents).expect("Error parsing build template");
            let gear = chatr::GearTemplate::parse_string(&contents).expect("Error parsing gear");

            let dep = BuildDependencies::from_templates(&gear, &build);
            if let Some(notes) = update.affects(&dep) {
                if args.verbose || args.builds.len() == 1 {
                    println!("{} was changed: \"{}\"", file_path.clone(), notes);
                } else if ! args.invert {
                    println!("{}", file_path.clone());
                }
            } else {
                if args.verbose || args.builds.len() == 1 {
                    println!("{} was unchanged", file_path.clone());
                } else if args.invert {
                    println!("{}", file_path.clone());
                }
            }
        }
    }
}
