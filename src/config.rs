use crate::config::OS::{LINUX, MACOS, WINDOWS};
use crate::files::get_cache_folder;

use std::env;
use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::io::BufWriter;
use std::io::Write;

use toml::{Table, Value};

pub const CONFIG_FILE: &str = "epubit-integral-config.toml";

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum OS {
    WINDOWS,
    MACOS,
    LINUX,
}

impl OS {
    pub fn to_str(&self) -> &str {
        match self {
            WINDOWS => "windows",
            MACOS => "macos",
            LINUX => "linux",
        }
    }

    pub fn is(&self, os: &str) -> bool {
        self.to_str().eq_ignore_ascii_case(os)
    }
}

pub fn str_to_os(os: &str) -> OS {
    if WINDOWS.is(os) {
        WINDOWS
    } else if MACOS.is(os) {
        MACOS
    } else {
        LINUX
    }
}

pub struct StringKey<'a>(pub Vec<&'a str>, pub &'a str);

impl StringKey<'_> {
    pub fn get_value(&self) -> String {
        let config = get_config().unwrap_or_default();
        let keys = self.0.to_owned();
        let mut result;
        for key in keys {
            if config.contains_key(key) {
                result = config[key].as_str().unwrap().to_string()
            } else {
                result = env::var(key).unwrap_or_default()
            }
            if !result.is_empty() {
                return result;
            }
        }
        self.1.to_owned()
    }
}

pub struct IntegerKey<'a>(pub &'a str, pub i64);

impl IntegerKey<'_> {
    pub fn get_value(&self) -> i64 {
        let config = get_config().unwrap_or_default();
        let key = self.0;
        if config.contains_key(key) {
            config[key].as_integer().unwrap() as i64
        } else {
            env::var(key)
                .unwrap_or_default()
                .parse::<i64>()
                .unwrap_or_else(|_| self.1.to_owned())
        }
    }
}

pub struct BooleanKey<'a>(pub &'a str, pub bool);

impl BooleanKey<'_> {
    pub fn get_value(&self) -> bool {
        let config = get_config().unwrap_or_default();
        let key = self.0;
        if config.contains_key(key) {
            config[key].as_bool().unwrap()
        } else {
            env::var(key)
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or_else(|_| self.1.to_owned())
        }
    }
}

pub fn update_config(key: &str, value: Value) -> Result<(), Box<dyn Error>> {
    let mut config = get_config().unwrap_or_default();
    config.insert(key.into(), value);
    let mut file = std::fs::File::create(CONFIG_FILE)?;
    file.write_all(&config.to_string().into_bytes())?;
    Ok(())
}

fn get_config() -> Result<Table, Box<dyn Error>> {
    //    let config_path = get_cache_folder().join(CONFIG_FILE);
    Ok(read_to_string(CONFIG_FILE)?.parse()?)
}

fn concat(prefix: &str, suffix: &str) -> String {
    let mut version_label: String = prefix.to_owned();
    version_label.push_str(suffix);
    version_label
}
