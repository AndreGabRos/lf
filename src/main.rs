use std::fs::{self};
use clap::Parser;
use textwrap::termwidth;

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

        if let Ok(name) = dir_file.file_name().into_string() {
            if !name.starts_with('.') || show_all {
                line_length = print_file(&name, data.is_dir(), line_length, terminal_width);
            }
        }
    }

    Ok(())
} 

fn print_file(file_name: &str, is_dir: bool, mut line_length: usize, terminal_width: usize) -> usize {
    let mut name_length = 0;
    if is_dir {
        name_length = file_name.len() + 4;
        if name_length + line_length > terminal_width {
            println!();
            line_length = 0;
        }
        print!("{}/   ", file_name);
    } else {
        name_length = file_name.len() + 3;
        if name_length + line_length > terminal_width {
            println!();
            line_length = 0;
        }
        print!("{}   ", file_name);
    }

    line_length += name_length;

    line_length
}
