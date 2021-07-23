use encryptfile as ef;
use encryptfile::{ EncryptError };
use crate::logger::Logger;

pub fn encrypt_file(path: &'static str, password: &str, logger: &Logger) -> Result<(), &'static str> {
    let mut in_file = std::env::var("HOME").unwrap();
    let mut out_file = std::env::var("HOME").unwrap();
    in_file.push_str("/buffer.storage");
    out_file.push_str(path);

    logger.min(
        format!("Encrypting '{}' to '{}'", &in_file, &out_file)
            .as_str()
    );

    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
        .output_stream(ef::OutputStream::File(out_file.to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text(password.to_owned(), ef::scrypt_defaults()))
        .encrypt();
    match ef::process(&c) {
        Ok(()) => Ok(()),
        Err(e) => {
            match e {
                _ => {
                    Err("Unhandled error")
                }
            }
        }
    }
}

pub fn decrypt_file(path: &'static str, password: &str, logger: &Logger) -> Result<(), &'static str>{
    let mut in_file = std::env::var("HOME").unwrap();
    let mut out_file = std::env::var("HOME").unwrap();
    in_file.push_str(path);
    out_file.push_str("/buffer.storage");

    logger.min(
        format!("Decrypt '{}' to '{}'", &in_file, &out_file)
            .as_str()
    );

    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_file.to_owned()))
        .output_stream(ef::OutputStream::File(out_file.to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(password.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
        .decrypt();
    match ef::process(&c) {
        Ok(()) => Ok(()),
        Err(e) => {
            match e {
                EncryptError::IoError(_error) => {
                    Err("IoError")
                },
                _ => {
                    Err("Unhandled error")
                }
            }
        }
    }
}

