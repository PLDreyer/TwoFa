#![allow(dead_code,unused_variables)]
use std::fs::{ File, remove_file };
use std::path::Path;
use std::io::{Read, Write};
use std::result::{ Result };
use crate::logger::Logger;

pub enum FileReadError {
    NoFile,
    NoContent,
}

pub enum FileSaveError {
    NoSave,
    NoCreate,
    AlreadyExists,
}

pub enum FileDeleteError {
    FileExists,
    FileNotFound,
    FileDeleteError,
}

pub struct Storage {
    pub dir: String,
    pub en_file: String,
    pub de_file: String,
}

impl Storage {
    pub fn new(dir: String, en_file: String, de_file: String) -> Self {
        Self {
            dir,
            en_file,
            de_file,
        }
    }
}

pub fn read_storage(path: &str) -> Result<String, FileReadError> {
    let f = File::open(&path);
    match f {
        Ok(mut file) => {
            let mut buffer = String::new();
            let s = file.read_to_string(&mut buffer);
            match s {
                Ok(_) => {
                    Ok(buffer)
                },
                Err(e) => {
                    println!("{}", e);
                    Err(FileReadError::NoContent)
                }
            }
        },
        Err(e) => {
            Err(FileReadError::NoFile)
        }
    }
}

pub fn save_storage(path: &str, data: String, force: Option<bool>) -> Result<(), FileSaveError> {
    if Path::new(&path).exists() && (force.is_some() && force.unwrap().eq(&false) || force.is_none())  {
        return Err(FileSaveError::AlreadyExists);
    }

    let f = File::create(&path);
    match f {
        Ok(mut file) => {
            match file.write_all(data.as_bytes()) {
                Ok(_) => {
                    Ok(())
                },
                Err(e) => {
                    Err(FileSaveError::NoSave)
                }
            }
        },
        Err(e) => {
            Err(FileSaveError::NoCreate)
        }
    }
}

pub fn delete_file(path: &str, logger: &Logger) -> Result<(), &'static str> {
    if let Err(e) = remove_file(&path) {
        return Err("Error deleting file");
    }

    logger.min(
        format!("File '{}' deleted", &path)
            .as_str()
    );

    Ok(())
}

pub fn get_storage_path() -> Storage {
    let mut folder_path = std::env::var("HOME").expect("HOME var needed");
    folder_path.push_str("/.twofa");

    let mut de_file = folder_path.clone();
    de_file.push_str("/buffer.storage");

    let mut en_file = folder_path.clone();
    en_file.push_str("/twofa.storage");

    Storage::new(
        folder_path,
            en_file,
            de_file,
    )
}
