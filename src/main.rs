mod storage;
mod crypto;
mod twofa;
mod logger;
mod helper;

use clap::{AppSettings, Clap};
use serde_json::{from_str, Result as SerdeResult, Value, Map, Number};
use crate::storage::{read_storage, FileReadError, save_storage, delete_file, get_storage_path, Storage, check_storage};
use crate::crypto::{encrypt_file, decrypt_file};
use crate::twofa::{create_twofa_settings, create_code_with_twofa_settings, create_twofa_settings_with_input};
use crate::logger::{Logger};
use crate::helper::{prompt_for_input, merge_json, create_folder};

#[derive(Clap)]
#[clap(version = "1.0.0", author = "Paul D. <paullenardo@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// get / set / init
    action: String,
    #[clap(short, long)]
    /// name of application
    application: Option<String>,
    #[clap(short, long)]
    /// provide password
    password: String,
    #[clap(short, long)]
    /// set secret
    secret: Option<String>,
    #[clap(short, long, default_value = "30")]
    /// set window
    window: Number,
    #[clap(short, long, default_value = "sha1")]
    /// set hash
    hash: String,
    #[clap(short, long, default_value = "base32")]
    /// set encoding
    encoding: String,
    #[clap(short, long, parse(from_occurrences))]
    /// set debug level
    debug: i32,
}

fn main() {
    let opts: Opts = Opts::parse();
    let logger: Logger = Logger::new(opts.debug.clone());
    let storage_path = get_storage_path();

    let action = &opts.action.clone();
    let application = &opts.application.clone();
    if let Some(app) = application {
        logger.min(
            format!("Action: {}, App: {}", &opts.action.clone(), &app,
            ).as_str());
    } else {
        logger.min(
            format!("Action: {}", &action)
                .as_str()
        );
    }

    match &opts.action[..] {
        "set" => {
            set_secret(opts, storage_path, logger).expect("Failed to set secret");
        },
        "get" => {
            get_code(opts, storage_path, logger).expect("Failed to get code");
        },
        "init" => {
            create_storage(opts, storage_path, logger).expect("Failed to create storage");
        },
        _ => {
            println!("Action '{}' not supported", &opts.action);
            std::process::exit(1);
        }
    }
}

fn set_secret(opts: Opts, storage_path: Storage, logger: Logger) -> Result<(), &'static str>{
    let app = opts.application.clone().unwrap();
    let twofa_settings = create_twofa_settings_with_input(&opts).unwrap();

    logger.min(
        format!("Created settings: {}", twofa_settings.to_string())
            .as_str()
    );

    if let Err(_) = decrypt_file(&storage_path.en_file[..], &storage_path.de_file[..], &opts.password, &logger) {
        println!("Could not decrypt file");
        std::process::exit(1);
    };

    let mut data_from_file: String = String::new();

    match read_storage(&storage_path.de_file[..]) {
        Ok(data) => {
            data_from_file.push_str(&data[..]);
        },
        Err(e) => {
            match e {
                FileReadError::NoContent => { return Err("No content") },
                FileReadError::NoFile => {

                    if !check_storage(&storage_path.de_file[..]) {
                        if let Err(_) = save_storage(&storage_path.de_file[..], String::from("{}")) {
                            println!("Could not save storage");
                            std::process::exit(1);
                        }
                        data_from_file.push_str("{}");
                    } else {
                        logger.norm("file exists but could not be read");
                        std::process::exit(1);
                    }
                }
            };
        },
    };

    logger.min(
        format!("Data from file: \n {}", data_from_file)
            .as_str()
    );

    let deserialized_data: SerdeResult<Value> = from_str(data_from_file.as_str());
    if let SerdeResult::Err(_) = deserialized_data {
        return Err("Could not parse storage from file");
    }

    let mut data = deserialized_data.unwrap();

    if &data[&app] != &Value::Null {
        let user_prompt = prompt_for_input("Application is already configured. Overwrite ? [y/N] ").unwrap();
        if user_prompt.ne(&String::from("y")) {
            println!("Stopping action");
            if let Err(_) = delete_file(&storage_path.de_file[..], &logger) {
                println!("Could not delete decrypted file");
            }
            std::process::exit(0);
        }
    }

    let json_data = serde_json::json!({
        app: twofa_settings.to_json()
    });

    merge_json(&mut data, json_data);

    logger.min(
        format!("Merged data: \n {}", &data.to_string())
            .as_str()
    );

    match save_storage(&storage_path.de_file[..], data.to_string()) {
        Err(_) => {
            println!("Could not save storage");
            std::process::exit(1);
        },
        _ => {}
    };

    if let Err(_) = encrypt_file(&storage_path.de_file[..], &storage_path.en_file[..], &opts.password, &logger) {
        println!("Could not encrypt file");
        std::process::exit(1);
    };

    if let Err(e) = delete_file(&storage_path.de_file[..], &logger) {
        return Err(e);
    }
    Ok(())
}

fn get_code(opts: Opts, storage_path: Storage, logger: Logger) -> Result<(), &'static str>{
    let app = opts.application.clone().unwrap();

    if let Err(_) = decrypt_file(&storage_path.en_file[..], &storage_path.de_file[..], &opts.password, &logger) {
        println!("Could not decrypt file");
        std::process::exit(1);
    };

    let mut data_from_file: String = String::new();
    let application_data: Option<Map<String, Value>>;

    match read_storage(&storage_path.de_file[..]) {
        Ok(data) => {
            data_from_file.push_str(&data[..]);
        },
        Err(e) => {
            match e {
                FileReadError::NoContent => { return Err("No content") },
                FileReadError::NoFile => {
                    if let Err(_) = save_storage(&storage_path.en_file[..], String::from("{}")) {
                        logger.norm(
                            format!("Could not save storage")
                                .as_str()
                        );
                        std::process::exit(1);
                    };

                    data_from_file.push_str("{}");
                }
            };
        },
    };

    logger.min(
        format!("Data from file: \n {}", &data_from_file)
            .as_str()
    );

    let deserialized_data: SerdeResult<Value> = from_str(data_from_file.as_str());

    if let SerdeResult::Err(_) = deserialized_data {
        return Err("Could not parse storage from file");
    }

    match deserialized_data {
        SerdeResult::Ok(data) => {
            match data[&app.as_str()].clone() {
                Value::Object(obj) => {
                  application_data = Some(obj);
                },
                _ => {
                    println!("Application does not exist. Exiting.");
                    std::process::exit(0);
                }
            };
        },
        SerdeResult::Err(_) => {
            return Err("Error accessing deserialized data");
        }
    };

    let twofa_settings = create_twofa_settings(application_data).expect("Could not create TwofaSettings");

    logger.min(
        format!("Created TwofaSettings: \n {}", &twofa_settings.to_string())
            .as_str()
    );

    let code = create_code_with_twofa_settings(&twofa_settings).expect("Could not get code with settings");

    println!("Code: {}", code);

    encrypt_file(&storage_path.de_file[..], &storage_path.en_file[..], &opts.password, &logger).expect("Could not encrypt file");

    if let Err(e) = delete_file(&storage_path.de_file[..], &logger) {
        return Err(e);
    }
    Ok(())
}

fn create_storage(opts: Opts, storage_path: Storage, logger: Logger) -> Result<(), &'static str> {
    create_folder(&storage_path.dir[..]);

    println!("Folderpath: {}", &storage_path.dir[..]);
    println!("Storagepath Decrypted: {}", &storage_path.de_file[..]);
    println!("Storagepath Encrypted: {}", &storage_path.en_file[..]);

    if check_storage(&storage_path.en_file[..]) {
        let user_prompt = prompt_for_input("Storage already exist. Overwrite ? [y/N] ").unwrap();
        if user_prompt.ne(&String::from("y")) {
            println!("Stopping action");
            std::process::exit(0);
        }
        if let Err(_) = save_storage(&storage_path.de_file[..], String::from("{}")) {
            return Err("Could not overwrite file");
        }
    } else {
        if let Err(_) = save_storage(&storage_path.de_file[..], String::from("{}")) {
            println!("Could not save storage");
            std::process::exit(1);
        }
    }


    if let Err(e) = encrypt_file(&storage_path.de_file[..], &storage_path.en_file[..], &opts.password, &logger) {
        return Err(e);
    }

    if let Err(e) = delete_file(&storage_path.de_file[..], &logger) {
        return Err(e);
    }

    Ok(())
}
