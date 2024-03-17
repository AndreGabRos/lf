use std::fs::{self};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool, // show all files

    #[clap(default_value = ".")]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Cli::parse();

    list_files(&cli_args.path, cli_args.all)?;

    Ok(())
}

fn list_files(dir: &str, show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let dir_files = entry?;
        let data = dir_files.metadata()?;
        if data.is_dir() {
            if let Ok(name) = dir_files.file_name().into_string() {
                if !name.starts_with('.') || show_all {
                    print!("{}/    ", name);
                }
            }
        } else if let Ok(name) = dir_files.file_name().into_string() {
            if !name.starts_with('.') || show_all {
                print!("{}   ", name);
            }
        }
    }

    Ok(())
} 
