use encryptfile as ef;
use encryptfile::{ EncryptError };
use crate::logger::Logger;

pub fn encrypt_file(in_path: &str, out_path: &str, password: &str, logger: &Logger) -> Result<(), &'static str> {
    logger.min(
        format!("Encrypting '{}' to '{}'", &in_path, &out_path)
            .as_str()
    );

    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_path.to_owned()))
        .output_stream(ef::OutputStream::File(out_path.to_owned()))
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

pub fn decrypt_file(in_path: &str, out_path: &str, password: &str, logger: &Logger) -> Result<(), &'static str>{
    logger.min(
        format!("Decrypt '{}' to '{}'", &in_path, &out_path)
            .as_str()
    );

    let mut c = ef::Config::new();
    c.input_stream(ef::InputStream::File(in_path.to_owned()))
        .output_stream(ef::OutputStream::File(out_path.to_owned()))
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

