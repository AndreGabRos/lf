use core::panic;
use std::{fs::{self}, os::unix::prelude::{PermissionsExt, MetadataExt}};

use chrono::{ DateTime, Local };

use users::{get_user_by_uid, User};

pub struct File {
    name: String,
    file_perm: String,
    pub created_at: DateTime<Local>,
    pub creator: User,
    pub size: Option<u64>
}

pub struct Files {
    pub dir: String,
    pub files: Vec<File>,
}

impl File {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_perm(&self) -> &str {
        &self.file_perm
    }

    pub fn new(name: String, path: String) -> File {
        let metadata = fs::metadata(format!("{path}{name}"));

        let metadata = match metadata {
            Ok(md) => md,
            Err(e) => panic!("{e}"),
        };

        let user_id = metadata.uid();

        let created_at = metadata.created().unwrap();
        

        File {
            file_perm: get_perm(&format!("{path}{name}")),
            name,
            created_at: created_at.into(),
            creator: get_user_by_uid(user_id).unwrap(),
            size: if !metadata.is_dir() { Some(metadata.size()) } else { None }, 
        }
    }
}


pub fn get_file_names_in_dir(dir: &str, show_all: bool) -> 
    Result<Vec<File>, Box<dyn std::error::Error>> {
    let mut files: Vec<File> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let dir_file = entry?;
        let data = dir_file.metadata()?;

        if let Ok(mut name) = dir_file.file_name().into_string() {
            let indice = files.iter().position(|x| x.name.ge(&name) ).unwrap_or(files.len());

            if !name.starts_with('.') || show_all {
                if data.is_dir() {
                    name.push('/');
                }

                let file = File::new(name, dir.to_string());

                files.insert(indice, file);
            }
        }
    }

    Ok(files)
}

fn get_file_mode(file: &str) -> u32 {
    let metadata = fs::metadata(file);

    let metadata = match metadata {
        Ok(md) => md,
        Err(e) => panic!("{e}"),
    };

    let permissions = metadata.permissions();
    permissions.mode()
}

pub fn get_perm(file_name: &str) -> String {
    let mode = get_file_mode(file_name);
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
            if count == 1 || count == 4 || count == 7 {
                perm.push('r');
            } else if count == 2 || count == 5 || count == 8 {
                perm.push('w');
            } else if count == 3 || count == 6 || count == 9{
                perm.push('x');
            }
        }
        else {
            perm.push('-');
      }
        count += 1;
    }

    perm
}
