use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, TimeZone, Utc};

static DEFAULT_FILE_PATH: &str = "./last_update.timestamp";

fn get_file_path() -> PathBuf {
    let path = env::var("LAST_UPDATE_FILE").unwrap_or(DEFAULT_FILE_PATH.to_string());
    Path::new(&path).to_path_buf()
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn write_file(path: &Path, contents: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn get_last_update() -> DateTime<Utc> {
    let path = get_file_path();

    let contents = match read_file(&path) {
        Ok(contents) => contents,
        Err(_) => {
            write_file(&path, "0");
            read_file(&path).unwrap()
        }
    };

    let timestamp = contents.parse::<i64>().unwrap();
    Utc.timestamp_opt(timestamp, 0).unwrap()
}

pub fn set_last_update(timestamp: DateTime<Utc>) {
    write_file(&get_file_path(), &timestamp.timestamp().to_string());
}

pub fn set_last_update_to_now() {
    set_last_update(Utc::now());
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use serial_test::serial;
    use std::env;

    #[ctor::ctor]
    fn setup_env() {
        env::set_var("LAST_UPDATE_FILE", "./last_update.timestamp.test");
    }

    #[test]
    #[serial]
    fn get_last_update_file_does_not_exist() {
        let path = get_file_path();
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }

        let last_update = get_last_update();
        assert_eq!(last_update.timestamp(), 0);

        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        } else {
            panic!("File should have been created");
        }
    }

    #[test]
    #[serial]
    fn get_last_update_valid() {
        let path = get_file_path();
        write_file(&path, "1635196800"); // A sample timestamp
        let result = get_last_update();
        let expected = Utc.timestamp_opt(1635196800, 0).unwrap();
        assert_eq!(result, expected);

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    #[serial]
    fn set_last_update_to_now() {
        let path = get_file_path();
        if path.exists() {
            std::fs::remove_file(&path).unwrap();
        }

        let timestamp = Utc::now();
        set_last_update(timestamp);

        let result = read_file(&path).unwrap();
        let expected = timestamp.timestamp().to_string();
        assert_eq!(result, expected);

        std::fs::remove_file(&path).unwrap();
    }
}
