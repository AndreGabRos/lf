use std::fs::{self, DirBuilder};
use clap::Parser;
use textwrap::{termwidth, fill};

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
    let terminal_width = termwidth();

    // Variável auxiliar q verifica o len restante de uma linha do terminal
    let mut line_length = 0; 

    for entry in fs::read_dir(dir)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;
        let mut name_length = 0;

        if let Ok(name) = dir_file.file_name().into_string() {
            if !name.starts_with('.') || show_all {
                if data.is_dir() {
                    name_length = name.len() + 4;
                    if name_length + line_length > terminal_width {
                        println!();
                        line_length = 0;
                    }
                    print!("{}/   ", name);
                } else {
                    name_length = name.len() + 3;
                    if name_length + line_length > terminal_width {
                        println!();
                        line_length = 0;
                    }
                    print!("{}   ", name);
                }
            }
        }
        line_length += name_length;
    }

    Ok(())
} 
