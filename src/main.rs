use std::{fs::{self, Metadata}, os::unix::prelude::{PermissionsExt, MetadataExt}, ops::AddAssign};
use clap::Parser;
use textwrap::termwidth;
use users::get_user_by_uid;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format::{self, TableFormat}};

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

fn get_table_format() -> TableFormat {
    format::FormatBuilder::new()
        .column_separator('\t')
        .borders(' ')
        .build()
}

fn list_files(dir: &str, show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let terminal_width = termwidth();

    let mut maior_len: usize = 0;
    let mut name_files: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;

        if let Ok(mut name) = dir_file.file_name().into_string() {
            if !name.starts_with('.') || show_all {
                if data.is_dir() {
                    name.push('/');
                }                

                if name.len() > maior_len {
                    maior_len = name.len();
                }

                name_files.push(name);
            }
        }
    }
    let collums = terminal_width / (maior_len + 7);
    println!("{collums}");
    if collums == 0 {
        for name in name_files {
            println!("{name}");
        }
    } else {
        print_files_in_table(name_files, collums);
    }

    Ok(())
} 

fn print_files_in_table(files_name: Vec<String>, collums: usize) {
    let mut table = Table::new();
    table.set_format(get_table_format());

    for names in files_name.chunks(collums) {
        let mut cells: Vec<Cell> = Vec::new();
        for name in names {
            cells.push(Cell::new(name));
        }
        table.add_row(Row::new(cells));
    }
    let table_text = table.to_string();

    println!("{}", table_text);
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

        let user_uid = data.uid();

        let user = get_user_by_uid(user_uid).unwrap();
        let user_name = user.name().to_str().unwrap();

        if let Ok(file_name) = dir_file.file_name().into_string() {
            if !file_name.starts_with('.') || show_all {
                println!("{} {} {}", perm, user_name, file_name);
            }
        }
    }

    Ok(())
}
