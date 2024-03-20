use std::{fs::{self, Metadata}, os::unix::prelude::PermissionsExt};
use clap::Parser;
use textwrap::termwidth;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    all: bool, // show all files
    #[arg(short, long)]
    long: bool,

    #[clap(default_value = ".")]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args = Cli::parse();
    if cli_args.long {
        list_file_with_metadata(&cli_args.path, cli_args.all)?;

    } else {
        list_files(&cli_args.path, cli_args.all)?;
    }

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

fn get_file_mode(metadata: &Metadata) -> u32 {
    let permissions = metadata.permissions();
    permissions.mode()
    
}

fn turn_mode_into_readable_perm(mode: u32) -> String {
    let perm_bin = format!("{:b}", mode);
    let len = perm_bin.len();
    let p = &perm_bin[len-9..];
    let perm_chars = p.chars();
    let mut perm = String::new();
    let mut count = 1;

    if len == 16 {
        perm.push('.');
    } else {
        perm.push('d');
    }

    for i in perm_chars {
        if i == '1' {
            if count % 3 == 0 {
                perm.push('x');
            } else if count % 2 == 0 {
                perm.push('w');
            } else {
                perm.push('r');
            }
        }
        else {
            perm.push('-');
        }
        count += 1;
    }

    perm
}

fn list_file_with_metadata(path: &str, show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;
        let mode = get_file_mode(&data);
        let perm = turn_mode_into_readable_perm(mode);
        if let Ok(name) = dir_file.file_name().into_string() {
            if !name.starts_with('.') || show_all {
                println!("{} {}", perm, name);
            }
        }
    }

    Ok(())
}
