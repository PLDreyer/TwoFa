use std::io::{stdin,stdout,Write};
use serde_json::{Value};
use std::fs::{create_dir_all};
use std::path::{Path};

pub fn prompt_for_input(input: &'static str) -> Result<String, &'static str> {
    let mut s=String::new();
    print!("{}: ", input);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    Ok(s)
}

pub fn merge_json(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                }
                else {
                    merge_json(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}

pub fn create_folder(path: &str) -> () {
    let file_or_dir = Path::new(path);
    return if file_or_dir.exists() {
        if file_or_dir.is_file() {
            println!("Dir to create is already a file. Aborting");
            std::process::exit(1);
        };

        if file_or_dir.is_dir() {
            println!("Directory already exists.");
        }

        ()
    } else {
        if let Err(_) = create_dir_all(path) {
            println!("Could not create directories");
            std::process::exit(1);
        }

        ()
    };
}