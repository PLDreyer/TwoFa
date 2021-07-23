use std::cmp::{PartialOrd, PartialEq};

#[derive(PartialOrd, PartialEq)]
enum LogLevel {
    Norm = 0,
    Min = 1,
    Mid = 2,
    Max = 3,
}

pub struct Logger {
    level: LogLevel,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(level: i32) -> Self {
            let log_level: Option<LogLevel>;
            match level {
                0 => {
                    log_level = Some(LogLevel::Norm);
                },
                1 => {
                    log_level = Some(LogLevel::Min);
                },
                2 => {
                    log_level = Some(LogLevel::Mid);
                },
                3 => {
                    log_level = Some(LogLevel::Max);
                },
                x => {
                    println!("LogLevel '{}' not supported. Norm chosen.", x);
                    log_level = Some(LogLevel::Norm);
                }
            };
            Self {
                level: log_level.unwrap(),
            }
    }

    pub fn norm(&self, msg: &str) -> () {
        println!("{}", msg);
    }

    pub fn min(&self, msg: &str) -> () {
        if &self.level >= &LogLevel::Min {
            println!("DEBUG: {}", msg);
        }
    }

    pub fn mid(&self, msg: &str) -> () {
        if &self.level >= &LogLevel::Mid {
            println!("DEBUG: {}", msg);
        }
    }

    pub fn max(&self, msg: &str) -> () {
        if &self.level >= &LogLevel::Max {
            println!("DEBUG: {}", msg);
        }
    }
}
