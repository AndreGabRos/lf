use prettytable::{format::{TableFormat, self}, Table, Cell, Row};
use textwrap::termwidth;
use ansiterm::Color::*;

use crate::file_operations::{ File, Files };

fn get_table_format() -> TableFormat {
    format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .build()
}

fn get_collums(files: &Files) -> usize {
    let terminal_width = termwidth();
    let mut maior_len = 0;
    for file in &files.files {
        if file.get_name().len() > maior_len {
            maior_len = file.get_name().len()
        }
    }

    terminal_width/(maior_len+1)
}

fn get_maior_len(files: &Files) -> usize {
    let mut maior_len = 0;
    for file in &files.files {
        if file.get_name().len() > maior_len {
            maior_len = file.get_name().len()
        }
    }

    maior_len+1
}

pub fn print_files_in_table(files: &mut Files) {
    let collums = get_collums(files);
    let mut table = Table::new();
    table.set_format(get_table_format());

    if collums > 0 {
        for file_chunk in files.files.chunks(collums) {
            let mut cells: Vec<Cell> = Vec::new();
            for file in file_chunk {
                cells.push(Cell::new(&format!("{} ", &set_print_color(file))));
            }
            table.add_row(Row::new(cells));
        } 
    } else {
        for file in &files.files {
            println!("{}", set_print_color(file));
        }
    }
    table.printstd();
}

fn set_print_color(file: &File) -> String {
    if file.get_name().ends_with('/') {
        return Blue.paint(file.get_name()).to_string()
    }

    set_print_color_by_ext_perm(file.get_name(), file.get_perm())
}

fn set_print_color_by_ext_perm(file_name: &str, file_perm: &str) -> String {
    let ext = file_name.split('.').last().unwrap().to_string();

    let is_exec = file_perm.find('x');

    let purple_ext = ["gz", "tar", "zip", "rar", "tgz", "zst"];
    let red_ext = ["rs", "py", "c", "cpp", "js", "ts", "toml", "yml", "json", "conf"];
    let yellow_ext = ["pdf", "pptx", "word", "docx"];
    let green_ext = ["sh", "AppImage"];

    if purple_ext.contains(&ext.as_str()) {
        return BrightPurple.paint(file_name).to_string();
    } else if red_ext.contains(&ext.as_str()) {
        return Red.paint(file_name).to_string();
    } else if yellow_ext.contains(&ext.as_str()) {
        return Yellow.paint(file_name).to_string();
    } else if green_ext.contains(&ext.as_str()) && is_exec.is_some() {
        return Green.paint(file_name).to_string();
    } 

    file_name.to_string()
}

fn set_perm_color(file_perm: &str) -> String {
    let mut colored_perm = String::new();
    for char in file_perm.chars() {
        if char == 'd' {
            colored_perm.push_str(&Blue.paint("d").to_string());
        } else if char == 'x' {
            colored_perm.push_str(&Red.paint("x").to_string());
        } else if char == 'w' {
            colored_perm.push_str(&Yellow.paint("w").to_string());
        } else if char == 'r' {
            colored_perm.push_str(&Green.paint("r").to_string());
        } else {
            colored_perm.push(char);
        }

    }

    colored_perm
}

pub fn list_files_long(files: &Files) {
    let mut table = Table::new();
    table.set_format(get_table_format());

    let maior_len_user = get_maior_len_name_user(&files.files);

    for file in &files.files {
        let len_user = file.creator.name().to_str().unwrap().len();
        let espaco_tam = " ".repeat(4-put_sufix_in_number(file.size.unwrap_or(0)).len());
        let espaco_user = " ".repeat(maior_len_user-len_user);

        let print_size = if let Some(num) = file.size {
            Green.paint(put_sufix_in_number(num)).to_string()
        } else {
                "  ".to_string()
        };

        println!(
            "{:width$} {}{} {} {}{} {}", 
            set_perm_color(file.get_perm()),
            print_size,
            espaco_tam,
            Purple.paint(&format!("{} ", file.created_at.format("%d/%m/%Y %H:%M"))),
            Yellow.paint(&format!("{} ", file.creator.name().to_str().unwrap())),
            espaco_user,
            set_print_color(file),
            width = 11,
        );
   } 
}

fn get_maior_len_name_user(files: &[File]) -> usize {
    let mut maior_len = 0;
    let mut _len_user = 0;

    for item in files {
        _len_user = item.creator.name().to_str().unwrap().len();

        if _len_user > maior_len {
            maior_len = _len_user;
        }
    }

    maior_len
}

fn put_sufix_in_number(number: u64) -> String {
    let mut sufix = ' ';
    let mut number = number;
    if number > 1000 {
        number /= 1000;
        sufix = 'K';
    }
    if number > 1000 {
        number /= 1000;
        sufix = 'M';
    }
    if number > 1000 {
        number /= 1000;
        sufix = 'G';
    }

    format!("{number}{sufix}")
}
