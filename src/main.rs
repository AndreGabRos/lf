use std::{fs::{self, Metadata}, os::unix::prelude::{PermissionsExt, MetadataExt}};
use clap::Parser;
use textwrap::termwidth;
use users::get_user_by_uid;
use chrono::{Local, TimeZone};
use ansiterm::Color::*;

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
    // if cli_args.long {
    //     list_file_with_metadata(&cli_args.path, cli_args.all)?;

    // } else {
    //     list_files(&cli_args.path, cli_args.all)?;
    // }
    let mut v = Vec::new();
    v = get_file_names(&cli_args.path, cli_args.all)?;
    print_names(&mut v);

    Ok(())
}

fn get_table_format() -> TableFormat {
    format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .build()
}

fn get_file_names(dir: &str, show_all: bool) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    let mut name_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;

        if let Ok(mut name) = dir_file.file_name().into_string() {
            let indice = name_files.iter().position(|x| x >= &name).unwrap_or(name_files.len());

            if !name.starts_with('.') || show_all {
                if data.is_dir() {
                    name.push('/');
                }

                name_files.insert(indice, name);
            }
        }
    }

    Ok(name_files)
}

fn print_names(files_name: &mut Vec<String>) {
    set_print_color(files_name);
    for i in files_name {
        println!("{}", i);
    }
}

fn list_files(dir: &str, show_all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let terminal_width = termwidth();

    let mut maior_len: usize = 0;
    let mut name_files: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;

        if let Ok(mut name) = dir_file.file_name().into_string() {
            let name_len = name.len() + 1;

            if name_len > maior_len {
                maior_len = name_len;
            }

            let indice;
            println!("{name}");
            if name_files.is_empty() {
                indice = 1;
            } else {
                indice = name_files.iter().position(|x| x >= &name).unwrap_or(name_files.len());
            }
            println!("{indice}");

            println!("{:?}", name_files);
            

            if !name.starts_with('.') || show_all {
                if data.is_dir() {
                    name.push_str("/ ");
                    name = Blue.paint(name).to_string();
                } else {
                    name = set_print_color_by_ext_perm(name, data.mode());
                    name.push(' ');
                }

                name_files.insert(indice-1, name);
            }
        }
    }

    let collums = terminal_width / maior_len;

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
    table.printstd();
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
        perm.push_str(&Blue.paint("d").to_string());
    }

    for i in perm_chars {
        if i == '1' {
            if count == 1 || count == 4 || count == 7 {
                perm.push_str(&Green.paint("r").to_string());
            } else if count == 2 || count == 5 || count == 8 {
                perm.push_str(&Red.paint("w").to_string());
            } else if count == 3 || count == 6 || count == 9{
                perm.push_str(&Yellow.paint("x").to_string());
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
    let mut table = Table::new();
    table.set_format(get_table_format());

    for entry in fs::read_dir(path)? {
        let dir_file = entry?;

        let data = dir_file.metadata()?;

        let mode = get_file_mode(&data);
        let perm = turn_mode_into_readable_perm(mode);

        let user_uid = data.uid();

        let user = get_user_by_uid(user_uid).unwrap();
        let user_name = user.name().to_str().unwrap();

        let last_modification_date = data.mtime();
        let date = Local.timestamp_opt(last_modification_date, 0);
        let date = date.unwrap();
        let date = date.format("%d/%m %H:%M");


        let mut size = 0;
        let mut sufixo: char = ' ';

        if !data.is_dir() {
            size = data.size();
            if size > 1000 {
                size /= 1000;
                sufixo = 'K';
            }
            if size > 1000 {
                size /= 1000;
                sufixo = 'M';
            }
            if size > 1000 {
                size /= 1000;
                sufixo = 'G';
            }
        }


        if let Ok(file_name) = dir_file.file_name().into_string() {
            if !file_name.starts_with('.') || show_all {
                if !data.is_dir() {
                    table.add_row(row![perm, user_name, format!("{size}{sufixo}"), date, file_name]);
                } else {
                    table.add_row(row![perm, user_name, "-", date, file_name]);
                }
            }
        }
    }

    table.printstd();

    Ok(())
}

fn set_print_color(files_name: &mut Vec<String>) {
    files_name.iter_mut().for_each(|mut element| {
        let nome = element.clone();
        if element.ends_with('/') {
            element.push(' ');
            element = &mut Blue.paint(nome).to_string();
        }
    })
    // for mut name in files_name {
    //     if name.ends_with('/') {
    //         name.push(' ');
    //         name = &mut Blue.paint(name.clone()).to_string();
    //     }
    // }
}

fn set_print_color_by_ext_perm(file_name: String, file_mode: u32) -> String {
    let ext = file_name.split('.').last().unwrap().to_string();

    let bin_mode = format!("{:b}", file_mode);
    let a = bin_mode.chars().nth(9).unwrap();

    let purple_ext = ["gz", "tar", "zip", "rar", "tgz", "zst"];
    let red_ext = ["rs", "py", "c", "cpp", "js", "ts", "toml", "yml", "json"];
    let yellow_ext = ["pdf", "pptx", "word"];
    let green_ext = ["sh"];

    if purple_ext.contains(&ext.as_str()) {
        return BrightPurple.paint(file_name).to_string();
    } else if red_ext.contains(&ext.as_str()) {
        return Red.paint(file_name).to_string();
    } else if yellow_ext.contains(&ext.as_str()) {
        return Yellow.paint(file_name).to_string();
    } else if green_ext.contains(&ext.as_str()) && a == '1' {
        return Green.paint(file_name).to_string();
    } 

    file_name
}
