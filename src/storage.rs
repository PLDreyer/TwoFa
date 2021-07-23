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

pub fn read_storage(path: &'static str) -> Result<String, FileReadError> {
    let mut in_file = std::env::var("HOME").unwrap();
    in_file.push_str(path);
    let f = File::open(in_file);
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

pub fn save_storage(path: &'static str, data: String, force: Option<bool>) -> Result<(), FileSaveError> {
    let mut in_file = std::env::var("HOME").unwrap();
    in_file.push_str(path);
    let mut encrypted_file = std::env::var("HOME").unwrap();
    encrypted_file.push_str("/twofa.storage");

    if Path::new(encrypted_file.as_str()).exists() && (force.is_some() && force.unwrap().eq(&false) || force.is_none())  {
        return Err(FileSaveError::AlreadyExists);
    }

    let f = File::create(&in_file);
    match f {
        Ok(mut file) => {
            match file.write_all(data.as_bytes()) {
                Ok(_) => {
                    println!("File '{}' created", &in_file);
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

pub fn delete_file(path: &'static str, logger: &Logger) -> Result<(), &'static str> {
    let mut file_to_delete = std::env::var("HOME").unwrap();
    file_to_delete.push_str(path);
    if let Err(e) = remove_file(&file_to_delete) {
        return Err("Error deleting file");
    }

    logger.min(
        format!("File '{}' deleted", &file_to_delete)
            .as_str()
    );

    Ok(())
}
