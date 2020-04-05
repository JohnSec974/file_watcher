use std::{fs, process};

use json;



/// Reads contents from config file and returns as JSON
pub fn read_config_file() -> json::JsonValue {
    let contents = fs::read_to_string("config.json")
        .unwrap_or_else(|error| {
            write_error_log("Can not read contents from config.json file");
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
fn write_error_log(message: &str) {
	eprintln!("{0}", message);

    if let Err(e) = fs::write("/var/log/file_watcher.log", message) {
        eprintln!("Can not write in log file: {0}", e);
    };
}
