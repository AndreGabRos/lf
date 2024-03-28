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

pub fn print_files_in_table(files: &mut Files) {
    let collums = get_collums(files);
    let mut table = Table::new();
    table.set_format(get_table_format());

    for name_chunk in files.files.chunks(collums) {
        let mut cells: Vec<Cell> = Vec::new();
        for name in name_chunk {
            cells.push(Cell::new(&set_print_color(name)));
        }
        table.add_row(Row::new(cells));
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
    let red_ext = ["rs", "py", "c", "cpp", "js", "ts", "toml", "yml", "json"];
    let yellow_ext = ["pdf", "pptx", "word"];
    let green_ext = ["sh"];

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

    for file in &files.files {
        let cells: Vec<Cell> = vec![
            Cell::new(&set_perm_color(file.get_perm())),
            Cell::new(&Green.paint(put_sufix_in_number(file.size)).to_string()),
            Cell::new(&Purple.paint(&format!("{} ", file.created_at.format("%d/%m/%Y %H:%M"))).to_string()),
            Cell::new(&Yellow.paint(&format!("{} ", file.creator.name().to_str().unwrap())).to_string()),
            Cell::new(&set_print_color(file))
        ];
        table.add_row(Row::new(cells));
    } 
    table.printstd();
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
