extern crate chrono;
use chrono::{DateTime, Utc};

use std::{fs, process};

use json;
use std::io::Write;


/// Reads contents from config file and returns as JSON
pub fn read_config_file() -> json::JsonValue {
    let contents = fs::read_to_string("config.json")
        .unwrap_or_else(|error| {
            let message = format!("Can not read contents from config.json file: {0}", error);
            
            write_error_log(&message);
            process::exit(1);
        }
    );

    let json_result = json::parse(&contents)
        .unwrap_or_else(|error| {
            write_error_log(&error.to_string());
            process::exit(1);
        }
    );

	json_result
}


/// Writes message in stderr
/// Writes message in /var/log/file_watcher.log for persistence
pub fn write_error_log(message: &str) {
    // format date and message
    let now: DateTime<Utc> = Utc::now();
    let now =  now.format("%Y-%m-%d %H:%M:%S");
    let message = format!("[{0}] {1}\n", now, message);

    // write to stderr
	eprintln!("{0}", message);

    // prepare file
    let file_name = "/var/log/file_watcher.log";
    let file = fs::OpenOptions::new()
        .append(true)
        .open(file_name);

    // write in file
    match file {
        Ok(mut file) => {
            if let Err(e) = file.write(message.as_bytes()) {
                eprintln!("Failed when trying to append data in file: {0}", e);
            };
        },

        Err(e) => {
            eprintln!("Failed to open file as append mode: {0}", e);
        }
    }
}
